use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub mod factors;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/factors", factors::routes())
}
