use axum::{Extension, Json};
use base64urlsafedata::HumanBinaryData;
use color_eyre::eyre::{Context, ContextCompat, Result};
use serde::Deserialize;
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;
use webauthn_rs::prelude::{CreationChallengeResponse, CredentialID, Passkey};

use crate::{
    axum_error::AxumResult,
    database::get_user_by_id,
    middlewares::require_auth::{UnauthorizedError, UserId},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(webauthn_start_setup))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct WebAuthnRegistrationBody {
    #[validate(length(min = 1, max = 32))]
    pub display_name: String,
}

/// Start WebAuthn setup
///
/// Request a challenge to start the WebAuthn registration process. After receiving a response from the browser, the client should call the `/api/settings/factors/webauthn/finish` endpoint to complete the registration.
#[utoipa::path(
    method(post),
    path = "/",
    request_body = WebAuthnRegistrationBody,
    responses(
        (status = OK, description = "Success", body = crate::webauthn::types::CreationChallengeResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn webauthn_start_setup(
    Extension(user_id): Extension<UserId>,
    Extension(state): Extension<AppState>,
    session: Session,
    Json(body): Json<WebAuthnRegistrationBody>,
) -> AxumResult<Json<CreationChallengeResponse>> {
    let user = get_user_by_id(&state.database, &user_id)
        .await?
        .wrap_err("User not found")?;

    session.remove_value("reg_state").await?;

    let exclude_credentials = user
        .auth_factors
        .webauthn
        .iter()
        .map(|f| -> Result<CredentialID> {
            let passkey: Passkey = serde_json::from_str(&f.serialized_key)?;
            Ok(passkey.cred_id().clone())
        })
        .collect::<Result<Vec<HumanBinaryData>, _>>()?;

    let (ccr, reg_state) = state
        .webauthn
        .start_passkey_registration(
            user.uuid,
            &user.preferred_username,
            &user.display_name,
            Some(exclude_credentials),
        )
        .wrap_err("Challenge generation failed")?;

    session.insert("reg_state", reg_state).await?;
    session
        .insert("webauthn_display_name", body.display_name)
        .await?;

    Ok(Json(ccr))
}
