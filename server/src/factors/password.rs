use async_trait::async_trait;
use auth_core::{
    EnableResponse, Factor, FactorDisableError, FactorEnableError, FactorError, FactorRole,
    FlowType, NoData, SecurityLevel,
};

pub struct PasswordFactor;

#[async_trait]
impl Factor for PasswordFactor {
    const FLOW_TYPE: FlowType = FlowType::Simple;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::Knowledge;
    const ROLE: FactorRole = FactorRole::Primary;

    type EnableRequest = NoData;
    type EnableResponse = NoData;

    async fn enable(
        &self,
        args: Self::EnableRequest,
    ) -> Result<EnableResponse<Self::EnableResponse>, FactorEnableError> {
        todo!()
    }

    type DisableRequest = NoData;
    type DisableResponse = NoData;

    async fn disable(&self, args: Self::DisableRequest) -> Result<(), FactorDisableError> {
        todo!()
    }

    type AuthenticateRequest = NoData;
    type AuthenticateResponse = NoData;

    async fn authenticate(&self, args: Self::AuthenticateRequest) -> Result<(), FactorError> {
        todo!()
    }
}
