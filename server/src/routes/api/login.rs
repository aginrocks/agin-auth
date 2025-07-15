mod options;
mod password;
mod pgp;
mod recovery_codes;
mod totp;
mod webauthn;

use serde::Serialize;
use utoipa::ToSchema;

use crate::database::SecondFactor;

use super::Route;

pub fn routes() -> Vec<Route> {
    [
        options::routes(),
        password::routes(),
        totp::routes(),
        recovery_codes::routes(),
        pgp::routes(),
        webauthn::routes(),
    ]
    .concat()
}

#[derive(Serialize, ToSchema)]
struct SuccessfulLoginResponse {
    two_factor_required: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    second_factors: Option<Vec<SecondFactor>>,

    /// Recently used factor
    #[serde(skip_serializing_if = "Option::is_none")]
    recent_factor: Option<SecondFactor>,
}
