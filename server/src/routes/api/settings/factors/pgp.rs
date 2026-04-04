use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod disable;
mod enable;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/enable", enable::routes())
        .nest("/disable", disable::routes())
}
