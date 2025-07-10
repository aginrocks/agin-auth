use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::Json;
use base64::{Engine, engine::general_purpose};
use color_eyre::eyre;
use color_eyre::eyre::Context;
use mongodb::bson::doc;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{FirstFactor, SecondFactor, get_second_factors},
    extractors::unauthenticated_user::UnauthenticatedUser,
    routes::RouteProtectionLevel,
};

use super::Route;

const PATH: &str = "/api/login/password";

pub fn routes() -> Vec<Route> {
    vec![(routes!(login_with_password), RouteProtectionLevel::Public)]
}

#[derive(Deserialize, ToSchema)]
struct LoginBody {
    username: String,
    password: String,
}

#[derive(Serialize, ToSchema)]
#[serde(tag = "two_factor_required")]
enum LoginResponse {
    False,
    True { second_factors: Vec<SecondFactor> },
}

/// Get login options
///
/// Gets available login options for the user. If the user is not found, returns only password option.
#[utoipa::path(
    method(post),
    path = PATH,
    params(
        ("username" = String, Query, description = "Username or email address of the user the factors are requested for"),
    ),
    responses(
        (status = OK, description = "Success", body = LoginResponse)
    ),
    tag = "Login"
)]
async fn login_with_password(
    UnauthenticatedUser(user): UnauthenticatedUser,
    Json(body): Json<LoginBody>,
) -> AxumResult<Json<LoginResponse>> {
    // Hashing the password in order to prevent timing attacks
    if user.is_none() || user.clone().unwrap().password_hash.is_none() {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(body.password.as_bytes(), &salt)
            .map_err(|_| eyre::eyre!("Failed to compute hash"))?;

        return Err(AxumError::unauthorized(eyre::eyre!(
            "Invalid username or password"
        )));
    }

    let user = user.unwrap();

    let password_hash = &user.clone().password_hash.unwrap();

    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|_| eyre::eyre!("Failed to compute hash"))?;
    let argon2 = Argon2::default();

    argon2
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| AxumError::unauthorized(eyre::eyre!("Invalid username or password")))?;

    let second_factors = get_second_factors(&user);

    if second_factors.is_empty() {
        return Ok(Json(LoginResponse::False));
    }

    Ok(Json(LoginResponse::True { second_factors }))
}
