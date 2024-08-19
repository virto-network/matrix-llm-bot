use crate::domain::{
    CommunityMatrixId, InitiativeData, InitiativeHistory, InitiativeInfoContent,
    InitiativeInitContent, InitiativeVoteContent, InitiativeVoteData,
};
use crate::utils::error_chain_fmt;
use actix_web::ResponseError;
use actix_web::{web, Responder};
use anyhow::{Context, Error};
use matrix_sdk::config::SyncSettings;
use matrix_sdk::deserialized_responses::{SyncTimelineEvent, TimelineSlice};
use matrix_sdk::room::{Joined, MessagesOptions};
use matrix_sdk::ruma::{
    api::{self},
    events::AnyInitialStateEvent,
    serde::Raw,
};
use matrix_sdk::Client as MatrixClient;
use reqwest::StatusCode;
use ruma::api::client::filter::{LazyLoadOptions, RoomEventFilter};
use ruma::api::client::room::Visibility;
use ruma::events::OriginalSyncMessageLikeEvent;
use ruma::{assign, RoomId};
use serde_json::{to_string_pretty, Value};

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
    name = "Task to create a room", 
    skip(
        form
    ), 
    fields(
        initiative_sender = %form.init.sender,
        initiative_name = %form.info.name,
    )
)]
pub async fn create_initiative(
    form: web::Json<InitiativeData>,
    matrix_client: web::Data<MatrixClient>,
) -> Result<impl Responder, ServerError> {
    let response = on_handle_create_initiative_room(form.0, &matrix_client)
        .await
        .context("Failed to create initiative a room")?;

    Ok(web::Json(CommunityMatrixId { id: response }))
}

#[tracing::instrument(name = "create a initiative room", skip(matrix_client))]
pub async fn on_handle_create_initiative_room(
    data: InitiativeData,
    matrix_client: &MatrixClient,
) -> Result<String, Error> {
    let mut request = api::client::room::create_room::v3::Request::new();
    let init_state_ev_vec: Vec<Raw<AnyInitialStateEvent>> = vec![];

    request.name = Some(&data.info.name);
    request.is_direct = false;

    request.visibility = Visibility::Public;

    request.initial_state = &init_state_ev_vec;

    let response = matrix_client.create_room(request).await?;

    let _ = matrix_client.sync_once(SyncSettings::default()).await;

    loop {
        match matrix_client.get_joined_room(&response.room_id) {
            Some(room) => {
                tracing::Span::current().record(
                    "Room is now available in the client state.",
                    response.room_id.to_string(),
                );
                let _ = on_handle_init_initiative_message(
                    InitiativeInitContent {
                        sender: data.init.sender.clone(),
                        is_admin: data.init.is_admin,
                    },
                    room.clone(),
                )
                .await?;

                let _ = on_handle_info_initiative_message(
                    InitiativeInfoContent {
                        name: data.info.name,
                        description: data.info.description,
                        tags: data.info.tags,
                        actions: data.info.actions,
                    },
                    room,
                )
                .await?;
                break;
            }
            None => {
                let _ = matrix_client.sync_once(SyncSettings::default()).await;
                tracing::Span::current()
                    .record("Room not found after sync.", response.room_id.to_string());
            }
        }
    }

    Ok(response.room_id.to_string())
}

#[tracing::instrument(name = "init a initiative message", skip(room))]
pub async fn on_handle_init_initiative_message(
    data: InitiativeInitContent,
    room: Joined,
) -> Result<(), Error> {
    let content = serde_json::to_value(data)?;
    room.send_raw(content, "m.virto.initiative_init", None)
        .await?;

    Ok(())
}

#[tracing::instrument(name = "place info initiative message", skip(room))]
pub async fn on_handle_info_initiative_message(
    data: InitiativeInfoContent,
    room: Joined,
) -> Result<(), Error> {
    let content = serde_json::to_value(data)?;
    room.send_raw(content, "m.virto.initiative_info", None)
        .await?;

    Ok(())
}

#[tracing::instrument(name = "Task to cast a vote", skip(form))]
pub async fn cast_vote(
    form: web::Json<InitiativeVoteData>,
    matrix_client: web::Data<MatrixClient>,
) -> Result<impl Responder, ServerError> {
    on_handle_vote_initiative_message(form.0, &matrix_client)
        .await
        .context("Failed to cast a vote")?;

    Ok(web::Json(()))
}

#[tracing::instrument(name = "place vote initiative message", skip(matrix_client))]
pub async fn on_handle_vote_initiative_message(
    data: InitiativeVoteData,
    matrix_client: &MatrixClient,
) -> Result<(), Error> {
    let room = RoomId::parse(data.room)?;
    let room = matrix_client
        .get_joined_room(&room)
        .ok_or(anyhow::anyhow!("Room not found"))?;

    let content = serde_json::to_value(InitiativeVoteContent {
        user: data.user,
        vote: data.vote,
    })?;
    room.send_raw(content, "m.virto.initiative_vote", None)
        .await?;

    Ok(())
}

#[tracing::instrument(name = "Get initiative metadata", skip(path, matrix_client))]
pub async fn get_initiative_by_id(
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
) -> Result<InitiativeHistory, Error> {
    let room_id = RoomId::parse(id)?;

    let room = matrix_client
        .get_room(&room_id)
        .expect("should get the room");

    let filter = assign!(RoomEventFilter::default(), {
        lazy_load_options: LazyLoadOptions::Enabled { include_redundant_members: false },
    });
    let options = assign!(MessagesOptions::backward(), {
        filter,
        from: None
    });

    let m = room.messages(options).await?;

    let t = TimelineSlice::new(
        m.chunk.into_iter().map(SyncTimelineEvent::from).collect(),
        m.start,
        m.end.clone(),
        false,
        false,
    );

    let mut initiative_history = InitiativeHistory::default();

    for sync_timeline_event in t.events.iter() {
        let Ok(value) = serde_json::from_str::<Value>(sync_timeline_event.event.json().get())
        else {
            continue;
        };

        if let Ok(event) = serde_json::from_value::<
            OriginalSyncMessageLikeEvent<InitiativeVoteContent>,
        >(value.clone())
        {
            initiative_history.votes.push(event.content);
        };

        if let Ok(event) = serde_json::from_value::<
            OriginalSyncMessageLikeEvent<InitiativeInitContent>,
        >(value.clone())
        {
            initiative_history.init = event.content;
        };

        if let Ok(event) =
            serde_json::from_value::<OriginalSyncMessageLikeEvent<InitiativeInfoContent>>(value)
        {
            initiative_history.info = event.content;
        };
    }

    Ok(initiative_history)
}
