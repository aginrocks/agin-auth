pub mod api;

use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::{ApiDoc, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/api", api::routes())
}
