use strum::{Display, IntoStaticStr};

#[derive(IntoStaticStr, Copy, Clone, Display)]
pub enum EndpointBasePath {
    #[strum(serialize = "/api/auth/factors")]
    Authentication,

    #[strum(serialize = "/api/settings/factors")]
    Management,
}

/// A definition of a handler to be generated that does not include user-provided args
pub struct HandlerDefinition {
    /// Name of the method to call on the original struct (e.g. `authenticate`)
    pub method: &'static str,

    /// Base path of the endpoint (e.g. `/account/factors`).
    /// Needs to start with a slash and not end with a slash
    pub endpoint_base_path: EndpointBasePath,

    /// Name of the endpoint (e.g. `authenticate`)
    pub endpoint: &'static str,

    /// Name of the request struct on Self (e.g. `AuthenticateRequest`)
    pub request_type: &'static str,

    /// Name of the response struct on Self (e.g. `AuthenticateResponse`)
    pub response_type: &'static str,

    /// Optional full path to a generic type that wraps the response type (e.g. `::auth_core::AuthenticateResponse<T>`)
    pub response_generic: Option<&'static str>,

    /// Full path of the error type (e.g. `::auth_core::FactorError`)
    pub error_type: &'static str,
}

pub static AUTHENTICATE: HandlerDefinition = HandlerDefinition {
    method: "authenticate",
    endpoint_base_path: EndpointBasePath::Authentication,
    endpoint: "authenticate",
    request_type: "AuthenticateRequest",
    response_type: "AuthenticateResponse",
    response_generic: Some("::auth_core::AuthenticateResponse"),
    error_type: "::auth_core::FactorError",
};

pub static ENABLE: HandlerDefinition = HandlerDefinition {
    method: "enable",
    endpoint_base_path: EndpointBasePath::Management,
    endpoint: "enable",
    request_type: "EnableRequest",
    response_type: "EnableResponse",
    response_generic: Some("::auth_core::EnableResponse"),
    error_type: "::auth_core::FactorEnableError",
};

pub static DISABLE: HandlerDefinition = HandlerDefinition {
    method: "disable",
    endpoint_base_path: EndpointBasePath::Management,
    endpoint: "disable",
    request_type: "DisableRequest",
    response_type: "DisableResponse",
    response_generic: None,
    error_type: "::auth_core::FactorDisableError",
};

pub static CONFIRM_ENABLE: HandlerDefinition = HandlerDefinition {
    method: "confirm_enable",
    endpoint_base_path: EndpointBasePath::Management,
    endpoint: "enable/confirm",
    request_type: "ConfirmEnableRequest",
    response_type: "ConfirmEnableResponse",
    response_generic: Some("::auth_core::ConfirmEnableResponse"),
    error_type: "::auth_core::FactorEnableError",
};

pub static CHALLENGE_RESPONSE: HandlerDefinition = HandlerDefinition {
    method: "authenticate_challenge_response",
    endpoint_base_path: EndpointBasePath::Authentication,
    endpoint: "authenticate/challenge-response",
    request_type: "ChallengeResponse",
    response_type: "ChallengeAuthenticationResult",
    response_generic: Some("::auth_core::AuthenticateResponse"),
    error_type: "::auth_core::FactorError",
};

pub static METHODS: &[&HandlerDefinition] = &[
    &AUTHENTICATE,
    &ENABLE,
    &DISABLE,
    &CONFIRM_ENABLE,
    &CHALLENGE_RESPONSE,
];
