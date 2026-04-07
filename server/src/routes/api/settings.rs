use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub mod account;
pub mod factors;
pub mod password;
pub mod profile;
pub mod sessions;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/account", account::routes())
        .nest("/factors", factors::routes())
        .nest("/password", password::routes())
        .nest("/profile", profile::routes())
        .nest("/sessions", sessions::routes())
}
