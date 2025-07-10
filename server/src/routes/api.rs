mod health;
mod login;
mod register;

use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, schema};

use super::Route;

pub fn routes() -> Vec<Route> {
    [health::routes(), login::routes(), register::routes()].concat()
}

#[derive(Clone, Deserialize, Serialize)]
pub enum AuthState {
    Anonymous,
    BeforeTwoFactor,
    Authenticated,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true,"id": "60c72b2f9b1d8c001c8e4f5a"}))]
pub struct CreateSuccess {
    success: bool,
    id: String,
}
