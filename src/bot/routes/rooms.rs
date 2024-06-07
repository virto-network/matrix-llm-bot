use actix_multipart::Multipart;
use actix_web::ResponseError;
use actix_web::{web, Responder};
use anyhow::{Context, Error};
use futures_util::StreamExt as _;
use matrix_sdk::ruma::api::client::room::create_room::v3::CreationContent;
use matrix_sdk::ruma::RoomId;
use matrix_sdk::ruma::{
    api::{self},
    events::{
        AnyInitialStateEvent, EmptyStateKey, InitialStateEvent,
    },
    serde::Raw,
};
use matrix_sdk::Client as MatrixClient;
use reqwest::StatusCode;
use ruma::events::room::avatar::RoomAvatarEventContent;
use ruma::OwnedMxcUri;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::domain::{CommunityData, CommunityMatrixId, Uri};
use crate::utils::{error_chain_fmt, mxc_to_download_uri};

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
    println!("{:?}", form);
    let response = on_handle_create_room(form.0,&matrix_client)
        .await
        .context("Failed to create a room")?;

    Ok(web::Json(CommunityMatrixId {
        id: response
    }))
}

#[tracing::instrument(name = "Create a room", skip(matrix_client))]
pub async fn on_handle_create_room(
    room: CommunityData, 
    matrix_client: &MatrixClient,
) -> Result<String, Error> {
    let mut request = api::client::room::create_room::v3::Request::new();
    request.name = Some(&room.name);
    
    let topic = format!("{}\nTAG_VIRTO: {}", 
        room.description.unwrap_or("".to_string()), 
        room.industry 
    );
    request.topic = Some(&topic);
    
    let mut init_state_ev_vec: Vec<Raw<AnyInitialStateEvent>> = vec![];

    if let Some(uri) = room.logo {
        let mut avatar_content = RoomAvatarEventContent::new();
        avatar_content.url = Some(OwnedMxcUri::from(uri));

        let init_state_ev: InitialStateEvent<RoomAvatarEventContent> = InitialStateEvent {
            content: avatar_content,
            state_key: EmptyStateKey,
        };

        let raw_init_state_ev =
            Raw::new(&init_state_ev)?;

        let raw_any_init_state_ev: Raw<AnyInitialStateEvent> = raw_init_state_ev.cast();
        init_state_ev_vec.push(raw_any_init_state_ev);

        request.initial_state = &init_state_ev_vec;
    };

    let mut content = CreationContent::new();
    content.room_type = Some(ruma::room::RoomType::Space);

    request.creation_content = Some(Raw::new(&content)?);

    let response = matrix_client.create_room(request).await?;
    
    tracing::Span::current().record("room_id", &tracing::field::display(&response.room_id));

    Ok(response.room_id.to_string())
}

#[tracing::instrument(
    name = "Upload a file", 
    skip(payload, matrix_client) 
)]
pub async fn upload(mut payload: Multipart, matrix_client: web::Data<MatrixClient>) -> Result<impl Responder, ServerError> {
    let filepath = "./upload.tmp";
    let mut f = File::create(filepath).await.map_err(|e| ServerError::UnexpectedError(e.into()))?;

    if let Some(field) = payload.next().await {
        let mut field = field.map_err(|_| ServerError::ValidationError("File not found".to_string()))?;
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| ServerError::ValidationError("File not found".to_string()))?;
            f.write_all(&data).await.map_err(|e| ServerError::UnexpectedError(e.into()))?;
        }
    }

    let response = upload_to_matrix(filepath, &matrix_client).await?;

    Ok(web::Json(Uri {
        uri: response
    }))
}

#[tracing::instrument(name = "Upload a file to matrix", skip(filepath, matrix_client))]
async fn upload_to_matrix(filepath: &str, matrix_client: &MatrixClient) -> Result<String, Error> {

    let mut file = File::open(filepath).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let response = matrix_client.media().upload(&mime::IMAGE_JPEG, &buffer).await?;

    Ok(response.content_uri.to_string())
}

#[tracing::instrument(
    name = "Get community metadata", 
    skip(path, matrix_client) 
)]
pub async fn get_by_id(path: web::Path<String>, matrix_client: web::Data<MatrixClient>) -> Result<impl Responder, ServerError> {
    let id = path.into_inner();
    let response = get_by_id_from_matrix(&id, &matrix_client).await?;

    Ok(web::Json(response))
}

#[tracing::instrument(name = "Get a space from matrix", skip(id, matrix_client))]
async fn get_by_id_from_matrix(id: &str, matrix_client: &MatrixClient) -> Result<CommunityData, Error> {
    let room = RoomId::parse(id)?;
    let response = matrix_client.get_room(&room).expect("Should get a matrix room id");

    let name = response.name().unwrap_or("".to_string());
    let logo = response.avatar_url().and_then(|uri| mxc_to_download_uri(&uri));
    let mut description = None;
    let mut industry = String::new();

    
    if let Some(topic) = response.topic() {
        tracing::Span::current().record("topic", &tracing::field::display(&topic));
        let split = topic.split("TAG_VIRTO: ").collect::<Vec<&str>>();

        description = split.get(0).and_then(|v| Some(v.to_string()));
        industry = split.get(1).map(|v| v.to_string()).unwrap_or("".to_string());
    }

    let community = CommunityData { name, logo, description, industry };

    Ok(community)
}

