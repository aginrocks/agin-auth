pub mod api;

use utoipa_axum::router::UtoipaMethodRouter;

use crate::state::AppState;

pub fn routes() -> Vec<Route> {
    [api::routes()].concat()
}

#[derive(Clone)]
pub enum RouteProtectionLevel {
    Public,
    BeforeTwoFactor,
    Authenticated,
    Admin,
}

type Route = (UtoipaMethodRouter<AppState>, RouteProtectionLevel);
