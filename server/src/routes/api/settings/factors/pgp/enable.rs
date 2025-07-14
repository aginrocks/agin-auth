use axum::{Extension, Json};
use axum_valid::Valid;
use color_eyre::eyre;
use mongodb::bson::doc;
use pgp::{
    composed::{ArmorOptions, Deserializable, SignedPublicKey},
    types::KeyDetails,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::User,
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/settings/factors/pgp/enable";

pub fn routes() -> Vec<Route> {
    [vec![(
        routes!(enable_pgp),
        RouteProtectionLevel::Authenticated,
    )]]
    .concat()
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct EnablePgpBody {
    /// The display name for the TOTP factor (for example authenticator app name).
    #[validate(length(min = 1, max = 32))]
    pub display_name: String,

    // The public key
    #[validate(length(min = 1))]
    pub public_key: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "success": true }))]
pub struct EnablePgpResponse {
    pub success: bool,
}

/// Enable PGP
///
/// Enables PGP authentication factor for the user. This factor can only be used as a first factor.
#[utoipa::path(
    method(post),
    path = PATH,
    request_body = EnablePgpBody,
    responses(
        (status = OK, description = "Success", body = EnablePgpResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn enable_pgp(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Valid(Json(body)): Valid<Json<EnablePgpBody>>,
) -> AxumResult<Json<EnablePgpResponse>> {
    let (public_key, _) = SignedPublicKey::from_string(&body.public_key)
        .map_err(|_| AxumError::bad_request(eyre::eyre!("Invalid public key")))?;

    state
        .database
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "_id": *user_id },
            doc! {
                "$set": {
                    "auth_factors.pgp": {
                        "public_key": public_key.to_armored_string(ArmorOptions::default())?,
                        "fingerprint": public_key.fingerprint().to_string(),
                        "display_name": body.display_name,
                    }
                }
            },
        )
        .await?;

    Ok(Json(EnablePgpResponse { success: true }))
}
