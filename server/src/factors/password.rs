use async_trait::async_trait;
use auth_core::{
    AuthenticateResponse, EnableResponse, Factor, FactorDisableError, FactorEnableError,
    FactorError, FactorRole, FlowType, NoData, SecurityLevel,
};
use macros::factor;

pub struct PasswordFactor;

#[async_trait]
#[factor(slug = "password")]
impl Factor for PasswordFactor {
    const FLOW_TYPE: FlowType = FlowType::Simple;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::Knowledge;
    const ROLE: FactorRole = FactorRole::Primary;

    type EnableRequest = NoData;
    type EnableResponse = NoData;

    // Enable Docs here
    async fn enable(
        &self,
        args: Self::EnableRequest,
    ) -> Result<EnableResponse<Self::EnableResponse>, FactorEnableError> {
        todo!()
    }

    type DisableRequest = NoData;
    type DisableResponse = NoData;

    // Disable Docs here
    async fn disable(
        &self,
        args: Self::DisableRequest,
    ) -> Result<Self::DisableResponse, FactorDisableError> {
        todo!()
    }

    type AuthenticateRequest = NoData;
    type AuthenticateResponse = NoData;

    /// Authenticate Docs here
    async fn authenticate(
        &self,
        args: Self::AuthenticateRequest,
    ) -> Result<AuthenticateResponse<Self::AuthenticateResponse>, FactorError> {
        todo!()
    }
}
