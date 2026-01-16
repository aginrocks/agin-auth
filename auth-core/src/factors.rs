use async_trait::async_trait;
use color_eyre::eyre::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::{ToSchema, openapi::schema};

// TODO: Fix error types

#[derive(Debug, Error, ToSchema)]
#[schema(as = String)]
pub enum FactorError {
    #[error("Factor is not enabled")]
    NotEnabled,

    #[error(transparent)]
    #[schema(value_type = String)]
    Unauthorized(Error),

    #[error(transparent)]
    #[schema(value_type = String)]
    BadRequest(Error),

    #[error(transparent)]
    #[schema(value_type = String)]
    Other(#[from] Error),
}

#[derive(Debug, Error, ToSchema)]
pub enum FactorEnableError {
    #[error("Factor is already enabled")]
    AlreadyEnabled,

    #[error(transparent)]
    Other(#[from] FactorError),
}

#[derive(Debug, Error, ToSchema)]
pub enum FactorDisableError {
    #[error("Factor is not enabled")]
    NotEnabled,

    #[error("Cannot disable the only primary factor")]
    CannotDisableOnlyPrimary,

    #[error(transparent)]
    Other(#[from] FactorError),
}

/// Defines the type of authentication flow.
///
/// - `Simple`: A straightforward flow where the user provides credentials and gets authenticated in a single step.
/// - `RoundTrip`: A multi-step flow that involves providing a challenge by the `Factor` and receiving a response from the user to complete authentication.
///
/// For example, TOTP typically uses a `Simple` flow, while WebAuthn often employs a `RoundTrip` flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowType {
    /// A straightforward flow where the user provides credentials and gets authenticated in a single step.
    Simple,
    /// A multi-step flow that involves providing a challenge by the `Factor` and receiving a response from the user to complete authentication.
    RoundTrip,
}

/// Defines the security level provided by an authentication factor.
/// Higher levels indicate stronger security guarantees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityLevel {
    /// A password or similar knowledge-based factor.
    /// Vulnerable to phishing, guessing, and social engineering.
    Knowledge = 0,

    /// A factor that relies on an external channel, such as SMS or email codes.
    /// Vulnerable to interception and SIM swapping attacks.
    OutOfBand = 1,

    /// A possession-based software-backed factor, such as TOTP.
    /// These factors can be duplicated or cloned but rely on cryptography to provide security.
    Possession = 2,

    /// A possession-based hardware-backed factor, such as hardware security keys.
    /// These factors are generally more secure than software-backed factors
    /// and cannot be easily cloned.
    Hardware = 3,
}

/// Defines if the facotr is sufficient alone or requires other factors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactorRole {
    /// Can be used alone as the first/primary authentication factor
    /// Examples: Password, Passkey, QR Login
    /// Can bypass MFA if policy allows it
    Primary,

    /// Can only be used as an additional factor, never alone
    /// Examples: TOTP, SMS codes, recovery codes
    MultiFactorOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EnableResponse<T> {
    /// Indicates whether fully enabling the factor requires a call to `confirm_enable`.
    pub requires_confirmation: bool,

    /// Indicates whether the factor is now enabled after this call.
    pub enabled: bool,

    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfirmEnableResponse<T> {
    /// Indicates whether the factor is now enabled after this call.
    pub enabled: bool,

    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthenticateResponse<T> {
    /// Indicates whether additional factors are required to complete authentication.
    pub fully_authenticated: bool,

    // TODO: Add a stricter type
    /// A list of factors to choose frum for the next authentication step.
    pub next: Vec<String>,

    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NoData;

/// Defines the interface for an authentication factor.
///
/// In Agin Auth a `Factor` represents any method of user authentication, such as TOTP, WebAuthn, Passwords, etc.
/// Each factor can be enabled or disabled by the user, and multiple facotrs can (and should be) stacked together to provide multi-factor authentication (MFA).
/// Factors define a trust level that can be then matched by `Policy` to enforce security requirements.
#[async_trait]
pub trait Factor {
    /// Defines the authentication flow type of the factor.
    const FLOW_TYPE: FlowType;

    /// Defines how much security the factor provides.
    const SECURITY_LEVEL: SecurityLevel;

    /// Defines if the factor is sufficient alone or requires other factors.
    const ROLE: FactorRole;

    fn flow_type(&self) -> FlowType {
        Self::FLOW_TYPE
    }

    fn security_level(&self) -> SecurityLevel {
        Self::SECURITY_LEVEL
    }

    fn role(&self) -> FactorRole {
        Self::ROLE
    }

    type EnableRequest: Send + Sync + ToSchema;
    type EnableResponse: Send + Sync + ToSchema;

    /// Enable the specified factor (or start the enabling process)
    async fn enable(
        &self,
        args: Self::EnableRequest,
    ) -> Result<EnableResponse<Self::EnableResponse>, FactorEnableError>;

    type DisableRequest: Send + Sync + ToSchema;
    type DisableResponse: Send + Sync + ToSchema;

    /// Disable the specified factor
    async fn disable(
        &self,
        args: Self::DisableRequest,
    ) -> Result<Self::DisableResponse, FactorDisableError>;

    type AuthenticateRequest: Send + Sync + ToSchema;
    type AuthenticateResponse: Send + Sync + ToSchema;

    /// Authenticate using the specified factor (or request a challenge)
    async fn authenticate(
        &self,
        args: Self::AuthenticateRequest,
    ) -> Result<AuthenticateResponse<Self::AuthenticateResponse>, FactorError>;
}

#[async_trait]
pub trait FactorConfirmable: Factor {
    type ConfirmEnableRequest: Send + Sync + ToSchema;
    type ConfirmEnableResponse: Send + Sync + ToSchema;

    /// Optionally confirm the enabling if confirmation was requested from `enable`
    async fn confirm_enable(
        &self,
        args: Self::ConfirmEnableRequest,
    ) -> Result<ConfirmEnableResponse<Self::ConfirmEnableResponse>, FactorEnableError>;
}

#[async_trait]
pub trait FactorChallenge: Factor {
    type ChallengeResponse: Send + Sync + ToSchema;
    type ChallengeAuthenticationResult: Send + Sync + ToSchema;

    /// Respond to a challenge generated by `authenticate`
    async fn authenticate_challenge_response(
        &self,
        response: Self::ChallengeResponse,
    ) -> Result<AuthenticateResponse<Self::ChallengeAuthenticationResult>, FactorError>;
}
