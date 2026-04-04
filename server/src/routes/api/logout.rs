use axum::Json;
use serde::Serialize;
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{axum_error::AxumResult, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(logout))
}

#[derive(Serialize, ToSchema)]
struct LogoutResponse {
    success: bool,
}

/// Log out
///
/// Destroys the current session, effectively logging the user out.
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Logged out", body = LogoutResponse, content_type = "application/json"),
    ),
    tag = "Auth"
)]
async fn logout(session: Session) -> AxumResult<Json<LogoutResponse>> {
    session.flush().await?;
    Ok(Json(LogoutResponse { success: true }))
}
