use axum::{
    Extension,
    extract::Query,
    response::{IntoResponse, Redirect},
};
use chrono::{DateTime, Duration, Utc};
use mongodb::bson::{DateTime as BsonDateTime, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::User,
    state::AppState,
    utils::{generate_reset_token, hash_token},
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(confirm_email))
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
        tracing::info!(%user_id, "Mail service not configured, skipping confirmation email");
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

    if let Err(error) = mail.send_email_confirmation(email, &token).await {
        tracing::warn!(error = ?error, %user_id, "Failed to send confirmation email");
        return Err(AxumError::service_unavailable(color_eyre::eyre::eyre!(
            "Confirmation email service is unavailable"
        )));
    }

    Ok(())
}

#[derive(Deserialize, IntoParams)]
struct ConfirmEmailQuery {
    /// Confirmation token from the email link
    token: String,
}

/// Confirm email address
///
/// Validates the confirmation token, marks the email as confirmed, and redirects to the frontend result page.
#[utoipa::path(
    method(get),
    path = "/",
    params(ConfirmEmailQuery),
    responses(
        (status = 302, description = "Redirects to frontend with status"),
    ),
    tag = "Email Confirmation"
)]
async fn confirm_email(
    Extension(state): Extension<AppState>,
    Query(query): Query<ConfirmEmailQuery>,
) -> impl IntoResponse {
    let redirect_error =
        |reason: &str| Redirect::temporary(&format!("/confirm-email?status=error&reason={reason}"));

    let token_hash = hash_token(&query.token);

    let token_doc = match state
        .database
        .collection::<EmailConfirmationToken>("email_confirmation_tokens")
        .find_one_and_delete(doc! { "token_hash": &token_hash })
        .await
    {
        Ok(Some(doc)) => doc,
        Ok(None) => return redirect_error("invalid"),
        Err(e) => {
            tracing::warn!(error = ?e, "Failed to look up email confirmation token");
            return redirect_error("invalid");
        }
    };

    if Utc::now() > token_doc.expires_at {
        // Clean up other expired tokens in the background
        let db = state.database.clone();
        tokio::spawn(async move {
            let now = BsonDateTime::now();
            if let Err(e) = db
                .collection::<EmailConfirmationToken>("email_confirmation_tokens")
                .delete_many(doc! { "expires_at": { "$lt": now } })
                .await
            {
                tracing::warn!(error = ?e, "Failed to clean up expired email confirmation tokens");
            }
        });
        return redirect_error("expired");
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
        Ok(r) if r.matched_count > 0 => Redirect::temporary("/confirm-email?status=success"),
        Ok(_) => redirect_error("not_found"),
        Err(e) => {
            tracing::warn!(error = ?e, "Failed to update user email confirmation status");
            redirect_error("invalid")
        }
    }
}
