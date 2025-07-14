use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Extension, Json};
use color_eyre::eyre;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{FirstFactor, get_second_factors, get_user, set_recent_factor},
    routes::{RouteProtectionLevel, api::AuthState},
    state::AppState,
};

use super::{Route, SuccessfulLoginResponse};

const PATH: &str = "/api/login/password";

pub fn routes() -> Vec<Route> {
    vec![(routes!(login_with_password), RouteProtectionLevel::Public)]
}

#[derive(Deserialize, ToSchema)]
struct LoginBody {
    /// Username or email address
    username: String,
    password: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Invalid username or password"}))]
pub struct InvalidUserOrPass {
    error: String,
}

/// Log in with password
///
/// If user is not found or the password isn't enabled for the user returns the same response as if the password was incorrect.
#[utoipa::path(
    method(post),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = SuccessfulLoginResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = InvalidUserOrPass, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn login_with_password(
    Extension(state): Extension<AppState>,
    session: Session,
    Json(body): Json<LoginBody>,
) -> AxumResult<Json<SuccessfulLoginResponse>> {
    let user = get_user(&state.database, &body.username).await?;

    // Hashing the password in order to prevent timing attacks
    if user.is_none()
        || user
            .clone()
            .unwrap()
            .auth_factors
            .password
            .password_hash
            .is_none()
    {
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

    let password_hash = &user.clone().auth_factors.password.password_hash.unwrap();

    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|_| eyre::eyre!("Failed to compute hash"))?;
    let argon2 = Argon2::default();

    argon2
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| AxumError::unauthorized(eyre::eyre!("Invalid username or password")))?;

    session.insert("user_id", user.id).await?;

    let second_factors = get_second_factors(&user);

    if second_factors.is_empty() {
        session
            .insert("auth_state", AuthState::Authenticated)
            .await?;

        return Ok(Json(SuccessfulLoginResponse {
            two_factor_required: false,
            second_factors: None,
            recent_factor: None,
        }));
    }

    set_recent_factor(&state.database, &user.id, FirstFactor::Password.into()).await?;

    session
        .insert("auth_state", AuthState::BeforeTwoFactor)
        .await?;

    Ok(Json(SuccessfulLoginResponse {
        two_factor_required: true,
        second_factors: Some(second_factors),
        recent_factor: user.auth_factors.recent.second_factor,
    }))
}
