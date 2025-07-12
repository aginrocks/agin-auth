// mod disable;
mod enable;

use base32::{Alphabet, decode};
use color_eyre::eyre::{self, Context, ContextCompat};
use serde::{Deserialize, Serialize};
use totp_rs::TOTP;
use utoipa::ToSchema;
use validator::Validate;

use crate::axum_error::{AxumError, AxumResult};

use super::Route;

pub fn routes() -> Vec<Route> {
    [enable::routes()].concat()
}

pub fn create_totp_instance(
    secret: &str,
    email: Option<String>,
    app_name: Option<String>,
) -> AxumResult<TOTP> {
    let decoded_secret = decode(Alphabet::Rfc4648 { padding: false }, secret)
        .wrap_err("Failed to decode 2FA secret")?;

    let totp = TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        decoded_secret,
        app_name,
        email.unwrap_or("User".to_string()),
    )
    .wrap_err("Failed to create TOTP")?;

    Ok(totp)
}

pub fn verify_totp(secret: &str, code: &str) -> AxumResult<()> {
    let totp = create_totp_instance(secret, None, None)?;

    let code_matched = totp
        .check_current(code)
        .wrap_err("Failed to verify TOTP code")?;

    if !code_matched {
        return Err(AxumError::unauthorized(eyre::eyre!("Invalid 2FA code")));
    }

    Ok(())
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Invalid 2FA code"}))]
pub struct Invalid2faCode {
    pub error: String,
}

// TODO: Add proper code validation
#[derive(Deserialize, ToSchema, Validate)]
pub struct TotpCodeBody {
    /// TOTP code to confirm enabling the factor.
    #[validate(length(equal = 6))]
    pub code: String,
}
