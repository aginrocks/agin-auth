use std::ops::Deref;

use axum::{Extension, extract::Request, http::header, middleware::Next, response::Response};
use color_eyre::{eyre, eyre::Result};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::record_session,
    routes::api::AuthState,
    state::AppState,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId(pub ObjectId);

impl Deref for UserId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

async fn get_auth_state(session: &Session) -> Result<(ObjectId, AuthState)> {
    let user_id = session.get::<ObjectId>("user_id").await?;
    let auth_state = session.get::<AuthState>("auth_state").await?;
    if auth_state.is_none() || user_id.is_none() {
        return Err(eyre::eyre!("Unauthorized"));
    }

    let user_id = user_id.unwrap();
    let auth_state = auth_state.unwrap();

    Ok((user_id, auth_state))
}

fn extract_client_ip(request: &Request, trust_proxy: bool) -> String {
    if trust_proxy {
        return request
            .headers()
            .get("x-forwarded-for")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(',').next_back())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".to_string());
    }

    request
        .extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|info| info.0.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Middleware that ensures the user is authenticated
pub async fn require_auth(
    Extension(state): Extension<AppState>,
    session: Session,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    let (user_id, auth_state) = get_auth_state(&session)
        .await
        .map_err(|_| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

    if auth_state != AuthState::Authenticated {
        return Err(AxumError::unauthorized(eyre::eyre!("Unauthorized")));
    }

    // Record/update session in MongoDB
    if let Some(session_id) = session.id() {
        let ip = extract_client_ip(&request, state.settings.general.trust_proxy);
        let user_agent = request
            .headers()
            .get(header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let db = state.database.clone();
        let sid = session_id.to_string();
        let uid = user_id;
        if let Err(e) = record_session(&db, &sid, &uid, &ip, &user_agent).await {
            tracing::warn!(error = ?e, "Failed to record session");
        }
    }

    request.extensions_mut().insert(UserId(user_id));

    Ok(next.run(request).await)
}

/// Middleware that ensures the user has completed the first factor of authentication
pub async fn require_first_factor(
    session: Session,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    let (user_id, auth_state) = get_auth_state(&session)
        .await
        .map_err(|_| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

    if auth_state == AuthState::Anonymous {
        return Err(AxumError::unauthorized(eyre::eyre!("Unauthorized")));
    }

    if auth_state == AuthState::Authenticated {
        return Err(AxumError::forbidden(eyre::eyre!("Already logged in")));
    }

    request.extensions_mut().insert(UserId(user_id));

    Ok(next.run(request).await)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Forbidden"}))]
pub struct ForbiddenError {
    error: String,
}
