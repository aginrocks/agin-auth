use axum::{Extension, Json};
use axum_valid::Valid;
use chrono::{DateTime, Utc};
use color_eyre::eyre::{self};
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::get_user,
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/login/pgp/challenge";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_pgp_challenge, respond_to_pgp_challenge),
        RouteProtectionLevel::Public,
    )]
}

#[derive(Serialize, ToSchema)]
pub struct PgpChallengeResponse {
    challenge: String,
}

#[derive(Serialize, Deserialize)]
pub struct PgpChallengeConfig {
    expires_at: DateTime<Utc>,
    challenge: String,
}

/// Get PGP challenge
///
/// Returns a challenge that needs to be signed with the user's PGP key.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = PgpChallengeResponse, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn get_pgp_challenge(session: Session) -> AxumResult<Json<PgpChallengeResponse>> {
    let challenge = generate_pgp_challenge();
    let expires_at = Utc::now() + chrono::Duration::minutes(5);

    let challenge_config = PgpChallengeConfig {
        expires_at,
        challenge: challenge.clone(),
    };

    session
        .insert("login::pgp_challenge", challenge_config)
        .await?;

    Ok(Json(PgpChallengeResponse { challenge }))
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "error": "Invalid signature" }))]
struct InvalidSignature {
    error: String,
}

#[derive(Deserialize, ToSchema, Validate)]
struct PgpChallengeBody {
    /// Username or email address
    username: String,

    /// Signature of the challenge obtained from `GET /api/login/pgp/challenge`
    #[validate(length(min = 1))]
    signature: String,
}

/// Respond to PGP challenge
///
/// Sign the challenge obtained from `GET /api/login/pgp/challenge` with the user's PGP key and send the signature here to complete the login process.
#[utoipa::path(
    method(post),
    path = PATH,
    request_body = PgpChallengeBody,
    responses(
        (status = OK, description = "Success", body = PgpChallengeResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = InvalidSignature, content_type = "application/json"),
    ),
    tag = "Login"
)]
async fn respond_to_pgp_challenge(
    Extension(state): Extension<AppState>,
    session: Session,
    Valid(Json(body)): Valid<Json<PgpChallengeBody>>,
) -> AxumResult<Json<PgpChallengeResponse>> {
    let challenge_config = session
        .get::<PgpChallengeConfig>("login::pgp_challenge")
        .await?
        .ok_or_else(|| {
            AxumError::bad_request(eyre::eyre!(
                "No challenge found. Please request a new challenge."
            ))
        })?;

    if challenge_config.expires_at < Utc::now() {
        return Err(AxumError::bad_request(eyre::eyre!(
            "Challenge expired. Please request a new challenge."
        )));
    }

    let user = get_user(&state.database, &body.username).await?;

    if user.is_none() || user.clone().unwrap().auth_factors.pgp.is_none() {
        return Err(AxumError::unauthorized(eyre::eyre!("Invalid signature")));
    }

    // let (signature_message,_) = Message::from_string(&body.signature)
    //     .map_err(|_| AxumError::bad_request(eyre::eyre!("Invalid signature format")))?;

    // let user = user.unwrap();

    // let (public_key, _) = SignedPublicKey::from_string(&user.auth_factors.pgp.unwrap().public_key)
    // .wrap_err("Invalid public key")?;

    // let standalone_signature = StandaloneSignature { signature: Signature:: };

    todo!()
}

fn generate_pgp_challenge() -> String {
    let rng = rand::rngs::ThreadRng::default();

    rng.sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}
