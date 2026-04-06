use axum::{Extension, Json, http::StatusCode};
use color_eyre::eyre::{self, OptionExt};
use mongodb::bson::doc;
use serde::Deserialize;
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::User,
    middlewares::require_auth::UserId,
    state::AppState,
    utils::verify_password,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(delete_account))
}

#[derive(Deserialize, ToSchema)]
struct DeleteAccountBody {
    /// Current password for confirmation
    password: String,
}

/// Delete account
///
/// Permanently deletes the user's account after verifying their password. This action is irreversible.
#[utoipa::path(
    method(delete),
    path = "/",
    request_body = DeleteAccountBody,
    responses(
        (status = NO_CONTENT, description = "Account deleted"),
        (status = UNAUTHORIZED, description = "Invalid password"),
    ),
    tag = "Settings"
)]
async fn delete_account(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    session: Session,
    Json(body): Json<DeleteAccountBody>,
) -> AxumResult<StatusCode> {
    let user = state
        .database
        .collection::<User>("users")
        .find_one(doc! { "_id": *user_id })
        .await?
        .ok_or_eyre("User not found")?;

    // Verify password
    let password_hash = user
        .auth_factors
        .password
        .password_hash
        .ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Password not set")))?;

    verify_password(&body.password, &password_hash)
        .map_err(|_| AxumError::unauthorized(eyre::eyre!("Invalid password")))?;

    // Delete user
    state
        .database
        .collection::<User>("users")
        .delete_one(doc! { "_id": *user_id })
        .await?;

    // Clean up related data
    let db = state.database.clone();
    let uid = *user_id;
    tokio::spawn(async move {
        let _ = db
            .collection::<mongodb::bson::Document>("email_confirmation_tokens")
            .delete_many(doc! { "user_id": uid })
            .await;
        let _ = db
            .collection::<mongodb::bson::Document>("password_reset_tokens")
            .delete_many(doc! { "user_id": uid })
            .await;
    });

    // Destroy session
    session.flush().await?;

    Ok(StatusCode::NO_CONTENT)
}
