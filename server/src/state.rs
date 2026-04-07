use std::sync::Arc;

use fred::prelude::Pool;
use mail::MailService;
use mongodb::Database;
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
