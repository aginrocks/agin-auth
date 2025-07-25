mod admin;
mod health;
mod login;
mod register;
mod settings;

use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::{ToSchema, schema};

use super::Route;

pub fn routes() -> Vec<Route> {
    [
        health::routes(),
        login::routes(),
        register::routes(),
        settings::routes(),
        admin::routes(),
    ]
    .concat()
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq, Debug, Display)]
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
