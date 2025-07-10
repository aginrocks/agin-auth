use axum::Json;
use mongodb::bson::doc;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult, database::FirstFactor,
    extractors::unauthenticated_user::UnauthenticatedUser, routes::RouteProtectionLevel,
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
        (status = OK, description = "Success", body = OptionsRepsonse)
    ),
    tag = "Login"
)]
async fn get_login_options(
    UnauthenticatedUser(user): UnauthenticatedUser,
) -> AxumResult<Json<OptionsRepsonse>> {
    if user.is_none() {
        return Ok(Json(OptionsRepsonse {
            options: vec![FirstFactor::Password],
        }));
    }

    let user = user.unwrap();

    let mut options: Vec<FirstFactor> = vec![];

    if user.password_hash.is_some() {
        options.push(FirstFactor::Password);
    }

    if !user.two_factor.webauthn.is_empty() {
        options.push(FirstFactor::WebAuthn);
    }

    if !user.two_factor.gpg.is_empty() {
        options.push(FirstFactor::Gpg);
    }

    Ok(Json(OptionsRepsonse { options }))
}
