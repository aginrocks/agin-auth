use axum::{
    Extension, Json,
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use chrono::Utc;
use color_eyre::eyre::{self, Context as _};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Application, User, get_user_by_uuid},
    oidc::{
        AccessTokenClaims, AuthorizationCode, IdTokenClaims, RefreshToken, build_discovery,
    },
    state::AppState,
    utils::{generate_reset_token, hash_token},
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(jwks))
        .routes(routes!(authorize_get, authorize_post))
        .routes(routes!(token))
        .routes(routes!(userinfo))
}

// ── Discovery ────────────────────────────────────────────────────

/// OpenID Connect Discovery
#[utoipa::path(
    method(get),
    path = "/.well-known/openid-configuration",
    responses(
        (status = OK, description = "OIDC Discovery Document"),
    ),
    tag = "OAuth"
)]
async fn discovery(Extension(state): Extension<AppState>) -> impl IntoResponse {
    let issuer = state.settings.general.public_url.to_string().trim_end_matches('/').to_string();
    Json(build_discovery(&issuer))
}

// ── JWKS ─────────────────────────────────────────────────────────

/// JSON Web Key Set
#[utoipa::path(
    method(get),
    path = "/jwks",
    responses(
        (status = OK, description = "JWKS Document"),
    ),
    tag = "OAuth"
)]
async fn jwks(Extension(state): Extension<AppState>) -> impl IntoResponse {
    Json(state.oidc_keys.jwks_json.clone())
}

// ── Authorize (GET) ──────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuthorizeQuery {
    pub client_id: String,
    pub redirect_uri: String,
    pub response_type: String,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub nonce: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthorizeInfo {
    pub app_name: String,
    pub app_icon: Option<String>,
    pub scopes: Vec<String>,
    pub client_id: String,
    pub redirect_uri: String,
    pub state: Option<String>,
    pub nonce: Option<String>,
}

/// Get authorization info (requires session)
#[utoipa::path(
    method(get),
    path = "/authorize",
    params(
        ("client_id" = String, Query,),
        ("redirect_uri" = String, Query,),
        ("response_type" = String, Query,),
        ("scope" = Option<String>, Query,),
        ("state" = Option<String>, Query,),
        ("nonce" = Option<String>, Query,),
    ),
    responses(
        (status = OK, description = "Authorization info", body = AuthorizeInfo),
        (status = BAD_REQUEST, description = "Invalid request"),
    ),
    tag = "OAuth"
)]
async fn authorize_get(
    Extension(state): Extension<AppState>,
    session: Session,
    Query(params): Query<AuthorizeQuery>,
) -> AxumResult<Json<AuthorizeInfo>> {
    // Check user is authenticated
    let user_id = session
        .get::<mongodb::bson::oid::ObjectId>("user_id")
        .await?;
    let auth_state = session
        .get::<crate::routes::api::AuthState>("auth_state")
        .await?;

    if user_id.is_none()
        || !matches!(auth_state, Some(crate::routes::api::AuthState::Authenticated))
    {
        return Err(AxumError::unauthorized(eyre::eyre!(
            "Login required. Redirect to login page first."
        )));
    }

    // Validate response_type
    if params.response_type != "code" {
        return Err(AxumError::bad_request(eyre::eyre!(
            "Unsupported response_type. Only 'code' is supported."
        )));
    }

    // Look up application by client_id
    let app = state
        .database
        .collection::<Application>("applications")
        .find_one(doc! { "client_id": &params.client_id })
        .await
        .wrap_err("Database error")?
        .ok_or_else(|| AxumError::bad_request(eyre::eyre!("Unknown client_id")))?;

    // Validate redirect_uri
    if !app.redirect_uris.contains(&params.redirect_uri) {
        return Err(AxumError::bad_request(eyre::eyre!(
            "Invalid redirect_uri for this application"
        )));
    }

    // Parse scopes
    let scopes: Vec<String> = params
        .scope
        .unwrap_or_default()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let valid_scopes = ["openid", "profile", "email", "offline_access"];
    let filtered_scopes: Vec<String> = scopes
        .into_iter()
        .filter(|s| valid_scopes.contains(&s.as_str()))
        .collect();

    Ok(Json(AuthorizeInfo {
        app_name: app.name,
        app_icon: app.icon,
        scopes: filtered_scopes,
        client_id: params.client_id,
        redirect_uri: params.redirect_uri,
        state: params.state,
        nonce: params.nonce,
    }))
}

// ── Authorize (POST) ─────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuthorizeConsent {
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: Option<String>,
    pub nonce: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthorizeResponse {
    pub redirect_url: String,
}

/// Approve authorization (user consent)
#[utoipa::path(
    method(post),
    path = "/authorize",
    request_body = AuthorizeConsent,
    responses(
        (status = OK, description = "Authorization code issued", body = AuthorizeResponse),
        (status = BAD_REQUEST, description = "Invalid request"),
        (status = UNAUTHORIZED, description = "Not authenticated"),
    ),
    tag = "OAuth"
)]
async fn authorize_post(
    Extension(state): Extension<AppState>,
    session: Session,
    Json(body): Json<AuthorizeConsent>,
) -> AxumResult<Json<AuthorizeResponse>> {
    // Check user is authenticated
    let user_id = session
        .get::<mongodb::bson::oid::ObjectId>("user_id")
        .await?
        .ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Not authenticated")))?;

    let auth_state = session
        .get::<crate::routes::api::AuthState>("auth_state")
        .await?;
    if !matches!(auth_state, Some(crate::routes::api::AuthState::Authenticated)) {
        return Err(AxumError::unauthorized(eyre::eyre!("Not authenticated")));
    }

    // Validate application
    let app = state
        .database
        .collection::<Application>("applications")
        .find_one(doc! { "client_id": &body.client_id })
        .await
        .wrap_err("Database error")?
        .ok_or_else(|| AxumError::bad_request(eyre::eyre!("Unknown client_id")))?;

    if !app.redirect_uris.contains(&body.redirect_uri) {
        return Err(AxumError::bad_request(eyre::eyre!(
            "Invalid redirect_uri"
        )));
    }

    // Check user group access
    let user = state
        .database
        .collection::<User>("users")
        .find_one(doc! { "_id": &user_id })
        .await
        .wrap_err("Database error")?
        .ok_or_else(|| AxumError::bad_request(eyre::eyre!("User not found")))?;

    if !app.allowed_groups.is_empty() {
        let has_access = user
            .groups
            .iter()
            .any(|g| app.allowed_groups.contains(g));
        if !has_access {
            return Err(AxumError::forbidden(eyre::eyre!(
                "You don't have access to this application"
            )));
        }
    }

    // Generate authorization code
    let code = generate_reset_token(); // 64 char random string
    let code_hash = hash_token(&code);

    let auth_code = AuthorizationCode {
        code_hash,
        client_id: body.client_id.clone(),
        user_id: user_id.to_hex(),
        redirect_uri: body.redirect_uri.clone(),
        scope: body.scope.clone(),
        nonce: body.nonce.clone(),
        created_at: Utc::now(),
        used: false,
    };

    state
        .database
        .collection::<AuthorizationCode>("authorization_codes")
        .insert_one(auth_code)
        .await
        .wrap_err("Failed to store authorization code")?;

    // Build redirect URL with code and state
    let mut redirect_url = body.redirect_uri.clone();
    redirect_url.push_str(if redirect_url.contains('?') {
        "&"
    } else {
        "?"
    });
    redirect_url.push_str(&format!("code={code}"));
    if let Some(ref st) = body.state {
        redirect_url.push_str(&format!("&state={}", urlencoding::encode(st)));
    }

    Ok(Json(AuthorizeResponse { redirect_url }))
}

// ── Token ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    pub scope: String,
}

#[derive(Debug, Serialize)]
pub struct TokenError {
    pub error: String,
    pub error_description: String,
}

/// OAuth2 Token endpoint
#[utoipa::path(
    method(post),
    path = "/token",
    responses(
        (status = OK, description = "Token response", body = TokenResponse),
        (status = BAD_REQUEST, description = "Token error"),
    ),
    tag = "OAuth"
)]
async fn token(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    axum::Form(body): axum::Form<TokenRequest>,
) -> Result<Json<TokenResponse>, axum::response::Response> {
    // Extract client credentials from Basic auth header or body
    let (client_id, client_secret) = extract_client_credentials(&headers, &body);

    match body.grant_type.as_str() {
        "authorization_code" => {
            handle_authorization_code_grant(&state, &body, &client_id, &client_secret).await
        }
        "refresh_token" => {
            handle_refresh_token_grant(&state, &body, &client_id, &client_secret).await
        }
        _ => Err(token_error(
            StatusCode::BAD_REQUEST,
            "unsupported_grant_type",
            "Only authorization_code and refresh_token are supported",
        )),
    }
}

fn extract_client_credentials(
    headers: &HeaderMap,
    body: &TokenRequest,
) -> (Option<String>, Option<String>) {
    // Try Basic auth first
    if let Some(auth) = headers.get("authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if let Some(basic) = auth_str.strip_prefix("Basic ") {
                if let Ok(decoded) = base64::Engine::decode(
                    &base64::engine::general_purpose::STANDARD,
                    basic.trim(),
                ) {
                    if let Ok(creds) = String::from_utf8(decoded) {
                        if let Some((id, secret)) = creds.split_once(':') {
                            return (Some(id.to_string()), Some(secret.to_string()));
                        }
                    }
                }
            }
        }
    }

    // Fall back to body parameters
    (body.client_id.clone(), body.client_secret.clone())
}

async fn handle_authorization_code_grant(
    state: &AppState,
    body: &TokenRequest,
    client_id: &Option<String>,
    client_secret: &Option<String>,
) -> Result<Json<TokenResponse>, axum::response::Response> {
    let code = body.code.as_ref().ok_or_else(|| {
        token_error(StatusCode::BAD_REQUEST, "invalid_request", "Missing code")
    })?;

    let redirect_uri = body.redirect_uri.as_ref().ok_or_else(|| {
        token_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "Missing redirect_uri",
        )
    })?;

    let client_id = client_id.as_ref().ok_or_else(|| {
        token_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "Missing client_id",
        )
    })?;

    // Find the authorization code
    let code_hash = hash_token(code);
    let auth_code = state
        .database
        .collection::<AuthorizationCode>("authorization_codes")
        .find_one(doc! { "code_hash": &code_hash, "used": false })
        .await
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Database error",
            )
        })?
        .ok_or_else(|| {
            token_error(
                StatusCode::BAD_REQUEST,
                "invalid_grant",
                "Invalid or expired authorization code",
            )
        })?;

    // Validate code hasn't expired (10 minute lifetime)
    let age = Utc::now() - auth_code.created_at;
    if age.num_minutes() > 10 {
        // Mark as used to prevent replay
        let _ = state
            .database
            .collection::<AuthorizationCode>("authorization_codes")
            .update_one(
                doc! { "code_hash": &code_hash },
                doc! { "$set": { "used": true } },
            )
            .await;

        return Err(token_error(
            StatusCode::BAD_REQUEST,
            "invalid_grant",
            "Authorization code expired",
        ));
    }

    // Validate client_id and redirect_uri match
    if auth_code.client_id != *client_id {
        return Err(token_error(
            StatusCode::BAD_REQUEST,
            "invalid_grant",
            "client_id mismatch",
        ));
    }
    if auth_code.redirect_uri != *redirect_uri {
        return Err(token_error(
            StatusCode::BAD_REQUEST,
            "invalid_grant",
            "redirect_uri mismatch",
        ));
    }

    // Validate client_secret for confidential clients
    let app = state
        .database
        .collection::<Application>("applications")
        .find_one(doc! { "client_id": client_id })
        .await
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Database error",
            )
        })?
        .ok_or_else(|| {
            token_error(
                StatusCode::BAD_REQUEST,
                "invalid_client",
                "Unknown client",
            )
        })?;

    if matches!(app.client_type, crate::database::ClientType::Confidential) {
        let expected_secret = app.client_secret.as_ref().ok_or_else(|| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Confidential client has no secret",
            )
        })?;
        let provided_secret = client_secret.as_ref().ok_or_else(|| {
            token_error(
                StatusCode::UNAUTHORIZED,
                "invalid_client",
                "Client secret required",
            )
        })?;
        if provided_secret != expected_secret {
            return Err(token_error(
                StatusCode::UNAUTHORIZED,
                "invalid_client",
                "Invalid client secret",
            ));
        }
    }

    // Mark code as used
    state
        .database
        .collection::<AuthorizationCode>("authorization_codes")
        .update_one(
            doc! { "code_hash": &code_hash },
            doc! { "$set": { "used": true } },
        )
        .await
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Failed to invalidate code",
            )
        })?;

    // Get user for claims
    let user_oid = mongodb::bson::oid::ObjectId::parse_str(&auth_code.user_id).map_err(|_| {
        token_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "server_error",
            "Invalid user_id",
        )
    })?;

    let user = state
        .database
        .collection::<User>("users")
        .find_one(doc! { "_id": &user_oid })
        .await
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Database error",
            )
        })?
        .ok_or_else(|| {
            token_error(
                StatusCode::BAD_REQUEST,
                "invalid_grant",
                "User not found",
            )
        })?;

    let issuer = state
        .settings
        .general
        .public_url
        .to_string()
        .trim_end_matches('/')
        .to_string();

    let now = Utc::now().timestamp() as usize;
    let scopes: Vec<&str> = auth_code.scope.split_whitespace().collect();

    // Build access token (1 hour)
    let access_claims = AccessTokenClaims {
        iss: issuer.clone(),
        sub: user.uuid.to_string(),
        aud: client_id.clone(),
        exp: now + 3600,
        iat: now,
        scope: auth_code.scope.clone(),
        client_id: client_id.clone(),
    };
    let access_token = state
        .oidc_keys
        .sign_access_token(&access_claims)
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Failed to sign access token",
            )
        })?;

    // Build ID token if openid scope requested
    let id_token = if scopes.contains(&"openid") {
        let mut claims = IdTokenClaims {
            iss: issuer.clone(),
            sub: user.uuid.to_string(),
            aud: client_id.clone(),
            exp: now + 3600,
            iat: now,
            nonce: auth_code.nonce.clone(),
            name: None,
            preferred_username: None,
            email: None,
            email_verified: None,
            given_name: None,
            family_name: None,
        };

        if scopes.contains(&"profile") {
            claims.name = Some(user.display_name.clone());
            claims.preferred_username = Some(user.preferred_username.clone());
            claims.given_name = Some(user.first_name.clone());
            claims.family_name = Some(user.last_name.clone());
        }
        if scopes.contains(&"email") {
            claims.email = Some(user.email.clone());
            claims.email_verified = Some(user.email_confirmed);
        }

        Some(state.oidc_keys.sign_id_token(&claims).map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Failed to sign ID token",
            )
        })?)
    } else {
        None
    };

    // Generate refresh token if offline_access scope requested
    let refresh_token = if scopes.contains(&"offline_access") {
        let raw_token = generate_reset_token();
        let token_hash = hash_token(&raw_token);

        let rt = RefreshToken {
            token_hash,
            client_id: client_id.clone(),
            user_id: auth_code.user_id.clone(),
            scope: auth_code.scope.clone(),
            created_at: Utc::now(),
            revoked: false,
        };

        state
            .database
            .collection::<RefreshToken>("refresh_tokens")
            .insert_one(rt)
            .await
            .map_err(|_| {
                token_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "server_error",
                    "Failed to store refresh token",
                )
            })?;

        Some(raw_token)
    } else {
        None
    };

    Ok(Json(TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        refresh_token,
        id_token,
        scope: auth_code.scope,
    }))
}

async fn handle_refresh_token_grant(
    state: &AppState,
    body: &TokenRequest,
    client_id: &Option<String>,
    client_secret: &Option<String>,
) -> Result<Json<TokenResponse>, axum::response::Response> {
    let raw_token = body.refresh_token.as_ref().ok_or_else(|| {
        token_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "Missing refresh_token",
        )
    })?;

    let token_hash = hash_token(raw_token);
    let stored = state
        .database
        .collection::<RefreshToken>("refresh_tokens")
        .find_one(doc! { "token_hash": &token_hash, "revoked": false })
        .await
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Database error",
            )
        })?
        .ok_or_else(|| {
            token_error(
                StatusCode::BAD_REQUEST,
                "invalid_grant",
                "Invalid or revoked refresh token",
            )
        })?;

    // Validate refresh token age (30 days max)
    let age = Utc::now() - stored.created_at;
    if age.num_days() > 30 {
        let _ = state
            .database
            .collection::<RefreshToken>("refresh_tokens")
            .update_one(
                doc! { "token_hash": &token_hash },
                doc! { "$set": { "revoked": true } },
            )
            .await;
        return Err(token_error(
            StatusCode::BAD_REQUEST,
            "invalid_grant",
            "Refresh token expired",
        ));
    }

    // Validate client
    let req_client_id = client_id.as_ref().ok_or_else(|| {
        token_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "Missing client_id",
        )
    })?;

    if stored.client_id != *req_client_id {
        return Err(token_error(
            StatusCode::BAD_REQUEST,
            "invalid_grant",
            "client_id mismatch",
        ));
    }

    // Validate client_secret for confidential clients
    let app = state
        .database
        .collection::<Application>("applications")
        .find_one(doc! { "client_id": req_client_id })
        .await
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Database error",
            )
        })?
        .ok_or_else(|| {
            token_error(
                StatusCode::BAD_REQUEST,
                "invalid_client",
                "Unknown client",
            )
        })?;

    if matches!(app.client_type, crate::database::ClientType::Confidential) {
        let expected = app.client_secret.as_ref().ok_or_else(|| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Confidential client has no secret",
            )
        })?;
        let provided = client_secret.as_ref().ok_or_else(|| {
            token_error(
                StatusCode::UNAUTHORIZED,
                "invalid_client",
                "Client secret required",
            )
        })?;
        if provided != expected {
            return Err(token_error(
                StatusCode::UNAUTHORIZED,
                "invalid_client",
                "Invalid client secret",
            ));
        }
    }

    // Get user
    let user_oid =
        mongodb::bson::oid::ObjectId::parse_str(&stored.user_id).map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Invalid user_id",
            )
        })?;

    let user = state
        .database
        .collection::<User>("users")
        .find_one(doc! { "_id": &user_oid })
        .await
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Database error",
            )
        })?
        .ok_or_else(|| {
            token_error(
                StatusCode::BAD_REQUEST,
                "invalid_grant",
                "User not found",
            )
        })?;

    let issuer = state
        .settings
        .general
        .public_url
        .to_string()
        .trim_end_matches('/')
        .to_string();
    let now = Utc::now().timestamp() as usize;

    let access_claims = AccessTokenClaims {
        iss: issuer,
        sub: user.uuid.to_string(),
        aud: req_client_id.clone(),
        exp: now + 3600,
        iat: now,
        scope: stored.scope.clone(),
        client_id: req_client_id.clone(),
    };

    let access_token = state
        .oidc_keys
        .sign_access_token(&access_claims)
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Failed to sign access token",
            )
        })?;

    Ok(Json(TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        refresh_token: None,
        id_token: None,
        scope: stored.scope,
    }))
}

// ── UserInfo ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfoResponse {
    pub sub: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
}

/// OpenID Connect UserInfo endpoint
#[utoipa::path(
    method(get),
    path = "/userinfo",
    responses(
        (status = OK, description = "User info", body = UserInfoResponse),
        (status = UNAUTHORIZED, description = "Invalid or missing access token"),
    ),
    tag = "OAuth"
)]
async fn userinfo(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserInfoResponse>, axum::response::Response> {
    // Extract Bearer token
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            token_error(
                StatusCode::UNAUTHORIZED,
                "invalid_token",
                "Missing or invalid Bearer token",
            )
        })?;

    // Verify access token
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_audience(&[""]); // We'll skip audience validation for userinfo
    validation.validate_aud = false;

    let token_data = jsonwebtoken::decode::<AccessTokenClaims>(
        auth_header,
        &state.oidc_keys.decoding_key,
        &validation,
    )
    .map_err(|e| {
        token_error(
            StatusCode::UNAUTHORIZED,
            "invalid_token",
            &format!("Token verification failed: {e}"),
        )
    })?;

    let claims = token_data.claims;

    // Get user by UUID
    let user_uuid = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        token_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "server_error",
            "Invalid sub claim",
        )
    })?;

    let user = get_user_by_uuid(&state.database, &user_uuid)
        .await
        .map_err(|_| {
            token_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Database error",
            )
        })?
        .ok_or_else(|| {
            token_error(
                StatusCode::UNAUTHORIZED,
                "invalid_token",
                "User not found",
            )
        })?;

    let scopes: Vec<&str> = claims.scope.split_whitespace().collect();

    let mut response = UserInfoResponse {
        sub: user.uuid.to_string(),
        name: None,
        preferred_username: None,
        given_name: None,
        family_name: None,
        email: None,
        email_verified: None,
    };

    if scopes.contains(&"profile") {
        response.name = Some(user.display_name);
        response.preferred_username = Some(user.preferred_username);
        response.given_name = Some(user.first_name);
        response.family_name = Some(user.last_name);
    }

    if scopes.contains(&"email") {
        response.email = Some(user.email);
        response.email_verified = Some(user.email_confirmed);
    }

    Ok(Json(response))
}

// ── Helpers ──────────────────────────────────────────────────────

fn token_error(
    status: StatusCode,
    error: &str,
    description: &str,
) -> axum::response::Response {
    let body = TokenError {
        error: error.to_string(),
        error_description: description.to_string(),
    };
    (status, Json(body)).into_response()
}
