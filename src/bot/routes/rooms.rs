use actix_web::ResponseError;
use actix_web::{web, HttpResponse, Responder};
use anyhow::{Context, Error};
use chrono::Utc;
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId, RoomId};
use matrix_sdk::ruma::UserId;
use matrix_sdk::ruma::{
    api::{self, client::room::Visibility},
    events::{
        room::{
            guest_access::{GuestAccess, RoomGuestAccessEventContent},
            join_rules::{JoinRule, RoomJoinRulesEventContent},
        },
        AnyInitialStateEvent, EmptyStateKey, InitialStateEvent,
    },
    serde::Raw,
};
use matrix_sdk::Client as MatrixClient;
use reqwest::header::LOCATION;
use reqwest::StatusCode;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::domain::RoomInvite;
use crate::utils::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum CreateError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for CreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CreateError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            CreateError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(name = "Task to reate a room", skip(db_client, matrix_client))]
pub async fn create(
    mut rx: mpsc::Receiver<()>,
    db_client: web::Data<SqlitePool>,
    matrix_client: web::Data<MatrixClient>,
) -> Result<(), CreateError> {
    while rx.recv().await.is_some() {
        let count = count_rooms(&db_client).await.unwrap();

        for _ in 0..(10 - count) {
            on_handle_create_room(&db_client, &matrix_client)
            .await
            .context("Failed to create a room")?;
        }
    }
    Ok(())
}

#[tracing::instrument(name = "Create a room", skip(db_client, matrix_client))]
pub async fn on_handle_create_room(
    db_client: &SqlitePool,
    matrix_client: &MatrixClient,
) -> Result<(), Error> {
    let mut request = api::client::room::create_room::v3::Request::new();

    let mut init_state_ev_vec: Vec<Raw<AnyInitialStateEvent>> = vec![];

    // join rule

    let room_join = RoomJoinRulesEventContent::new(JoinRule::Invite);
    let init_state_ev: InitialStateEvent<RoomJoinRulesEventContent> = InitialStateEvent {
        content: room_join,
        state_key: EmptyStateKey,
    };

    let raw_init_state_ev = Raw::new(&init_state_ev).unwrap();

    let raw_any_init_state_ev: Raw<AnyInitialStateEvent> = raw_init_state_ev.cast();
    init_state_ev_vec.push(raw_any_init_state_ev);

    // guest

    let guest_access = RoomGuestAccessEventContent::new(GuestAccess::CanJoin);
    let init_state_ev_guest: InitialStateEvent<RoomGuestAccessEventContent> = InitialStateEvent {
        content: guest_access,
        state_key: EmptyStateKey,
    };

    let raw_init_state_ev_guest = Raw::new(&init_state_ev_guest).unwrap();

    let raw_any_init_state_ev_guest: Raw<AnyInitialStateEvent> = raw_init_state_ev_guest.cast();
    init_state_ev_vec.push(raw_any_init_state_ev_guest);

    // others

    request.initial_state = &init_state_ev_vec;

    request.name = Some("test_room_10");
    request.is_direct = false;

    let visibility = Visibility::Private;
    request.visibility = visibility;

    let response = matrix_client.create_room(request).await?;

    store_room(&db_client, response.room_id.as_str()).await?;

    Ok(())
}

pub struct StoreRoomError(sqlx::Error);

impl std::fmt::Display for StoreRoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encoutered while \
            trying to store a room"
        )
    }
}

impl std::error::Error for StoreRoomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for StoreRoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[tracing::instrument(name = "Store room in the database", skip(room_id, db_client))]
pub async fn store_room(db_client: &SqlitePool, room_id: &str) -> Result<(), StoreRoomError> {
    sqlx::query!(
        r#"
            INSERT INTO rooms (id, room_id, available, created_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        room_id,
        true,
        Utc::now()
    )
    .execute(db_client)
    .await
    .map_err(|e|{
        tracing::error!("Failed to execute query: {:?}", e);
        StoreRoomError(e)
    })?;

    Ok(())
}

#[tracing::instrument(name = "Get the next room id", skip(db_client))]
pub async fn next(tx: web::Data<Sender<()>>, db_client: web::Data<SqlitePool>) -> HttpResponse {
    tx.send(()).await.expect("Failed to send message");

    match on_handle_next_room(&db_client).await {
        Ok(room_id) => {
            let relocation = format!("https://app.virto.dev/{}", room_id);
            HttpResponse::SeeOther()
                .insert_header((LOCATION, relocation))
                .finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Get last room from database", skip(db_client))]
pub async fn on_handle_next_room(db_client: &SqlitePool) -> Result<String, anyhow::Error> {
    let room = sqlx::query!(
        r#"
            SELECT * 
            FROM rooms
            WHERE available = true
            ORDER BY created_at DESC
            LIMIT 1
        "#
    )
    .fetch_one(db_client)
    .await
    .map(|row| row.room_id).map_err(|e|{
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(room)
}

#[tracing::instrument(name = "Count rooms from database", skip(db_client))]
pub async fn count_rooms(db_client: &SqlitePool) -> Result<i64, anyhow::Error> {
    let result = sqlx::query!(
        r#"
            SELECT Count(room_id) 
            FROM rooms
            WHERE available = true
        "#
    )
    .fetch_one(db_client)
    .await
    .map(|row| row.count)
    .map_err(|e|{
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    let count  = result.unwrap();

    Ok(count)
}

#[derive(thiserror::Error)]
pub enum InviteError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for InviteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for InviteError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            InviteError::ValidationError(_) => StatusCode::BAD_REQUEST,
            InviteError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

struct Invite {
    room_id: OwnedRoomId,
    user_id: OwnedUserId,
}

impl TryFrom<RoomInvite> for Invite {
    type Error = String;

    fn try_from(value: RoomInvite) -> Result<Self, Self::Error> {
        let room_id = RoomId::parse(&value.room_id).map_err(|_| format!("{} is not a room_id", value.room_id))?;
        let user_id = UserId::parse(&value.user_id).map_err(|_| format!("{} is not a user_id", value.user_id))?;

        Ok(Self { room_id, user_id })
    }
}

#[tracing::instrument(
    name = "Invite a user to a room", 
    skip(form, matrix_client) 
    fields(room_id=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn invite(
    form: web::Form<RoomInvite>,
    matrix_client: web::Data<MatrixClient>,
) -> Result<impl Responder, InviteError> {
    let invite = form.0.try_into().map_err(InviteError::ValidationError)?;
    on_handle_invite(invite, &matrix_client).await.context("Failed to invite a user")?;

    Ok(HttpResponse::Ok())
}

#[tracing::instrument(name = "Invite a user by id", skip(invite, matrix_client))]
pub async fn on_handle_invite(
    invite: Invite,
    matrix_client: &MatrixClient,
) -> Result<(), Error> {
    let room = matrix_client.get_joined_room(&invite.room_id).unwrap();
    room.invite_user_by_id(&invite.user_id).await?;

    Ok(())
}
