use color_eyre::eyre::{Context, Result};
use mongodb::{
    Client, Database,
    bson::{self, Bson, doc, oid::ObjectId},
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

use crate::settings::Settings;
use crate::{
    axum_error::AxumResult,
    mongo_id::{object_id_as_string_required, vec_oid_to_vec_string},
};

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
    pub fully_enabled: bool,
}

impl TOTPFactor {
    pub fn to_public(&self) -> PublicTOTPFactor {
        PublicTOTPFactor {
            display_name: self.display_name.clone(),
            fully_enabled: self.fully_enabled,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct WebAuthnFactor {
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

impl From<RecoveryCodeFactor> for Bson {
    fn from(value: RecoveryCodeFactor) -> Self {
        bson::to_bson(&value).unwrap()
    }
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
pub struct RecentFactors {
    pub first_factor: Option<FirstFactor>,
    pub second_factor: Option<SecondFactor>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct AuthFactors {
    pub totp: Option<TOTPFactor>,
    pub webauthn: Vec<WebAuthnFactor>,
    pub recovery_codes: Vec<RecoveryCodeFactor>,
    pub pgp: Option<PGPFactor>,
    pub password: PasswordFactor,
    pub recent: RecentFactors,
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
            pgp: self.pgp.iter().map(|factor| factor.to_public()).collect(),
            password: PublicPasswordFactor {
                is_set: self.password.password_hash.is_some(),
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicTOTPFactor {
    pub display_name: String,
    pub fully_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct PublicPasswordFactor {
    pub is_set: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicWebAuthnFactor {
    pub credential_id: Uuid,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct PublicRecoveryCodeFactor {
    pub remaining_codes: u8,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicPGPFactor {
    pub fingerprint: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct PublicAuthFactors {
    pub totp: Option<PublicTOTPFactor>,
    pub webauthn: Vec<PublicWebAuthnFactor>,
    pub recovery_codes: PublicRecoveryCodeFactor,
    pub pgp: Vec<PublicPGPFactor>,
    pub password: PublicPasswordFactor,
    pub recent: RecentFactors,
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
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(untagged)]
pub enum AnyFactor {
    First(FirstFactor),
    Second(SecondFactor),
}

impl From<FirstFactor> for AnyFactor {
    fn from(f: FirstFactor) -> Self {
        AnyFactor::First(f)
    }
}

impl From<SecondFactor> for AnyFactor {
    fn from(s: SecondFactor) -> Self {
        AnyFactor::Second(s)
    }
}

impl From<AnyFactor> for Bson {
    fn from(value: AnyFactor) -> Self {
        Bson::String(serde_plain::to_string(&value).unwrap())
    }
}

database_object!(User {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    uuid: Uuid,
    first_name: String,
    last_name: String,
    display_name: String,
    preferred_username: String,
    email: String,
    auth_factors: AuthFactors,

    #[serde(with = "vec_oid_to_vec_string")]
    #[schema(value_type = Vec<String>)]
    groups: Vec<ObjectId>,
});

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    Public,
    Confidential,
}

database_object!(Application {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    name: String,
    slug: String,
    icon: String,
    client_type: ClientType,
    client_id: String,
    client_secret: String,
    redirect_uris: Vec<String>,

    #[serde(with = "vec_oid_to_vec_string")]
    #[schema(value_type = Vec<String>)]
    allowed_groups: Vec<ObjectId>,
});

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PublicApplication {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub id: ObjectId,
    pub name: String,
    pub slug: String,
    pub icon: String,
    pub client_type: ClientType,
    pub client_id: String,
    pub redirect_uris: Vec<String>,

    #[serde(with = "vec_oid_to_vec_string")]
    #[schema(value_type = Vec<String>)]
    pub allowed_groups: Vec<ObjectId>,
}

impl Application {
    pub fn to_public(&self) -> PublicApplication {
        PublicApplication {
            id: self.id,
            name: self.name.clone(),
            slug: self.slug.clone(),
            icon: self.icon.clone(),
            client_type: self.client_type.clone(),
            client_id: self.client_id.clone(),
            redirect_uris: self.redirect_uris.clone(),
            allowed_groups: self.allowed_groups.clone(),
        }
    }
}

database_object!(Group {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    name: String,
    is_admin: bool,
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

pub async fn get_user_by_id(
    database: &Database,
    user_id: &ObjectId,
) -> std::result::Result<Option<User>, mongodb::error::Error> {
    database
        .collection::<User>("users")
        .find_one(doc! { "_id": user_id })
        .await
}

pub fn get_second_factors(user: &User) -> Vec<SecondFactor> {
    let mut second_factors = vec![];

    if !user.auth_factors.webauthn.is_empty() {
        second_factors.push(SecondFactor::WebAuthn);
    }

    if user
        .auth_factors
        .totp
        .clone()
        .is_some_and(|totp| totp.fully_enabled)
    {
        second_factors.push(SecondFactor::Totp);
    }

    if !user.auth_factors.recovery_codes.is_empty() {
        second_factors.push(SecondFactor::RecoveryCode);
    }

    second_factors
}

pub async fn set_recent_factor(
    database: &Database,
    user_id: &ObjectId,
    factor: AnyFactor,
) -> AxumResult<()> {
    let update_key = match &factor {
        AnyFactor::First(_) => "auth_factors.recent.first_factor",
        AnyFactor::Second(_) => "auth_factors.recent.second_factor",
    };

    database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": user_id },
            doc! {
                "$set": {
                    update_key: factor
                }
            },
        )
        .await
        .wrap_err("Failed to update recent factor")?;

    Ok(())
}
