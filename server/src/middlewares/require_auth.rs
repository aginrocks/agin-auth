use std::ops::Deref;

use axum::{extract::Request, middleware::Next, response::Response};
use color_eyre::{eyre, eyre::Result};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;

use crate::{
    axum_error::{AxumError, AxumResult},
    routes::api::AuthState,
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

    let auth_state = session.get::<AuthState>("user_id").await?;
    if auth_state.is_none() || user_id.is_none() {
        return Err(eyre::eyre!("Unauthorized"));
    }

    let user_id = user_id.unwrap();
    let auth_state = auth_state.unwrap();

    Ok((user_id, auth_state))
}

/// Middleware that ensures the user is authenticated
pub async fn require_auth(
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

    request.extensions_mut().insert(UserId(user_id));

    Ok(next.run(request).await)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}
