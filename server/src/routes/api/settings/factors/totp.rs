// mod disable;
mod enable;

use base32::{Alphabet, decode};
use color_eyre::eyre::{self, Context, ContextCompat};
use serde::Serialize;
use totp_rs::TOTP;
use utoipa::{ToSchema, schema};

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
