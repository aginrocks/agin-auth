use strum::IntoStaticStr;

#[derive(IntoStaticStr)]
pub enum EndpointBasePath {
    #[strum(serialize = "/auth/factors")]
    Authentication,

    #[strum(serialize = "/account/factors")]
    Management,
}

/// A definition of a handler to be generated that does not include user-provided args
pub struct HandlerDefinition {
    /// Name of the method to call on the original struct (e.g. `authenticate`)
    pub method: &'static str,

    /// Base path of the endpoint (e.g. `/account/factors`)
    pub endpoint_base_path: EndpointBasePath,

    /// Name of the endpoint (e.g. `authenticate`)
    pub endpoint: &'static str,

    /// Name of the request struct (e.g. `AuthenticateRequest`)
    pub request_type: &'static str,

    /// Name of the response struct (e.g. `AuthenticateResponse`)
    pub response_type: &'static str,

    /// Name of the error type (e.g. `FactorError`)
    pub error_type: &'static str,
}

static AUTHENTICATE: HandlerDefinition = HandlerDefinition {
    method: "authenticate",
    endpoint_base_path: EndpointBasePath::Authentication,
    endpoint: "authenticate",
    request_type: "AuthenticateRequest",
    response_type: "AuthenticateResponse",
    error_type: "FactorError",
};

static METHODS: &[&HandlerDefinition] = &[&AUTHENTICATE];
