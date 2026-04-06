use std::sync::Arc;

use mail::MailService;
use mongodb::Database;
use tower_sessions_redis_store::fred::prelude::Pool;
use webauthn_rs::Webauthn;

use crate::{oidc::OidcKeys, settings::Settings};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub settings: Arc<Settings>,
    pub webauthn: Arc<Webauthn>,
    pub mail_service: Option<Arc<MailService>>,
    pub oidc_keys: Arc<OidcKeys>,
    pub redis_pool: Pool,
}
