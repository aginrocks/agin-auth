mod axum_error;
mod database;
mod entity;
mod extractors;
mod factors;
mod init;
mod middlewares;
mod mongo_id;
mod routes;
mod settings;
mod state;
mod utils;
mod validators;
mod webauthn;

use std::sync::Arc;

use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use tracing::info;
use utoipa::OpenApi;

use crate::{
    database::{init_database, init_session_store},
    init::{init_axum, init_listener, init_tracing},
    settings::Settings,
    state::AppState,
    webauthn::init_webauthn,
};

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenvy::dotenv().ok();
    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    let settings = Arc::new(Settings::try_load()?);

    let database = init_database(&settings).await?;

    let webauthn = init_webauthn(&settings)?;

    let app_state = AppState {
        database,
        settings: settings.clone(),
        webauthn,
    };

    let session_layer = init_session_store(&settings).await?;
    let app = init_axum(app_state, session_layer).await?;
    let listener = init_listener(&settings).await?;

    info!(
        "listening on {} ({})",
        listener
            .local_addr()
            .wrap_err("failed to get local address")?,
        settings.general.public_url
    );

    axum::serve(listener, app.into_make_service())
        .await
        .wrap_err("failed to run server")?;

    Ok(())
}
