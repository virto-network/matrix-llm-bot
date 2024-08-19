use crate::domain::{CommunityData, CommunityMatrixId, Uri};
use crate::utils::{error_chain_fmt, mxc_to_download_uri};
use actix_multipart::Multipart;
use actix_web::ResponseError;
use actix_web::{web, Responder};
use anyhow::{Context, Error};
use futures_util::StreamExt as _;
use matrix_sdk::ruma::api::client::room::create_room::v3::CreationContent;
use matrix_sdk::ruma::RoomId;
use matrix_sdk::ruma::{
    api::{self},
    events::{AnyInitialStateEvent, EmptyStateKey, InitialStateEvent},
    serde::Raw,
};
use matrix_sdk::Client as MatrixClient;
use reqwest::StatusCode;
use ruma::api::client::state::get_state_events_for_key;
use ruma::events::macros::EventContent;
use ruma::events::room::avatar::RoomAvatarEventContent;
use ruma::{OwnedMxcUri, RoomAliasId};
use serde::{Deserialize, Serialize};

const SERVER: &str = "virto.community";
const EXPECTED_BYTES: usize = 32;
const SLUG_LEN: usize = EXPECTED_BYTES - 1 - 1 - SERVER.len() - 2; // EXPECTED_BYTES - '#' - ':' - SERVER - OFFSET

#[derive(thiserror::Error)]
pub enum ServerError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ServerError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServerError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Task to reate a room", 
    skip(
        form
    ), 
    fields(
        space_name = %form.name,
        space_industry = %form.industry
    )
)]
pub async fn create(
    form: web::Json<CommunityData>,
    matrix_client: web::Data<MatrixClient>,
) -> Result<impl Responder, ServerError> {
    let response = on_handle_create_room(form.0, &matrix_client)
        .await
        .context("Failed to create a room")?;

    Ok(web::Json(CommunityMatrixId { id: response }))
}

#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[ruma_event(type = "m.virto.industry", kind = State, state_key_type = String)]
pub struct IndustryEventContent {
    pub industry_name: String,
}

#[tracing::instrument(name = "Create a room", skip(matrix_client))]
pub async fn on_handle_create_room(
    room: CommunityData,
    matrix_client: &MatrixClient,
) -> Result<String, Error> {
    let mut request = api::client::room::create_room::v3::Request::new();
    request.name = Some(&room.name);

    let topic = room.description.unwrap_or("".to_string());
    request.topic = Some(&topic);

    let mut init_state_ev_vec: Vec<Raw<AnyInitialStateEvent>> = vec![];

    if let Some(uri) = room.logo {
        let mut avatar_content = RoomAvatarEventContent::new();
        avatar_content.url = Some(OwnedMxcUri::from(uri));

        let init_state_ev: InitialStateEvent<RoomAvatarEventContent> = InitialStateEvent {
            content: avatar_content,
            state_key: EmptyStateKey,
        };

        let raw_init_state_ev = Raw::new(&init_state_ev)?;
        let raw_any_init_state_ev: Raw<AnyInitialStateEvent> = raw_init_state_ev.cast();
        init_state_ev_vec.push(raw_any_init_state_ev);
    }

    let custom_content = IndustryEventContent {
        industry_name: room.industry,
    };

    let custom_state_ev: InitialStateEvent<IndustryEventContent> = InitialStateEvent {
        content: custom_content,
        state_key: "".to_string(),
    };

    let raw_custom_state_ev = Raw::new(&custom_state_ev)?;
    let raw_any_custom_state_ev: Raw<AnyInitialStateEvent> = raw_custom_state_ev.cast();
    init_state_ev_vec.push(raw_any_custom_state_ev);

    request.initial_state = &init_state_ev_vec;

    let mut content = CreationContent::new();
    content.room_type = Some(ruma::room::RoomType::Space);

    request.creation_content = Some(Raw::new(&content)?);

    let slug = name_to_slug(&room.name);
    let mut offset = String::new();

    while let Ok(_) =
        get_by_alias_from_matrix(&format!("#{}{}:{}", slug, offset, SERVER), matrix_client).await
    {
        let mut offset_number = offset.parse::<u8>().unwrap_or(0);
        offset_number += 1u8;
        offset = offset_number.to_string();
    }
    let slug = format!("{}{}", slug, offset);
    request.room_alias_name = Some(&slug);

    let response = matrix_client.create_room(request).await?;

    tracing::Span::current().record("room_id", &tracing::field::display(&response.room_id));

    let request = ruma::api::client::space::get_hierarchy::v1::Request::new(&response.room_id);
    let response = matrix_client.send(request, None).await?;

    let response = response
        .rooms
        .get(0)
        .ok_or(anyhow::anyhow!("Room not found"))?
        .clone();
    let alias = response
        .canonical_alias
        .ok_or(anyhow::anyhow!("Room not found"))?
        .clone();

    Ok(alias.to_string())
}

#[tracing::instrument(name = "Upload a file", skip(payload, matrix_client))]
pub async fn upload(
    mut payload: Multipart,
    matrix_client: web::Data<MatrixClient>,
) -> Result<impl Responder, ServerError> {
    let mut buffer = Vec::new();

    if let Some(field) = payload.next().await {
        let mut field =
            field.map_err(|_| ServerError::ValidationError("File not found".to_string()))?;
        while let Some(chunk) = field.next().await {
            let data =
                chunk.map_err(|_| ServerError::ValidationError("File not found".to_string()))?;
            buffer.extend_from_slice(&data);
        }
    }

    let response = upload_to_matrix(&buffer, &matrix_client).await?;

    Ok(web::Json(Uri { uri: response }))
}

#[tracing::instrument(name = "Upload a file to matrix", skip(buffer, matrix_client))]
async fn upload_to_matrix(buffer: &[u8], matrix_client: &MatrixClient) -> Result<String, Error> {
    let response = matrix_client
        .media()
        .upload(&mime::IMAGE_JPEG, buffer)
        .await?;

    Ok(response.content_uri.to_string())
}

#[tracing::instrument(name = "Get community metadata", skip(path, matrix_client))]
pub async fn get_by_alias(
    path: web::Path<String>,
    matrix_client: web::Data<MatrixClient>,
) -> Result<impl Responder, ServerError> {
    let id = path.into_inner();
    let response = get_by_alias_from_matrix(&id, &matrix_client).await?;
    let response = get_by_id_from_matrix(&response, &matrix_client).await?;

    Ok(web::Json(response))
}

#[tracing::instrument(name = "Get a space from matrix", skip(alias, matrix_client))]
async fn get_by_alias_from_matrix(
    alias: &str,
    matrix_client: &MatrixClient,
) -> Result<String, Error> {
    let alias = RoomAliasId::parse(alias)?;

    let request = ruma::api::client::alias::get_alias::v3::Request::new(&alias);
    let response = matrix_client.send(request, None).await?;
    Ok(response.room_id.to_string())
}

#[tracing::instrument(name = "Get community metadata", skip(path, matrix_client))]
pub async fn get_by_id(
    path: web::Path<String>,
    matrix_client: web::Data<MatrixClient>,
) -> Result<impl Responder, ServerError> {
    let id = path.into_inner();
    let response = get_by_id_from_matrix(&id, &matrix_client).await?;

    Ok(web::Json(response))
}

#[tracing::instrument(name = "Get a space from matrix", skip(id, matrix_client))]
async fn get_by_id_from_matrix(
    id: &str,
    matrix_client: &MatrixClient,
) -> Result<CommunityData, Error> {
    let room = RoomId::parse(id)?;
    let request = ruma::api::client::space::get_hierarchy::v1::Request::new(&room);
    let response = matrix_client.send(request, None).await?;

    let response = response
        .rooms
        .get(0)
        .ok_or(anyhow::anyhow!("Room not found"))?
        .clone();

    let name = response.name.unwrap_or("".to_string());
    let logo = response
        .avatar_url
        .and_then(|uri| mxc_to_download_uri(&uri));
    let description = response.topic;

    let state_key = "".to_string();
    let request =
        get_state_events_for_key::v3::Request::new(&room, "m.virto.industry".into(), &state_key);

    let response = matrix_client.send(request, None).await?;
    let event = response.content;
    let event_json = serde_json::to_string(&event)?;
    let custom_event: IndustryEventContent = serde_json::from_str(&event_json)?;

    let industry = custom_event.industry_name;

    let community = CommunityData {
        name,
        logo,
        description,
        industry,
    };

    Ok(community)
}

fn name_to_slug(name: &str) -> String {
    let filtered: String = name
        .chars()
        .filter_map(|c| match c {
            ' ' => Some('_'),
            _ if c.is_ascii_alphanumeric() => Some(c),
            _ => None,
        })
        .collect();

    let filtered = if filtered.len() > SLUG_LEN {
        filtered[..SLUG_LEN].to_string()
    } else {
        filtered
    };

    filtered
}
