use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub mod password;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().merge(password::routes())
}
