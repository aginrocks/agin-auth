use std::ops::Deref;

use axum::{extract::Request, middleware::Next, response::Response};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{axum_error::AxumResult, database::User};

/// User data type for request extensions
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct UserData(pub User);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId(pub ObjectId);

impl Deref for UserId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Middleware that ensures the user is authenticated
pub async fn require_auth(
    // State(state): State<AppState>,
    request: Request,
    next: Next,
) -> AxumResult<Response> {
    // TODO: Implement auth

    Ok(next.run(request).await)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}
