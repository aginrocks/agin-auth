use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod discoverable_finish;
mod discoverable_start;
mod finish;
mod start;

pub fn two_factor_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/start", start::routes())
        .nest("/finish", finish::routes())
}

pub fn passwordless_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/start", discoverable_start::routes())
        .nest("/finish", discoverable_finish::routes())
}
