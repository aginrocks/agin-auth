use color_eyre::eyre::Result;
use mongodb::{
    Client, Database,
    bson::{doc, oid::ObjectId},
};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_redis_store::{
    RedisStore,
    fred::prelude::{ClientLike, Config, Pool},
};
use utoipa::ToSchema;
use uuid::Uuid;
use visible::StructFields;

use crate::mongo_id::object_id_as_string_required;
use crate::settings::Settings;

macro_rules! database_object {
    ($name:ident { $($field:tt)* }$(, $($omitfield:ident),*)?) => {
        #[derive(Partial, Debug, Serialize, Deserialize, ToSchema, Clone)]
        #[partial(omit(id $(, $($omitfield),* )?), derive(Debug, Serialize, Deserialize, ToSchema, Clone))]
        #[StructFields(pub)]
        pub struct $name {
            $($field)*
        }
    };
}

pub async fn init_database(settings: &Settings) -> Result<Database> {
    let client = Client::with_uri_str(&settings.db.connection_string).await?;
    let database = client.database(&settings.db.database_name);

    Ok(database)
}

pub async fn init_session_store(
    settings: &Settings,
) -> Result<SessionManagerLayer<RedisStore<Pool>>> {
    let config = Config::from_url(&settings.redis.connection_string)?;
    let pool = Pool::new(config, None, None, None, 6)?;

    let _redis_conn = pool.connect();
    pool.wait_for_connect().await?;

    let session_store = RedisStore::<Pool>::new(pool);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    Ok(session_layer)
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TOTPFactor {
    pub secret: String,
    pub display_name: String,
}

impl TOTPFactor {
    pub fn to_public(&self) -> PublicTOTPFactor {
        PublicTOTPFactor {
            display_name: self.display_name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct WebAuthnFactor {
    #[schema(value_type = String)]
    pub credential_id: Uuid,
    pub public_key: String,
    pub display_name: String,
}

impl WebAuthnFactor {
    pub fn to_public(&self) -> PublicWebAuthnFactor {
        PublicWebAuthnFactor {
            credential_id: self.credential_id,
            display_name: self.display_name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RecoveryCodeFactor {
    pub code_hash: String,
    pub used: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PGPFactor {
    pub public_key: String,
    pub fingerprint: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct PasswordFactor {
    pub password_hash: Option<String>,
}

impl PGPFactor {
    pub fn to_public(&self) -> PublicPGPFactor {
        PublicPGPFactor {
            fingerprint: self.fingerprint.clone(),
            display_name: self.display_name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct AuthFactors {
    pub totp: Option<TOTPFactor>,
    pub webauthn: Vec<WebAuthnFactor>,
    pub recovery_codes: Vec<RecoveryCodeFactor>,
    pub gpg: Vec<PGPFactor>,
    pub password: PasswordFactor,
}

impl AuthFactors {
    pub fn to_public(&self) -> PublicAuthFactors {
        let remaining_recovery_codes =
            self.recovery_codes.iter().filter(|code| !code.used).count() as u8;

        PublicAuthFactors {
            totp: self.totp.clone().map(|factor| factor.to_public()),
            webauthn: self
                .webauthn
                .iter()
                .map(|factor| factor.to_public())
                .collect(),
            recovery_codes: PublicRecoveryCodeFactor {
                remaining_codes: remaining_recovery_codes,
            },
            gpg: self.gpg.iter().map(|factor| factor.to_public()).collect(),
            password: PublicPasswordFactor {
                is_set: self.password.password_hash.is_some(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicTOTPFactor {
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicPasswordFactor {
    pub is_set: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicWebAuthnFactor {
    #[schema(value_type = String)]
    pub credential_id: Uuid,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicRecoveryCodeFactor {
    pub remaining_codes: u8,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicPGPFactor {
    pub fingerprint: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicAuthFactors {
    pub totp: Option<PublicTOTPFactor>,
    pub webauthn: Vec<PublicWebAuthnFactor>,
    pub recovery_codes: PublicRecoveryCodeFactor,
    pub gpg: Vec<PublicPGPFactor>,
    pub password: PublicPasswordFactor,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FirstFactor {
    Password,
    WebAuthn,
    Pgp,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SecondFactor {
    Totp,
    WebAuthn,
    RecoveryCode,
    Pgp,
}

database_object!(User {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    #[schema(value_type = String)]
    uuid: Uuid,
    first_name: String,
    last_name: String,
    display_name: String,
    preferred_username: String,
    email: String,
    auth_factors: AuthFactors,
});

pub async fn get_user(
    database: &Database,
    username_or_email: &str,
) -> std::result::Result<Option<User>, mongodb::error::Error> {
    database
        .collection::<User>("users")
        .find_one(doc! {
            "$or": [
                { "preferred_username": username_or_email },
                { "email": username_or_email }
            ]
        })
        .await
}

pub fn get_second_factors(user: &User) -> Vec<SecondFactor> {
    let mut second_factors = vec![];

    if !user.auth_factors.webauthn.is_empty() {
        second_factors.push(SecondFactor::WebAuthn);
    }

    if user.auth_factors.totp.is_some() {
        second_factors.push(SecondFactor::Totp);
    }

    if !user.auth_factors.gpg.is_empty() {
        second_factors.push(SecondFactor::Pgp);
    }

    if !user.auth_factors.recovery_codes.is_empty() {
        second_factors.push(SecondFactor::RecoveryCode);
    }

    second_factors
}
