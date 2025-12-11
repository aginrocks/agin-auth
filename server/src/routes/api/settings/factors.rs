pub mod pgp;
pub mod recovery_codes;
pub mod totp;
pub mod webauthn;

use axum::{Extension, Json};
use color_eyre::eyre::ContextCompat;
use mongodb::bson::doc;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    database::{PublicAuthFactors, User},
    middlewares::require_auth::{UnauthorizedError, UserId},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_factors))
        .nest("/totp", totp::routes())
        .nest("/recovery-codes", recovery_codes::routes())
        .nest("/pgp", pgp::routes())
        .nest("/webauthn", webauthn::routes())
}

/// Get factors
///
/// Gets all authentication factors for the current user.
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = PublicAuthFactors, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn get_factors(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<PublicAuthFactors>> {
    let user = state
        .database
        .collection::<User>("users")
        .find_one(doc! {
            "_id": *user_id
        })
        .await?
        .wrap_err("User not found")?;

    let public_factors = user.auth_factors.to_public();

    Ok(Json(public_factors))
}
