use axum::{Extension, Json, extract::Path};
use color_eyre::eyre;
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
    OpenApiRouter::new().routes(routes!(delete_webauthn))
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "success": true }))]
struct DeleteWebAuthnResponse {
    success: bool,
}

/// Delete WebAuthn key
///
/// Removes a WebAuthn passkey by its display name.
#[utoipa::path(
    method(delete),
    path = "/{display_name}",
    params(
        ("display_name" = String, Path, description = "Display name of the WebAuthn key to delete")
    ),
    responses(
        (status = OK, description = "Success", body = DeleteWebAuthnResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Key not found", body = String, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn delete_webauthn(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(display_name): Path<String>,
) -> AxumResult<Json<DeleteWebAuthnResponse>> {
    let result = state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": *user_id, "auth_factors.webauthn.display_name": &display_name },
            doc! { "$pull": { "auth_factors.webauthn": { "display_name": &display_name } } },
        )
        .await?;

    if result.matched_count == 0 {
        return Err(AxumError::not_found(eyre::eyre!("WebAuthn key not found")));
    }

    if let Some(mail) = &state.mail_service {
        let user = get_user_by_id(&state.database, &user_id).await?;
        if let Some(user) = user {
            let email = user.email;
            let mail = mail.clone();
            tokio::spawn(async move {
                if let Err(e) = mail.send_factor_removed(&email, "WebAuthn passkey").await {
                    tracing::warn!(error = ?e, "Failed to send factor removed notification");
                }
            });
        }
    }

    Ok(Json(DeleteWebAuthnResponse { success: true }))
}
