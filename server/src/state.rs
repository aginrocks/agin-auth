use std::sync::Arc;

use mail::MailService;
use mongodb::Database;
use webauthn_rs::Webauthn;

use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub settings: Arc<Settings>,
    pub webauthn: Arc<Webauthn>,
    pub mail_service: Option<Arc<MailService>>,
}
