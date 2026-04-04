use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod delete;
mod finish;
mod start;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/start", start::routes())
        .nest("/finish", finish::routes())
        .nest("/delete", delete::routes())
}
