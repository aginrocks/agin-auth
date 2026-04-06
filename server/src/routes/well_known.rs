use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{oidc::build_provider_metadata, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(openid_configuration))
}

#[utoipa::path(
    method(get),
    path = "/openid-configuration",
    responses(
        (status = OK, description = "OpenID Connect Discovery document"),
        (status = INTERNAL_SERVER_ERROR, description = "Failed to build discovery document")
    ),
    tag = "OpenID Connect"
)]
async fn openid_configuration(Extension(state): Extension<AppState>) -> impl IntoResponse {
    let issuer = state.settings.general.public_url.to_string();
    let issuer = issuer.trim_end_matches('/');
    match build_provider_metadata(issuer) {
        Ok(metadata) => Json(metadata).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to build discovery document",
        )
            .into_response(),
    }
}
