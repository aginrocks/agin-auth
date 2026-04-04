use axum::{Extension, Json, extract::Query, response::{Html, IntoResponse}};
use axum_valid::Valid;
use chrono::{DateTime, Duration, Utc};
use color_eyre::eyre;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::User,
    state::AppState,
    utils::{generate_reset_token, hash_token},
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(confirm_email))
        .routes(routes!(confirm_email_get))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfirmationToken {
    pub token_hash: String,
    pub user_id: ObjectId,
    pub expires_at: DateTime<Utc>,
}

pub async fn send_confirmation_email(
    state: &AppState,
    user_id: ObjectId,
    email: &str,
) -> AxumResult<()> {
    let Some(mail) = &state.mail_service else {
        return Ok(());
    };

    let token = generate_reset_token();
    let token_hash = hash_token(&token);
    let expires_at = Utc::now() + Duration::hours(24);

    state
        .database
        .collection::<EmailConfirmationToken>("email_confirmation_tokens")
        .insert_one(EmailConfirmationToken {
            token_hash,
            user_id,
            expires_at,
        })
        .await?;

    let mail = mail.clone();
    let email = email.to_owned();
    tokio::spawn(async move {
        if let Err(e) = mail.send_email_confirmation(&email, &token).await {
            tracing::warn!(error = ?e, "Failed to send confirmation email");
        }
    });

    Ok(())
}

#[derive(Deserialize, ToSchema, Validate)]
struct ConfirmEmailBody {
    token: String,
}

#[derive(Serialize, ToSchema)]
struct ConfirmEmailResponse {
    success: bool,
}

/// Confirm email address
///
/// Validates the confirmation token and marks the user's email as confirmed. Tokens expire after 24 hours and are deleted on use.
#[utoipa::path(
    method(post),
    path = "/",
    request_body = ConfirmEmailBody,
    responses(
        (status = OK, description = "Email confirmed", body = ConfirmEmailResponse, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Invalid or expired token", body = String, content_type = "application/json"),
    ),
    tag = "Email Confirmation"
)]
async fn confirm_email(
    Extension(state): Extension<AppState>,
    Valid(Json(body)): Valid<Json<ConfirmEmailBody>>,
) -> AxumResult<Json<ConfirmEmailResponse>> {
    let token_hash = hash_token(&body.token);

    let token_doc = state
        .database
        .collection::<EmailConfirmationToken>("email_confirmation_tokens")
        .find_one_and_delete(doc! { "token_hash": &token_hash })
        .await?;

    let Some(token_doc) = token_doc else {
        return Err(AxumError::bad_request(eyre::eyre!("Invalid or expired token")));
    };

    if Utc::now() > token_doc.expires_at {
        // Clean up other expired tokens in the background
        let db = state.database.clone();
        tokio::spawn(async move {
            let now = mongodb::bson::DateTime::now();
            if let Err(e) = db
                .collection::<EmailConfirmationToken>("email_confirmation_tokens")
                .delete_many(doc! { "expires_at": { "$lt": now } })
                .await
            {
                tracing::warn!(error = ?e, "Failed to clean up expired email confirmation tokens");
            }
        });
        return Err(AxumError::bad_request(eyre::eyre!("Invalid or expired token")));
    }

    let result = state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": token_doc.user_id },
            doc! { "$set": { "email_confirmed": true } },
        )
        .await?;

    if result.matched_count == 0 {
        return Err(AxumError::not_found(eyre::eyre!("User not found")));
    }

    Ok(Json(ConfirmEmailResponse { success: true }))
}

#[derive(Deserialize)]
struct ConfirmEmailQuery {
    token: String,
}

/// Confirm email address (GET — link from email)
#[utoipa::path(
    method(get),
    path = "/",
    params(("token" = String, Query, description = "Confirmation token")),
    responses(
        (status = OK, description = "Email confirmed"),
        (status = BAD_REQUEST, description = "Invalid or expired token"),
    ),
    tag = "Email Confirmation"
)]
async fn confirm_email_get(
    Extension(state): Extension<AppState>,
    Query(query): Query<ConfirmEmailQuery>,
) -> impl IntoResponse {
    let token_hash = hash_token(&query.token);

    let token_doc = state
        .database
        .collection::<EmailConfirmationToken>("email_confirmation_tokens")
        .find_one_and_delete(doc! { "token_hash": &token_hash })
        .await;

    let Ok(Some(token_doc)) = token_doc else {
        return Html("<h2>Invalid or expired confirmation link.</h2>").into_response();
    };

    if Utc::now() > token_doc.expires_at {
        return Html("<h2>This confirmation link has expired.</h2>").into_response();
    }

    let result = state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": token_doc.user_id },
            doc! { "$set": { "email_confirmed": true } },
        )
        .await;

    match result {
        Ok(r) if r.matched_count > 0 => {
            Html("<h2>Email confirmed! You can close this tab.</h2>").into_response()
        }
        _ => Html("<h2>User not found.</h2>").into_response(),
    }
}
