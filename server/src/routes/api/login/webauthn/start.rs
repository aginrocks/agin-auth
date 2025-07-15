use axum::{Extension, Json};
use color_eyre::eyre::{self, Context, ContextCompat, Result};
use tower_sessions::Session;
use utoipa_axum::routes;
use webauthn_rs::prelude::{Passkey, RequestChallengeResponse};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::get_user_by_id,
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/login/webauthn/start";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(webauthn_start_login),
        RouteProtectionLevel::BeforeTwoFactor,
    )]
}

/// Start WebAuthn 2FA
///
/// **This endpoint can only be used as a second factor.** For passwordless authentication, see `/api/login/webauthn/passwordless/start`. Request a challenge to start the WebAuthn login process. After receiving a response from the browser, the client should call the `/api/login/webauthn/finish` endpoint to complete the login.
#[utoipa::path(
    method(post),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = crate::webauthn::types::RequestChallengeResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn webauthn_start_login(
    Extension(user_id): Extension<UserId>,
    Extension(state): Extension<AppState>,
    session: Session,
) -> AxumResult<Json<RequestChallengeResponse>> {
    let user = get_user_by_id(&state.database, &user_id)
        .await?
        .wrap_err("User not found")?;

    session.remove_value("webauthn_login_state").await?;

    let allow_credentials = user
        .auth_factors
        .webauthn
        .iter()
        .map(|f| -> Result<Passkey> {
            let passkey: Passkey = serde_json::from_str(&f.serialized_key)?;
            Ok(passkey)
        })
        .collect::<Result<Vec<Passkey>, _>>()?;

    if allow_credentials.is_empty() {
        return Err(AxumError::unauthorized(eyre::eyre!(
            "No WebAuthn credentials found for user."
        )));
    }

    let (rcr, auth_state) = state
        .webauthn
        .start_passkey_authentication(&allow_credentials)
        .wrap_err("Challenge generation failed")?;

    session.insert("webauthn_login_state", auth_state).await?;

    Ok(Json(rcr))
}
