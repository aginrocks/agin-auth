use axum::{Extension, Json};
use color_eyre::eyre::OptionExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    database::User,
    middlewares::require_auth::UserId,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_profile))
        .routes(routes!(update_profile))
}

#[derive(Serialize, ToSchema)]
struct ProfileResponse {
    preferred_username: String,
    display_name: String,
    email: String,
    email_confirmed: bool,
    first_name: String,
    last_name: String,
}

/// Get user profile
///
/// Returns the current user's profile information.
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Profile data", body = ProfileResponse, content_type = "application/json"),
    ),
    tag = "Settings"
)]
async fn get_profile(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<ProfileResponse>> {
    let user = state
        .database
        .collection::<User>("users")
        .find_one(doc! { "_id": *user_id })
        .await?
        .ok_or_eyre("User not found")?;

    Ok(Json(ProfileResponse {
        preferred_username: user.preferred_username,
        display_name: user.display_name,
        email: user.email,
        email_confirmed: user.email_confirmed,
        first_name: user.first_name,
        last_name: user.last_name,
    }))
}

#[derive(Deserialize, ToSchema)]
struct UpdateProfileBody {
    display_name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Serialize, ToSchema)]
struct UpdateProfileResponse {
    success: bool,
}

/// Update user profile
///
/// Updates the current user's profile. Only provided fields will be updated.
#[utoipa::path(
    method(patch),
    path = "/",
    request_body = UpdateProfileBody,
    responses(
        (status = OK, description = "Profile updated", body = UpdateProfileResponse, content_type = "application/json"),
        (status = BAD_REQUEST, description = "No fields to update"),
    ),
    tag = "Settings"
)]
async fn update_profile(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(body): Json<UpdateProfileBody>,
) -> AxumResult<Json<UpdateProfileResponse>> {
    let mut update = doc! {};

    if let Some(display_name) = &body.display_name {
        let trimmed = display_name.trim();
        if !trimmed.is_empty() && trimmed.len() <= 64 {
            update.insert("display_name", trimmed);
        }
    }
    if let Some(first_name) = &body.first_name {
        let trimmed = first_name.trim();
        if trimmed.len() <= 64 {
            update.insert("first_name", trimmed);
        }
    }
    if let Some(last_name) = &body.last_name {
        let trimmed = last_name.trim();
        if trimmed.len() <= 64 {
            update.insert("last_name", trimmed);
        }
    }

    if update.is_empty() {
        return Ok(Json(UpdateProfileResponse { success: true }));
    }

    state
        .database
        .collection::<User>("users")
        .update_one(doc! { "_id": *user_id }, doc! { "$set": update })
        .await?;

    Ok(Json(UpdateProfileResponse { success: true }))
}
