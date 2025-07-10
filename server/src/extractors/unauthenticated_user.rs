use std::ops::Deref;

use axum::extract::{FromRequestParts, Query};
use mongodb::{Database, bson::doc};
use serde::Deserialize;

use crate::{axum_error::AxumError, database::User, state::AppState};

pub struct UnauthenticatedUser(pub Option<User>);

impl Deref for UnauthenticatedUser {
    type Target = Option<User>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct QueryParams {
    username: String,
}

impl FromRequestParts<AppState> for UnauthenticatedUser {
    type Rejection = AxumError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Handle body here

        let Query(QueryParams { username }) = Query::from_request_parts(parts, state).await?;

        let user = state
            .database
            .collection::<User>("users")
            .find_one(doc! {
                "$or": [
                    { "username": &username },
                    { "email": &username }
                ]
            })
            .await?;

        Ok(UnauthenticatedUser(user))
    }
}
