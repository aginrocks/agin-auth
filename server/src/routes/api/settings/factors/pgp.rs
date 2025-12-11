use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod enable;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/enable", enable::routes())
}
