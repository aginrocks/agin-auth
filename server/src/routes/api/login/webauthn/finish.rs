use axum::{Extension, Json};
use color_eyre::eyre::{self, Context, ContextCompat, Result};
use mongodb::bson::doc;
use tower_sessions::Session;
use utoipa_axum::{router::OpenApiRouter, routes};
use webauthn_rs::prelude::*;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{User, WebAuthnFactor, get_user_by_id},
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::api::{AuthState, login::SuccessfulLoginResponse},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(webauthn_finish_login))
}

/// Finish WebAuthn 2FA
///
/// **This endpoint can only be used as a second factor.** For passwordless authentication, see `/api/login/webauthn/passwordless/start`. Requires a previous call to `/api/login/webauthn/start` to initiate the login process.
#[utoipa::path(
    method(post),
    path = "/",
    request_body = crate::webauthn::types::PublicKeyCredential,
    responses(
        (status = OK, description = "Success", body = SuccessfulLoginResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn webauthn_finish_login(
    Extension(user_id): Extension<UserId>,
    Extension(state): Extension<AppState>,
    session: Session,
    Json(auth): Json<PublicKeyCredential>,
) -> AxumResult<Json<SuccessfulLoginResponse>> {
    let user = get_user_by_id(&state.database, &user_id)
        .await?
        .wrap_err("User not found")?;

    let auth_state: PasskeyAuthentication =
        session
            .get("webauthn_login_state")
            .await?
            .ok_or(AxumError::forbidden(eyre::eyre!(
                "Missing WebAuthn login session. Use the /api/login/webauthn/start endpoint first."
            )))?;

    session.remove_value("webauthn_login_state").await?;

    let auth_result = state
        .webauthn
        .finish_passkey_authentication(&auth, &auth_state)
        .map_err(|e| AxumError::bad_request(eyre::eyre!("WebAuthn registration failed: {}", e)))?;

    let mut user_keys = user
        .auth_factors
        .webauthn
        .iter()
        .map(|f| -> Result<(WebAuthnFactor, Passkey)> {
            let passkey: Passkey = serde_json::from_str(&f.serialized_key)?;
            Ok((f.clone(), passkey))
        })
        .collect::<Result<Vec<(WebAuthnFactor, Passkey)>, _>>()?;

    user_keys.iter_mut().for_each(|(_, sk)| {
        sk.update_credential(&auth_result);
    });

    let serialized_keys = user_keys
        .iter()
        .map(|(factor, sk)| {
            Ok(WebAuthnFactor {
                serialized_key: serde_json::to_string(&sk)
                    .wrap_err("Failed to serialize passkey")?,
                ..factor.clone()
            })
        })
        .collect::<Result<Vec<WebAuthnFactor>>>()?;

    state
        .database
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "_id": *user_id },
            doc! {
                "$set": {
                    "auth_factors.webauthn": serialized_keys,
                }
            },
        )
        .await?;

    session
        .insert("auth_state", AuthState::Authenticated)
        .await?;

    Ok(Json(SuccessfulLoginResponse {
        two_factor_required: false,
        recent_factor: None,
        second_factors: None,
    }))
}
