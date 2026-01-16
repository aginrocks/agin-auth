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

pub static METHODS: &[&HandlerDefinition] = &[&AUTHENTICATE];
