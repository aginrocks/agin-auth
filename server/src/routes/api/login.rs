mod options;
mod password;
mod totp;

use serde::Serialize;
use utoipa::ToSchema;

use crate::database::SecondFactor;

use super::Route;

pub fn routes() -> Vec<Route> {
    [options::routes(), password::routes(), totp::routes()].concat()
}

#[derive(Serialize, ToSchema)]
struct SuccessfulLoginResponse {
    two_factor_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    second_factors: Option<Vec<SecondFactor>>,
}
