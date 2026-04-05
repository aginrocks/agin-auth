use axum::{Extension, Json};
use color_eyre::eyre::Context;
use tower_sessions::Session;
use utoipa_axum::{router::OpenApiRouter, routes};
use webauthn_rs::prelude::RequestChallengeResponse;

use crate::{axum_error::AxumResult, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(passwordless_start))
}

/// Start discoverable (passwordless) WebAuthn authentication
///
/// Initiates a discoverable credential authentication flow. No username is required — the authenticator
/// will present the user with a list of available passkeys. After receiving the browser response,
/// call `/api/login/webauthn/passwordless/finish` to complete authentication.
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Challenge created", body = crate::webauthn::types::RequestChallengeResponse, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn passwordless_start(
    Extension(state): Extension<AppState>,
    session: Session,
) -> AxumResult<Json<RequestChallengeResponse>> {
    session.remove_value("discoverable_auth_state").await?;

    let (rcr, auth_state) = state
        .webauthn
        .start_discoverable_authentication()
        .wrap_err("Failed to start discoverable authentication")?;

    session
        .insert("discoverable_auth_state", auth_state)
        .await?;

    Ok(Json(rcr))
}
