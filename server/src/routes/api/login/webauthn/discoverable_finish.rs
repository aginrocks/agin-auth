use axum::{Extension, Json};
use color_eyre::eyre::{self, Context, Result};
use mongodb::bson::doc;
use tower_sessions::Session;
use utoipa_axum::{router::OpenApiRouter, routes};
use webauthn_rs::prelude::*;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{User, WebAuthnFactor, get_user_by_uuid},
    middlewares::require_auth::UnauthorizedError,
    routes::api::{AuthState, login::SuccessfulLoginResponse},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(discoverable_finish))
}

/// Finish discoverable (passwordless) WebAuthn authentication
///
/// Completes the discoverable credential authentication flow. Requires a previous call to
/// `/api/login/webauthn/passwordless/start`. The server identifies the user from the credential's
/// embedded user handle (UUID).
#[utoipa::path(
    method(post),
    path = "/",
    request_body = crate::webauthn::types::PublicKeyCredential,
    responses(
        (status = OK, description = "Success", body = SuccessfulLoginResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Invalid credential or user not found", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn discoverable_finish(
    Extension(state): Extension<AppState>,
    session: Session,
    Json(auth): Json<PublicKeyCredential>,
) -> AxumResult<Json<SuccessfulLoginResponse>> {
    let auth_state: DiscoverableAuthentication = session
        .get("discoverable_auth_state")
        .await?
        .ok_or(AxumError::forbidden(eyre::eyre!(
            "Missing discoverable auth session. Use /api/login/webauthn/passwordless/start first."
        )))?;

    session.remove_value("discoverable_auth_state").await?;

    let (user_uuid, _cred_id) = state
        .webauthn
        .identify_discoverable_authentication(&auth)
        .map_err(|e| {
            AxumError::unauthorized(eyre::eyre!("Failed to identify credential: {}", e))
        })?;

    let user = get_user_by_uuid(&state.database, &user_uuid)
        .await?
        .ok_or(AxumError::unauthorized(eyre::eyre!(
            "User not found for this credential"
        )))?;

    let discoverable_keys = user
        .auth_factors
        .webauthn
        .iter()
        .map(|f| -> Result<DiscoverableKey> {
            let passkey: Passkey = serde_json::from_str(&f.serialized_key)?;
            Ok(passkey.into())
        })
        .collect::<Result<Vec<DiscoverableKey>, _>>()?;

    if discoverable_keys.is_empty() {
        return Err(AxumError::unauthorized(eyre::eyre!(
            "No WebAuthn credentials found for user"
        )));
    }

    let auth_result = state
        .webauthn
        .finish_discoverable_authentication(&auth, auth_state, &discoverable_keys)
        .map_err(|e| {
            AxumError::unauthorized(eyre::eyre!("Discoverable authentication failed: {}", e))
        })?;

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
            doc! { "_id": user.id },
            doc! {
                "$set": {
                    "auth_factors.webauthn": serialized_keys,
                }
            },
        )
        .await?;

    session.insert("user_id", user.id).await?;
    session
        .insert("auth_state", AuthState::Authenticated)
        .await?;

    Ok(Json(SuccessfulLoginResponse {
        two_factor_required: false,
        recent_factor: None,
        second_factors: None,
    }))
}
