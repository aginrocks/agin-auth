use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub mod applications;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/applications", applications::routes())
}

// TODO: Add proper auth
