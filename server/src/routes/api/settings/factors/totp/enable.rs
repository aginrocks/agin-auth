mod confirm;

use axum::{Extension, Json};
use axum_valid::Valid;
use base32::{Alphabet, encode};
use color_eyre::eyre::{self, ContextCompat};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use totp_rs::Secret;
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{User, get_user_by_id},
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::{RouteProtectionLevel, api::settings::factors::totp::create_totp_instance},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/settings/factors/totp/enable";

pub fn routes() -> Vec<Route> {
    [
        vec![(routes!(enable_totp), RouteProtectionLevel::Authenticated)],
        confirm::routes(),
    ]
    .concat()
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct EnableTotpBody {
    /// The display name for the TOTP factor (for example authenticator app name).
    #[validate(length(min = 1, max = 32))]
    pub display_name: String,
}

#[derive(Serialize, ToSchema)]
pub struct EnableTotpResponse {
    /// The secret won't be shown again, so save it securely.
    pub secret: String,
    /// QR code URL that'll add the TOTP factor to your authenticator app. Won't be shown again.
    pub qr: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "error": "TOTP is already enabled. To rotate your TOTP secret, disable it first and then enable it again."
}))]
pub struct AlreadyEnabledError {
    pub error: String,
}

/// Enable TOTP
///
/// Generates TOTP secret and saves it. To fully enable TOTP, a call to `/api/settings/factors/totp/enable/confirm` is required.
#[utoipa::path(
    method(post),
    path = PATH,
    request_body = EnableTotpBody,
    responses(
        (status = OK, description = "Success", body = EnableTotpResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Already Enabled", body = AlreadyEnabledError, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn enable_totp(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Valid(Json(body)): Valid<Json<EnableTotpBody>>,
) -> AxumResult<Json<EnableTotpResponse>> {
    let user = get_user_by_id(&state.database, &user_id)
        .await?
        .wrap_err("User not found")?;

    let already_enabled = user
        .auth_factors
        .totp
        .is_some_and(|totp| totp.fully_enabled);

    if already_enabled {
        return Err(AxumError::forbidden(eyre::eyre!(
            "TOTP is already enabled. To rotate your TOTP secret, disable it first and then enable it again."
        )));
    }

    let raw_secret = Secret::generate_secret().to_bytes()?;
    let encoded_secret = encode(Alphabet::Rfc4648 { padding: false }, &raw_secret);

    state
        .database
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "_id": *user_id },
            doc! {
                "$set": {
                    "auth_factors.totp": {
                        "secret": &encoded_secret,
                        "display_name": body.display_name,
                        "fully_enabled": false,
                    }
                }
            },
        )
        .await?;

    let totp = create_totp_instance(
        &encoded_secret,
        Some(user.email),
        Some("Agin Auth".to_string()),
    )?;

    let qr = totp.get_url();

    Ok(Json(EnableTotpResponse {
        secret: encoded_secret,
        qr,
    }))
}
