use axum::{Extension, Json, extract::Query};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    database::{FirstFactor, get_user},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/login/options";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_login_options), RouteProtectionLevel::Public)]
}

#[derive(Serialize, ToSchema)]
struct OptionsRepsonse {
    options: Vec<FirstFactor>,
}

#[derive(Deserialize, ToSchema)]
struct OptionsQuery {
    username: String,
}

/// Get login options
///
/// Gets available login options for the user. If the user is not found, returns only password option.
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("username" = String, Query, description = "Username or email address of the user the factors are requested for"),
    ),
    responses(
        (status = OK, description = "Success", body = OptionsRepsonse, content_type = "application/json")
    ),
    tag = "Login"
)]
async fn get_login_options(
    Extension(state): Extension<AppState>,
    Query(OptionsQuery { username }): Query<OptionsQuery>,
) -> AxumResult<Json<OptionsRepsonse>> {
    let user = get_user(&state.database, &username).await?;

    if user.is_none() {
        return Ok(Json(OptionsRepsonse {
            options: vec![
                FirstFactor::Password,
                FirstFactor::WebAuthn,
                FirstFactor::Gpg,
            ],
        }));
    }

    let user = user.unwrap();

    let mut options: Vec<FirstFactor> = vec![];

    if user.auth_factors.password.password_hash.is_some() {
        options.push(FirstFactor::Password);
    }

    if !user.auth_factors.webauthn.is_empty() {
        options.push(FirstFactor::WebAuthn);
    }

    if !user.auth_factors.gpg.is_empty() {
        options.push(FirstFactor::Gpg);
    }

    Ok(Json(OptionsRepsonse { options }))
}
