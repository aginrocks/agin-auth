use axum::{Extension, Json};
use color_eyre::eyre::{self, Context, ContextCompat};
use mongodb::bson::doc;
use serde::Serialize;
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::routes;
use webauthn_rs::prelude::*;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::User,
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/settings/factors/webauthn/finish";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(webauthn_start_setup),
        RouteProtectionLevel::Authenticated,
    )]
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "success": true }))]
struct WebAuthnFinishSuccess {
    success: bool,
}

/// Finish WebAuthn setup
///
/// Requires a previous call to `/api/settings/factors/webauthn/start` to initiate the registration process.
#[utoipa::path(
    method(post),
    path = PATH,
    request_body = crate::webauthn::types::RegisterPublicKeyCredential,
    responses(
        (status = OK, description = "Success", body = WebAuthnFinishSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn webauthn_start_setup(
    Extension(user_id): Extension<UserId>,
    Extension(state): Extension<AppState>,
    session: Session,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> AxumResult<Json<WebAuthnFinishSuccess>> {
    let reg_state: PasskeyRegistration = session.get("reg_state").await?.ok_or(AxumError::forbidden(eyre::eyre!("Missing WebAuthn registration session. Use the /api/settings/factors/webauthn/start endpoint first.")))?;

    session.remove_value("reg_state").await?;

    let sk = state
        .webauthn
        .finish_passkey_registration(&reg, &reg_state)
        .map_err(|e| AxumError::bad_request(eyre::eyre!("WebAuthn registration failed: {}", e)))?;

    let display_name: String = session
        .get("webauthn_display_name")
        .await?
        .wrap_err("Missing display name")?;

    let serialized_key = serde_json::to_string(&sk).wrap_err("Failed to serialize passkey")?;

    state
        .database
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "$id": *user_id },
            doc! {
                "$push": {
                    "auth_factors.webauthn": {
                        "serialized_key": serialized_key,
                        "display_name": display_name,
                    }
                }
            },
        )
        .await?;

    Ok(Json(WebAuthnFinishSuccess { success: true }))
}
