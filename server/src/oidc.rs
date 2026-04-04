use std::sync::Arc;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use color_eyre::eyre::{Context, Result};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::pkcs8::LineEnding;
use rsa::traits::PublicKeyParts;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::info;

/// RSA key size for OIDC signing
const RSA_KEY_BITS: usize = 2048;

/// OIDC signing key data stored in AppState
#[derive(Clone)]
pub struct OidcKeys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub kid: String,
    pub jwks_json: JwksDocument,
}

/// JWKS document returned at /oauth/jwks
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JwksDocument {
    pub keys: Vec<Jwk>,
}

/// Single JWK entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwk {
    pub kty: String,
    pub r#use: String,
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

/// Standard OIDC ID token claims
#[derive(Debug, Serialize, Deserialize)]
pub struct IdTokenClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
}

/// Access token claims (shorter, just authorization)
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub scope: String,
    pub client_id: String,
}

/// OpenID Connect Discovery document
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryDocument {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
    pub response_types_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub grant_types_supported: Vec<String>,
}

/// Authorization code stored in MongoDB
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorizationCode {
    pub code_hash: String,
    pub client_id: String,
    pub user_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub nonce: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub used: bool,
}

/// Refresh token stored in MongoDB
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshToken {
    pub token_hash: String,
    pub client_id: String,
    pub user_id: String,
    pub scope: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub revoked: bool,
}

const KEY_FILE: &str = "oidc-signing-key.pem";

/// Initialize OIDC keys — load from file or generate new
pub fn init_oidc_keys() -> Result<Arc<OidcKeys>> {
    let (private_key, public_key) = if std::path::Path::new(KEY_FILE).exists() {
        info!("Loading OIDC signing key from {KEY_FILE}");
        let pem = std::fs::read_to_string(KEY_FILE)
            .wrap_err("Failed to read OIDC signing key")?;
        let private_key = rsa::RsaPrivateKey::from_pkcs1_pem(&pem)
            .wrap_err("Failed to parse OIDC signing key")?;
        let public_key = RsaPublicKey::from(&private_key);
        (private_key, public_key)
    } else {
        info!("Generating new OIDC signing key ({RSA_KEY_BITS} bits)...");
        let mut rng = rsa::rand_core::OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_BITS)
            .wrap_err("Failed to generate RSA key")?;
        let public_key = RsaPublicKey::from(&private_key);

        // Save for persistence across restarts
        let pem = private_key
            .to_pkcs1_pem(LineEnding::LF)
            .wrap_err("Failed to encode RSA key to PEM")?;
        std::fs::write(KEY_FILE, pem.as_str())
            .wrap_err("Failed to write OIDC signing key")?;
        info!("OIDC signing key saved to {KEY_FILE}");

        (private_key, public_key)
    };

    // Build jsonwebtoken keys
    let private_pem = private_key
        .to_pkcs1_pem(LineEnding::LF)
        .wrap_err("Failed to encode private key")?;
    let encoding_key = EncodingKey::from_rsa_pem(private_pem.as_bytes())
        .wrap_err("Failed to create encoding key")?;

    let public_pem = public_key
        .to_pkcs1_pem(LineEnding::LF)
        .wrap_err("Failed to encode public key")?;
    let decoding_key = DecodingKey::from_rsa_pem(public_pem.as_bytes())
        .wrap_err("Failed to create decoding key")?;

    // Build kid from public key hash
    let kid = {
        let der = public_key
            .to_pkcs1_der()
            .wrap_err("Failed to encode public key DER")?;
        let hash = Sha256::digest(der.as_bytes());
        URL_SAFE_NO_PAD.encode(&hash[..8])
    };

    // Build JWKS
    let n = URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be());
    let e = URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be());

    let jwk = Jwk {
        kty: "RSA".to_string(),
        r#use: "sig".to_string(),
        kid: kid.clone(),
        alg: "RS256".to_string(),
        n,
        e,
    };

    let jwks_json = JwksDocument { keys: vec![jwk] };

    info!("OIDC provider initialized with kid={kid}");

    Ok(Arc::new(OidcKeys {
        encoding_key,
        decoding_key,
        kid,
        jwks_json,
    }))
}

impl OidcKeys {
    /// Sign an ID token JWT
    pub fn sign_id_token(&self, claims: &IdTokenClaims) -> Result<String> {
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.kid.clone());

        jsonwebtoken::encode(&header, claims, &self.encoding_key)
            .wrap_err("Failed to sign ID token")
    }

    /// Sign an access token JWT
    pub fn sign_access_token(&self, claims: &AccessTokenClaims) -> Result<String> {
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.kid.clone());

        jsonwebtoken::encode(&header, claims, &self.encoding_key)
            .wrap_err("Failed to sign access token")
    }
}

/// Build the OIDC discovery document
pub fn build_discovery(issuer: &str) -> DiscoveryDocument {
    DiscoveryDocument {
        issuer: issuer.to_string(),
        authorization_endpoint: format!("{issuer}/api/oauth/authorize"),
        token_endpoint: format!("{issuer}/api/oauth/token"),
        userinfo_endpoint: format!("{issuer}/api/oauth/userinfo"),
        jwks_uri: format!("{issuer}/api/oauth/jwks"),
        response_types_supported: vec!["code".to_string()],
        subject_types_supported: vec!["public".to_string()],
        id_token_signing_alg_values_supported: vec!["RS256".to_string()],
        scopes_supported: vec![
            "openid".to_string(),
            "profile".to_string(),
            "email".to_string(),
            "offline_access".to_string(),
        ],
        token_endpoint_auth_methods_supported: vec![
            "client_secret_basic".to_string(),
            "client_secret_post".to_string(),
            "none".to_string(),
        ],
        claims_supported: vec![
            "sub".to_string(),
            "iss".to_string(),
            "aud".to_string(),
            "exp".to_string(),
            "iat".to_string(),
            "name".to_string(),
            "given_name".to_string(),
            "family_name".to_string(),
            "preferred_username".to_string(),
            "email".to_string(),
            "email_verified".to_string(),
        ],
        grant_types_supported: vec![
            "authorization_code".to_string(),
            "refresh_token".to_string(),
        ],
    }
}
