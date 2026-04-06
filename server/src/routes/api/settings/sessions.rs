use axum::{Extension, Json};
use color_eyre::eyre;
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tower_sessions_redis_store::fred::prelude::KeysInterface;
use tracing::warn;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::SessionRecord,
    middlewares::require_auth::UserId,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_sessions))
        .routes(routes!(delete_session))
}

#[derive(Serialize, ToSchema)]
struct SessionItem {
    id: String,
    ip_address: String,
    user_agent: String,
    created_at: String,
    last_active: String,
    current: bool,
}

#[derive(Serialize, ToSchema)]
struct SessionsResponse {
    sessions: Vec<SessionItem>,
}

async fn ensure_public_session_id(
    state: &AppState,
    session_key: &str,
    public_id: Option<String>,
) -> AxumResult<String> {
    if let Some(public_id) = public_id {
        return Ok(public_id);
    }

    let public_id = Uuid::new_v4().to_string();
    let update_result = state
        .database
        .collection::<SessionRecord>("sessions")
        .update_one(
            doc! {
                "_id": session_key,
                "public_id": { "$exists": false },
            },
            doc! {
                "$set": {
                    "public_id": &public_id,
                }
            },
        )
        .await?;

    if update_result.modified_count == 0 {
        let record = state
            .database
            .collection::<SessionRecord>("sessions")
            .find_one(doc! { "_id": session_key })
            .await?;
        if let Some(public_id) = record.and_then(|record| record.public_id) {
            return Ok(public_id);
        }
    }

    Ok(public_id)
}

/// List active sessions
///
/// Returns all active sessions for the current user.
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Sessions list", body = SessionsResponse, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn list_sessions(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    session: Session,
) -> AxumResult<Json<SessionsResponse>> {
    let current_session_id = session.id().map(|id| id.to_string()).unwrap_or_default();

    let mut cursor = state
        .database
        .collection::<SessionRecord>("sessions")
        .find(doc! { "user_id": *user_id })
        .sort(doc! { "last_active": -1_i32 })
        .await?;

    let mut sessions = Vec::new();
    let mut stale_session_keys = Vec::new();
    while let Some(record) = cursor.try_next().await? {
        let exists: i64 = state.redis_pool.exists(&record.id).await.map_err(|error| {
            AxumError::new(eyre::eyre!("Failed to validate session state: {}", error))
        })?;

        if exists == 0 {
            stale_session_keys.push(record.id);
            continue;
        }

        let public_id = ensure_public_session_id(&state, &record.id, record.public_id).await?;
        sessions.push(SessionItem {
            id: public_id,
            ip_address: record.ip_address,
            user_agent: record.user_agent,
            created_at: record
                .created_at
                .try_to_rfc3339_string()
                .unwrap_or_default(),
            last_active: record
                .last_active
                .try_to_rfc3339_string()
                .unwrap_or_default(),
            current: record.id == current_session_id,
        });
    }

    if !stale_session_keys.is_empty() {
        state
            .database
            .collection::<SessionRecord>("sessions")
            .delete_many(doc! {
                "_id": { "$in": stale_session_keys }
            })
            .await?;
    }

    // Sort: current session first, then by last_active desc
    sessions.sort_by(|a, b| {
        b.current
            .cmp(&a.current)
            .then(b.last_active.cmp(&a.last_active))
    });

    Ok(Json(SessionsResponse { sessions }))
}

#[derive(Deserialize, ToSchema)]
struct DeleteSessionPath {
    session_id: String,
}

#[derive(Serialize, ToSchema)]
struct DeleteSessionResponse {
    success: bool,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "error": "Session not found" }))]
struct SessionErrorResponse {
    error: String,
}

/// Revoke a session
///
/// Deletes a specific session by ID. Cannot revoke the current session (use logout instead).
#[utoipa::path(
    method(delete),
    path = "/{session_id}",
    params(
        ("session_id" = String, Path, description = "The session ID to revoke"),
    ),
    responses(
        (status = OK, description = "Session revoked", body = DeleteSessionResponse, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Cannot revoke current session", body = SessionErrorResponse, content_type = "application/json"),
        (status = NOT_FOUND, description = "Session not found", body = SessionErrorResponse, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn delete_session(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    session: Session,
    axum::extract::Path(path): axum::extract::Path<DeleteSessionPath>,
) -> AxumResult<Json<DeleteSessionResponse>> {
    let current_session_id = session.id().map(|id| id.to_string()).unwrap_or_default();
    let record = state
        .database
        .collection::<SessionRecord>("sessions")
        .find_one(doc! {
            "public_id": &path.session_id,
            "user_id": *user_id,
        })
        .await?
        .ok_or_else(|| AxumError::not_found(eyre::eyre!("Session not found")))?;

    if record.id == current_session_id {
        return Err(AxumError::bad_request(eyre::eyre!(
            "Cannot revoke the current session. Use logout instead."
        )));
    }

    let _: i64 = state
        .redis_pool
        .del(&record.id)
        .await
        .map_err(|e| AxumError::new(eyre::eyre!("Failed to invalidate session: {}", e)))?;

    if let Err(error) = state
        .database
        .collection::<SessionRecord>("sessions")
        .delete_one(doc! { "_id": &record.id })
        .await
    {
        warn!(
            error = ?error,
            public_id = %path.session_id,
            "Failed to clean up session record after revoke"
        );
    }

    Ok(Json(DeleteSessionResponse { success: true }))
}
