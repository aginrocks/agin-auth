use axum::{Extension, Json};
use color_eyre::eyre;
use mongodb::bson::doc;
use tower_sessions::Session;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{SecondFactor, get_user_by_id, set_recent_factor},
    middlewares::require_auth::UserId,
    routes::api::{
        AuthState,
        login::SuccessfulLoginResponse,
        settings::factors::totp::{Invalid2faCode, TotpCodeBody, verify_totp},
    },
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(login_with_totp))
}

/// Log in with TOTP
///
/// **This endpoint can only be used as a second factor.** TOTP is not considered secure enough to be used as a primary authentication method.
#[utoipa::path(
    method(post),
    path = "/",
    request_body = TotpCodeBody,
    responses(
        (status = OK, description = "Success", body = SuccessfulLoginResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = Invalid2faCode, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn login_with_totp(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    session: Session,
    Json(body): Json<TotpCodeBody>,
) -> AxumResult<Json<SuccessfulLoginResponse>> {
    let user = get_user_by_id(&state.database, &user_id).await?;

    if user.is_none()
        || !user
            .clone()
            .unwrap()
            .auth_factors
            .totp
            .clone()
            .is_some_and(|totp| totp.fully_enabled)
    {
        return Err(AxumError::unauthorized(eyre::eyre!("Invalid 2FA code")));
    }

    let user = user.unwrap();

    verify_totp(&user.auth_factors.totp.unwrap().secret, &body.code)?;

    set_recent_factor(&state.database, &user_id, SecondFactor::Totp.into()).await?;

    session
        .insert("auth_state", AuthState::Authenticated)
        .await?;

    Ok(Json(SuccessfulLoginResponse {
        two_factor_required: false,
        second_factors: None,
        recent_factor: None,
    }))
}
