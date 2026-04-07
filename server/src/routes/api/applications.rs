use axum::{Extension, Json};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    database::{Application, User},
    middlewares::require_auth::{ForbiddenError, UnauthorizedError, UserId},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_my_applications))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserApplication {
    pub name: String,
    pub slug: String,
    pub icon: Option<String>,
}

/// Get applications available to the current user
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = Vec<UserApplication>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json"),
    ),
    tag = "Applications"
)]
async fn get_my_applications(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<Vec<UserApplication>>> {
    let user = state
        .database
        .collection::<User>("users")
        .find_one(doc! { "_id": *user_id })
        .await?;

    let user_groups: Vec<ObjectId> = user.map(|u| u.groups).unwrap_or_default();

    let apps: Vec<Application> = state
        .database
        .collection::<Application>("applications")
        .find(doc! {})
        .await?
        .try_collect()
        .await?;

    let visible_apps = apps
        .into_iter()
        .filter(|app| {
            app.allowed_groups.is_empty()
                || app.allowed_groups.iter().any(|g| user_groups.contains(g))
        })
        .map(|app| UserApplication {
            name: app.name,
            slug: app.slug,
            icon: app.icon,
        })
        .collect();

    Ok(Json(visible_apps))
}
