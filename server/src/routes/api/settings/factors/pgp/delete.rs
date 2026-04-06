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
    OpenApiRouter::new().routes(routes!(delete_pgp_key))
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "success": true }))]
struct DeletePgpResponse {
    success: bool,
}

/// Delete PGP key
///
/// Removes a PGP key by its fingerprint.
#[utoipa::path(
    method(delete),
    path = "/{fingerprint}",
    params(
        ("fingerprint" = String, Path, description = "Fingerprint of the PGP key to delete")
    ),
    responses(
        (status = OK, description = "Success", body = DeletePgpResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Key not found", body = String, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn delete_pgp_key(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(fingerprint): Path<String>,
) -> AxumResult<Json<DeletePgpResponse>> {
    let result = state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": *user_id, "auth_factors.pgp.fingerprint": &fingerprint },
            doc! { "$pull": { "auth_factors.pgp": { "fingerprint": &fingerprint } } },
        )
        .await?;

    if result.matched_count == 0 {
        return Err(AxumError::not_found(eyre::eyre!("PGP key not found")));
    }

    if let Some(mail) = &state.mail_service {
        let user = get_user_by_id(&state.database, &user_id).await?;
        if let Some(user) = user {
            let email = user.email;
            let mail = mail.clone();
            tokio::spawn(async move {
                if let Err(e) = mail.send_factor_removed(&email, "PGP key").await {
                    tracing::warn!(error = ?e, "Failed to send factor removed notification");
                }
            });
        }
    }

    Ok(Json(DeletePgpResponse { success: true }))
}
