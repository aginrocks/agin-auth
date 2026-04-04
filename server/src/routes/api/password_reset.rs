use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Extension, Json};
use axum_valid::Valid;
use chrono::{DateTime, Duration, Utc};
use color_eyre::eyre::{self, ContextCompat};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::User,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(request_reset))
        .routes(routes!(confirm_reset))
}

// ── DB document ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
struct PasswordResetToken {
    token: String,
    user_id: ObjectId,
    expires_at: DateTime<Utc>,
}

// ── Request reset ─────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema, Validate)]
struct RequestResetBody {
    #[validate(email, length(max = 128))]
    email: String,
}

#[derive(Serialize, ToSchema)]
struct RequestResetResponse {
    success: bool,
}

/// Request password reset
///
/// Sends a password reset email if the given address is registered.
/// Always returns success to prevent user enumeration.
#[utoipa::path(
    method(post),
    path = "/",
    request_body = RequestResetBody,
    responses(
        (status = OK, description = "Email sent (or address not found — always succeeds)", body = RequestResetResponse, content_type = "application/json"),
        (status = SERVICE_UNAVAILABLE, description = "Mail not configured", body = String, content_type = "application/json"),
    ),
    tag = "Password Reset"
)]
async fn request_reset(
    Extension(state): Extension<AppState>,
    Valid(Json(body)): Valid<Json<RequestResetBody>>,
) -> AxumResult<Json<RequestResetResponse>> {
    let Some(mail) = &state.mail_service else {
        return Err(AxumError::service_unavailable(eyre::eyre!(
            "Mail is not configured"
        )));
    };

    let user = state
        .database
        .collection::<User>("users")
        .find_one(doc! { "email": &body.email })
        .await?;

    // Always return success — don't reveal whether the email exists
    let Some(user) = user else {
        return Ok(Json(RequestResetResponse { success: true }));
    };

    let token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(1);

    state
        .database
        .collection::<PasswordResetToken>("password_reset_tokens")
        .insert_one(PasswordResetToken {
            token: token.clone(),
            user_id: user.id,
            expires_at,
        })
        .await?;

    // Best-effort — don't fail the request if mail delivery fails
    if let Err(e) = mail.send_password_reset(&user.email, &token).await {
        tracing::warn!(error = ?e, "Failed to send password reset email");
    }

    Ok(Json(RequestResetResponse { success: true }))
}

// ── Confirm reset ─────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema, Validate)]
struct ConfirmResetBody {
    token: String,

    #[validate(length(min = 8))]
    new_password: String,
}

#[derive(Serialize, ToSchema)]
struct ConfirmResetResponse {
    success: bool,
}

/// Confirm password reset
///
/// Validates the token and sets the new password. Tokens expire after 1 hour
/// and are deleted on use.
#[utoipa::path(
    method(post),
    path = "/confirm",
    request_body = ConfirmResetBody,
    responses(
        (status = OK, description = "Password updated", body = ConfirmResetResponse, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Invalid or expired token", body = String, content_type = "application/json"),
    ),
    tag = "Password Reset"
)]
async fn confirm_reset(
    Extension(state): Extension<AppState>,
    Valid(Json(body)): Valid<Json<ConfirmResetBody>>,
) -> AxumResult<Json<ConfirmResetResponse>> {
    let token_doc = state
        .database
        .collection::<PasswordResetToken>("password_reset_tokens")
        .find_one(doc! { "token": &body.token })
        .await?;

    let Some(token_doc) = token_doc else {
        return Err(AxumError::bad_request(eyre::eyre!("Invalid or expired token")));
    };

    if Utc::now() > token_doc.expires_at {
        // Clean up expired token
        state
            .database
            .collection::<PasswordResetToken>("password_reset_tokens")
            .delete_one(doc! { "token": &body.token })
            .await?;
        return Err(AxumError::bad_request(eyre::eyre!("Invalid or expired token")));
    }

    // Hash the new password
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(body.new_password.as_bytes(), &salt)
        .map_err(|_| eyre::eyre!("Failed to hash password"))?
        .to_string();

    // Update user password
    state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": token_doc.user_id },
            doc! { "$set": { "auth_factors.password.password_hash": &hash } },
        )
        .await?
        .matched_count
        .eq(&1)
        .then_some(())
        .wrap_err("User not found")?;

    // Delete used token
    state
        .database
        .collection::<PasswordResetToken>("password_reset_tokens")
        .delete_one(doc! { "token": &body.token })
        .await?;

    Ok(Json(ConfirmResetResponse { success: true }))
}
