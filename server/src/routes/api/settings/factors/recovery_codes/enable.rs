use axum::{Extension, Json};
use color_eyre::eyre::{self};
use mongodb::bson::doc;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{RecoveryCodeFactor, User},
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::{
        RouteProtectionLevel,
        api::settings::factors::recovery_codes::{generate_recovery_codes, hash_recovery_codes},
    },
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/settings/factors/recovery-codes/enable";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(enable_recovery_codes),
        RouteProtectionLevel::Authenticated,
    )]
}

#[derive(Serialize, ToSchema)]
pub struct EnableRecoveryCodesResponse {
    /// Generated security codes. Save them securely as they won't be shown again.
    pub codes: Vec<String>,
}

/// Enable recovery codes
///
/// **Calling this endpoint again will regenerate the recovery codes.** The old codes will be forever lost.
#[utoipa::path(
    method(post),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = EnableRecoveryCodesResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn enable_recovery_codes(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<EnableRecoveryCodesResponse>> {
    let codes = generate_recovery_codes(10, 12);
    let hashed_codes = hash_recovery_codes(codes.clone())?;

    let db_codes = hashed_codes
        .iter()
        .map(|code| RecoveryCodeFactor {
            code_hash: code.clone(),
            used: false,
        })
        .collect::<Vec<_>>();

    let updated = state
        .database
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "_id": *user_id },
            doc! {
                "$set": {
                    "auth_factors.recovery_codes": db_codes,
                }
            },
        )
        .await?;

    if updated.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("User not found")));
    }

    Ok(Json(EnableRecoveryCodesResponse { codes }))
}
