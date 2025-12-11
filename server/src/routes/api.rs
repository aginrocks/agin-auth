mod admin;
mod health;
mod login;
mod register;
mod settings;

use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::{ToSchema, schema};
use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new()
        .nest("/admin", admin::routes())
        .nest("/settings", settings::routes());

    let public = OpenApiRouter::new()
        .nest("/health", health::routes())
        .nest("/login", login::routes())
        .nest("/register", register::routes());

    auth.merge(public)
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
