use macros::register_factors;
use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub mod password;

register_factors! {
    "password" => password::PasswordFactor,
}
