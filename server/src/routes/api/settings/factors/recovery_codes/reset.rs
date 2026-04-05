use axum::{Extension, Json};
use color_eyre::eyre::{self, ContextCompat};
use mongodb::bson::doc;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{RecoveryCodeFactor, User, get_user_by_id},
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::api::settings::factors::recovery_codes::{
        generate_recovery_codes, hash_recovery_codes,
    },
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(reset_recovery_codes))
}

#[derive(Serialize, ToSchema)]
pub struct ResetRecoveryCodesResponse {
    /// New recovery codes. Save them securely as they won't be shown again.
    pub codes: Vec<String>,
}

/// Reset recovery codes
///
/// Invalidates all existing recovery codes and generates a fresh set of 10.
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Success", body = ResetRecoveryCodesResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = BAD_REQUEST, description = "Recovery codes not enabled", body = String, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn reset_recovery_codes(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<ResetRecoveryCodesResponse>> {
    let user = get_user_by_id(&state.database, &user_id)
        .await?
        .wrap_err("User not found")?;

    if user.auth_factors.recovery_codes.is_empty() {
        return Err(AxumError::bad_request(eyre::eyre!(
            "Recovery codes are not enabled"
        )));
    }

    let codes = generate_recovery_codes(10, 12);
    let hashed_codes = hash_recovery_codes(codes.clone())?;

    let db_codes = hashed_codes
        .iter()
        .map(|code| RecoveryCodeFactor {
            code_hash: code.clone(),
            used: false,
        })
        .collect::<Vec<_>>();

    state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": *user_id },
            doc! { "$set": { "auth_factors.recovery_codes": db_codes } },
        )
        .await?;

    if let Some(mail) = &state.mail_service {
        let email = user.email.clone();
        let mail = mail.clone();
        tokio::spawn(async move {
            if let Err(e) = mail
                .send_factor_added(&email, "recovery codes (regenerated)")
                .await
            {
                tracing::warn!(error = ?e, "Failed to send factor notification");
            }
        });
    }

    Ok(Json(ResetRecoveryCodesResponse { codes }))
}
