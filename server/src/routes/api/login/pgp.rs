use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod challenge;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/challenge", challenge::routes())
}
