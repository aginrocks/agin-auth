use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Extension, Json};
use axum_valid::Valid;
use color_eyre::eyre::{self, ContextCompat};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{AuthFactors, PartialUser, PasswordFactor, User},
    routes::api::CreateSuccess,
    state::AppState,
    validators::username_validator,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(register))
}

#[derive(Deserialize, ToSchema, Validate)]
struct RegisterBody {
    #[validate(length(min = 1, max = 32))]
    first_name: String,

    #[validate(length(min = 1, max = 32))]
    last_name: String,

    #[validate(length(min = 1, max = 32))]
    display_name: String,

    #[validate(custom(function = "username_validator"), length(min = 1, max = 32))]
    preferred_username: String,

    #[validate(email, length(max = 128))]
    email: String,

    #[validate(length(min = 1))]
    password: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "User with this username or email already exists"}))]
pub struct BadRequestError {
    error: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true}))]
pub struct RegisterSuccess {
    success: bool,
}

/// Register
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = BAD_REQUEST, description = "BadRequest", body = BadRequestError, content_type = "application/json"),
    ),
    tag = "Register"
)]
async fn register(
    Extension(state): Extension<AppState>,
    Valid(Json(body)): Valid<Json<RegisterBody>>,
) -> AxumResult<Json<CreateSuccess>> {
    let already_exists = state
        .database
        .collection::<User>("users")
        .find_one(doc! {
            "$or": [
                { "preferred_username": &body.preferred_username },
                { "email": &body.email }
            ]
        })
        .await?;

    if already_exists.is_some() {
        return Err(AxumError::bad_request(eyre::eyre!(
            "User with this username or email already exists"
        )));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_password = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|_| eyre::eyre!("Failed to compute hash"))?;

    let uuid = Uuid::new_v4();

    let user = PartialUser {
        uuid,
        first_name: body.first_name,
        last_name: body.last_name,
        display_name: body.display_name,
        preferred_username: body.preferred_username,
        email: body.email,
        auth_factors: AuthFactors {
            password: PasswordFactor {
                password_hash: Some(hashed_password.to_string()),
            },
            ..Default::default()
        },
        groups: vec![],
    };

    let inserted = state
        .database
        .collection::<PartialUser>("users")
        .insert_one(user)
        .await?;

    let id = inserted
        .inserted_id
        .as_object_id()
        .wrap_err("Failed to fetch project ID")?
        .to_string();

    Ok(Json(CreateSuccess { success: true, id }))
}
