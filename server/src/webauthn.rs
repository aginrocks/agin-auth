use color_eyre::eyre::{Context, Result};
use std::sync::Arc;
use webauthn_rs::{Webauthn, WebauthnBuilder};

use crate::settings::Settings;

pub fn init_webauthn(settings: &Settings) -> Result<Arc<Webauthn>> {
    let client = WebauthnBuilder::new(&settings.webauthn.rp_id, &settings.webauthn.rp_origin)
        .wrap_err("Invalid WebAuthn configuration")?
        .rp_name(&settings.webauthn.rp_name)
        .allow_any_port(settings.webauthn.allow_any_port.unwrap_or(false))
        .allow_subdomains(settings.webauthn.allow_subdomains.unwrap_or(false))
        .build()?;

    let arc_client = Arc::new(client);

    Ok(arc_client)
}
