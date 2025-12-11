mod options;
mod password;
mod pgp;
mod recovery_codes;
mod totp;
mod webauthn;

use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

use crate::{database::SecondFactor, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/options", options::routes())
        .nest("/password", password::routes())
        .nest("/totp", totp::routes())
        .nest("/recovery-codes", recovery_codes::routes())
        .nest("/pgp", pgp::routes())
        .nest("/webauthn", webauthn::routes())
}

#[derive(Serialize, ToSchema)]
struct SuccessfulLoginResponse {
    two_factor_required: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    second_factors: Option<Vec<SecondFactor>>,

    /// Recently used factor
    #[serde(skip_serializing_if = "Option::is_none")]
    recent_factor: Option<SecondFactor>,
}
