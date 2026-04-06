use axum::{Extension, Json};
use color_eyre::eyre::{self, ContextCompat};
use mongodb::bson::doc;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{User, get_user_by_id},
    middlewares::require_auth::{UnauthorizedError, UserId},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(disable_pgp))
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "success": true }))]
struct DisablePgpResponse {
    success: bool,
}

/// Disable PGP
///
/// Removes the PGP authentication factor from the user's account.
#[utoipa::path(
    method(delete),
    path = "/",
    responses(
        (status = OK, description = "Success", body = DisablePgpResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = BAD_REQUEST, description = "PGP not enabled", body = String, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn disable_pgp(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<DisablePgpResponse>> {
    let user = get_user_by_id(&state.database, &user_id)
        .await?
        .wrap_err("User not found")?;

    if user.auth_factors.pgp.is_empty() {
        return Err(AxumError::bad_request(eyre::eyre!("PGP is not enabled")));
    }

    state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": *user_id },
            doc! { "$set": { "auth_factors.pgp": [] } },
        )
        .await?;

    if let Some(mail) = &state.mail_service {
        let email = user.email.clone();
        let mail = mail.clone();
        tokio::spawn(async move {
            if let Err(e) = mail.send_factor_removed(&email, "PGP key").await {
                tracing::warn!(error = ?e, "Failed to send factor removed notification");
            }
        });
    }

    Ok(Json(DisablePgpResponse { success: true }))
}
