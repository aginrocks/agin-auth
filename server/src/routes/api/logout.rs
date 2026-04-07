use axum::{Extension, Json};
use mongodb::bson::doc;
use serde::Serialize;
use tower_sessions::Session;
use tracing::warn;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{axum_error::AxumResult, database::SessionRecord, state::AppState};

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
async fn logout(
    Extension(state): Extension<AppState>,
    session: Session,
) -> AxumResult<Json<LogoutResponse>> {
    let session_id = session.id().map(|id| id.to_string());

    session.flush().await?;

    if let Some(session_id) = session_id
        && let Err(error) = state
            .database
            .collection::<SessionRecord>("sessions")
            .delete_one(doc! { "_id": session_id })
            .await
    {
        warn!(error = ?error, "Failed to clean up session record after logout");
    }

    Ok(Json(LogoutResponse { success: true }))
}
