use axum::{Extension, Json};
use axum_valid::Valid;
use color_eyre::eyre::{self, ContextCompat};
use mongodb::bson::doc;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{User, get_user_by_id},
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::api::settings::factors::totp::{TotpCodeBody, verify_totp},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(confirm_enabling_totp))
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "success": true }))]
pub struct ConfirmTotpResponse {
    pub success: bool,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "error": "TOTP is already enabled. To rotate your TOTP secret, disable it first and then enable it again."
}))]
pub struct AlreadyEnabledError {
    pub error: String,
}

/// Confirm enabling TOTP
///
/// Confirm enabling TOTP by providing the TOTP code.
#[utoipa::path(
    method(post),
    path = "/",
    request_body = TotpCodeBody,
    responses(
        (status = OK, description = "Success", body = ConfirmTotpResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Already Enabled", body = AlreadyEnabledError, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn confirm_enabling_totp(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Valid(Json(body)): Valid<Json<TotpCodeBody>>,
) -> AxumResult<Json<ConfirmTotpResponse>> {
    let user = get_user_by_id(&state.database, &user_id)
        .await?
        .wrap_err("User not found")?;

    let already_enabled = user
        .clone()
        .auth_factors
        .totp
        .is_some_and(|totp| totp.fully_enabled);

    if already_enabled {
        return Err(AxumError::forbidden(eyre::eyre!(
            "TOTP is already enabled. To rotate your TOTP secret, disable it first and then enable it again."
        )));
    }

    let secret = user
        .auth_factors
        .totp
        .ok_or(AxumError::forbidden(eyre::eyre!(
            "TOTP secret is not yet generated"
        )))?
        .secret;

    verify_totp(&secret, &body.code)?;

    state
        .database
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "_id": *user_id },
            doc! {
                "$set": {
                    "auth_factors.totp.fully_enabled": true,
                }
            },
        )
        .await?;

    Ok(Json(ConfirmTotpResponse { success: true }))
}
