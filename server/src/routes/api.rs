mod admin;
mod confirm_email;
mod health;
mod login;
mod logout;
mod password_reset;
mod register;
mod settings;

use axum::middleware;
use serde::{Deserialize, Serialize};
use strum::Display;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

use crate::{middlewares::require_auth::require_auth, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new()
        .nest("/admin", admin::routes())
        .nest("/logout", logout::routes())
        .nest("/settings", settings::routes())
        .layer(middleware::from_fn(require_auth));

    // Global rate limit: 5 burst, 1 replenish per 2s per IP
    let rate_limit_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let public = OpenApiRouter::new()
        .nest("/confirm-email", confirm_email::routes())
        .nest("/health", health::routes())
        .nest("/login", login::routes())
        .nest("/password-reset", password_reset::routes())
        .nest("/register", register::routes());

    auth.merge(public)
        .layer(GovernorLayer::new(rate_limit_conf))
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
