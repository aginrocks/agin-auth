use async_trait::async_trait;
use color_eyre::eyre::Error;
use thiserror::Error;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Error)]
pub enum FactorError {
    #[error(transparent)]
    Unauthorized(Error),

    #[error(transparent)]
    Other(#[from] Error),
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

    type EnableArgs: Send + Sync + ToSchema + Validate;
    type DisableArgs: Send + Sync + ToSchema + Validate;
    type AuthenticateArgs: Send + Sync + ToSchema + Validate;

    async fn enable(&self, args: Self::EnableArgs) -> Result<(), FactorError>;
    async fn disable(&self, args: Self::DisableArgs) -> Result<(), FactorError>;
    async fn authenticate(&self, args: Self::AuthenticateArgs) -> Result<(), FactorError>;
}
