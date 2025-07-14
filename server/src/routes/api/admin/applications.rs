use axum::{Extension, Json};
use futures::TryStreamExt;
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    database::{Application, PublicApplication},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/admin/applications";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_applications), RouteProtectionLevel::Public)]
}

/// Get applications
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Vec<PublicApplication>, content_type = "application/json")
    ),
    tag = "Admin"
)]
async fn get_applications(
    Extension(state): Extension<AppState>,
) -> AxumResult<Json<Vec<PublicApplication>>> {
    let cursor = state
        .database
        .collection::<Application>("applications")
        .find(doc! {})
        .await?;

    let applications: Vec<Application> = cursor.try_collect().await?;

    let public_applications = applications
        .iter()
        .map(|a| a.to_public())
        .collect::<Vec<_>>();

    Ok(Json(public_applications))
}
