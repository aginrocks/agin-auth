mod templates;

use color_eyre::Result;
use mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: String,
    /// Use implicit TLS (port 465). Set to false for STARTTLS (port 587).
    #[serde(default)]
    pub implicit_tls: bool,
}

pub struct MailService {
    config: MailConfig,
}

impl MailService {
    pub fn new(config: MailConfig) -> Self {
        Self { config }
    }

    async fn send(&self, to: &str, subject: &str, html: String) -> Result<()> {
        let message = MessageBuilder::new()
            .from((self.config.from_name.as_str(), self.config.from_address.as_str()))
            .to(vec![to])
            .subject(subject)
            .html_body(html);

        SmtpClientBuilder::new(self.config.host.as_str(), self.config.port)
            .implicit_tls(self.config.implicit_tls)
            .credentials((self.config.username.as_str(), self.config.password.as_str()))
            .connect()
            .await?
            .send(message)
            .await?;

        Ok(())
    }

    pub async fn send_password_reset(
        &self,
        to: &str,
        app_name: &str,
        reset_url: &str,
    ) -> Result<()> {
        let html = templates::password_reset(app_name, reset_url);
        self.send(to, &format!("Reset your {app_name} password"), html)
            .await
    }

    /// Not yet implemented — will be called once email confirmation flow is ready.
    pub async fn send_email_confirmation(
        &self,
        to: &str,
        app_name: &str,
        confirm_url: &str,
    ) -> Result<()> {
        let html = templates::email_confirmation(app_name, confirm_url);
        self.send(to, &format!("Confirm your {app_name} email address"), html)
            .await
    }

    /// Not yet implemented — will be called once login notification flow is ready (after DB migration).
    pub async fn send_login_notification(
        &self,
        to: &str,
        app_name: &str,
        ip: &str,
    ) -> Result<()> {
        let html = templates::login_notification(app_name, ip);
        self.send(to, &format!("New login to your {app_name} account"), html)
            .await
    }
}

// ── Standalone render helpers (no SMTP needed, useful for previews/tests) ──

pub fn render_password_reset(app_name: &str, reset_url: &str) -> String {
    templates::password_reset(app_name, reset_url)
}

pub fn render_email_confirmation(app_name: &str, confirm_url: &str) -> String {
    templates::email_confirmation(app_name, confirm_url)
}

pub fn render_login_notification(app_name: &str, ip: &str) -> String {
    templates::login_notification(app_name, ip)
}
