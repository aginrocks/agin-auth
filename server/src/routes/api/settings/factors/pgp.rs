use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod delete;
mod disable;
mod enable;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/enable", enable::routes())
        .nest("/disable", disable::routes())
        .nest("/delete", delete::routes())
}
