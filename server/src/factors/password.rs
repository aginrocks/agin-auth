use async_trait::async_trait;
use auth_core::{
    AuthenticateResponse, EnableResponse, Factor, FactorDisableError, FactorEnableError,
    FactorError, FactorMetadata, FactorRole, FlowType, NoData, SecurityLevel,
};
use macros::factor;
use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub struct PasswordFactor;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().merge(factor())
}

impl FactorMetadata for PasswordFactor {
    const FLOW_TYPE: FlowType = FlowType::Simple;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::Knowledge;
    const ROLE: FactorRole = FactorRole::Primary;
}

#[async_trait]
#[factor(slug = "password")]
impl Factor for PasswordFactor {
    type Config = NoData;

    type FactorState = NoData;

    type EnableRequest = NoData;
    type EnableResponse = NoData;

    // Enable Docs here
    async fn enable(
        &self,
        args: Self::EnableRequest,
    ) -> Result<EnableResponse<Self::EnableResponse>, FactorEnableError> {
        Err(FactorEnableError::Other(FactorError::Other(
            color_eyre::eyre::eyre!("Not implemented"),
        )))
    }

    type DisableRequest = NoData;
    type DisableResponse = NoData;

    // Disable Docs here
    async fn disable(
        &self,
        args: Self::DisableRequest,
    ) -> Result<Self::DisableResponse, FactorDisableError> {
        Err(FactorDisableError::Other(FactorError::Other(
            color_eyre::eyre::eyre!("Not implemented"),
        )))
    }

    type AuthenticateRequest = NoData;
    type AuthenticateResponse = NoData;

    /// Authenticate Docs here
    async fn authenticate(
        &self,
        args: Self::AuthenticateRequest,
    ) -> Result<AuthenticateResponse<Self::AuthenticateResponse>, FactorError> {
        Err(FactorError::Other(color_eyre::eyre::eyre!(
            "Not implemented"
        )))
    }
}
