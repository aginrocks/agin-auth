use axum::{Extension, Json};
use color_eyre::eyre::{self};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{SecondFactor, User, get_user_by_id, set_recent_factor},
    middlewares::require_auth::UserId,
    routes::api::{AuthState, settings::factors::recovery_codes::verify_recovery_code},
    state::AppState,
};

use super::SuccessfulLoginResponse;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(login_with_recovery_code))
}

#[derive(Deserialize, ToSchema)]
struct RecoveryCodeLoginBody {
    code: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Invalid recovery code"}))]
pub struct InvalidRecoveryCode {
    error: String,
}

/// Log in with a recovery code
///
/// **This endpoint can only be used as a second factor.** Each recovery code can be used only one time.
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Success", body = SuccessfulLoginResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = InvalidRecoveryCode, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn login_with_recovery_code(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    session: Session,
    Json(body): Json<RecoveryCodeLoginBody>,
) -> AxumResult<Json<SuccessfulLoginResponse>> {
    let user = get_user_by_id(&state.database, &user_id).await?;

    if user.is_none() || user.clone().unwrap().auth_factors.recovery_codes.is_empty() {
        return Err(AxumError::unauthorized(eyre::eyre!("Invalid 2FA code")));
    }

    let user = user.unwrap();

    let code_hash = verify_recovery_code(body.code, user.auth_factors.recovery_codes)?;

    let update_result = state
        .database
        .collection::<User>("users")
        .update_one(
            doc! {
                "_id": *user_id,
                "auth_factors.recovery_codes": {
                    "$elemMatch": {
                        "code_hash": &code_hash,
                        "used": false
                    }
                }
            },
            doc! {
                "$set": {
                    "auth_factors.recovery_codes.$.used": true
                }
            },
        )
        .await?;

    if update_result.modified_count == 0 {
        return Err(AxumError::unauthorized(eyre::eyre!(
            "Recovery code already used"
        )));
    }

    set_recent_factor(&state.database, &user_id, SecondFactor::RecoveryCode.into()).await?;

    session
        .insert("auth_state", AuthState::Authenticated)
        .await?;

    Ok(Json(SuccessfulLoginResponse {
        two_factor_required: false,
        second_factors: None,
        recent_factor: None,
    }))
}
