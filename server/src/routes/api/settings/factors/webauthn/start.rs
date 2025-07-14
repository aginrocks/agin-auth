use axum::{Extension, Json};
use color_eyre::eyre::{Context, ContextCompat};
use serde::Serialize;
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::routes;
use webauthn_rs::prelude::CreationChallengeResponse;

use crate::{
    axum_error::AxumResult,
    database::get_user_by_id,
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/settings/factors/webauthn/start";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(webauthn_start_setup),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Start WebAuthn setup
///
/// Request a challenge to start the WebAuthn registration process. After receiving a response from the browser, the client should call the `/api/settings/factors/webauthn/finish` endpoint to complete the registration.
#[utoipa::path(
    method(get),
    path = PATH,
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
) -> AxumResult<Json<CreationChallengeResponse>> {
    let user = get_user_by_id(&state.database, &user_id)
        .await?
        .wrap_err("User not found")?;

    session.remove_value("reg_state").await?;

    // let exclude_credentials

    let (ccr, reg_state) = state
        .webauthn
        .start_passkey_registration(
            user.uuid,
            &user.preferred_username,
            &user.display_name,
            None,
        )
        .wrap_err("Challenge generation failed")?;

    session.insert("reg_state", reg_state).await?;

    Ok(Json(ccr))
}
