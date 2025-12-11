use std::net::SocketAddr;

use axum::{Extension, Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use color_eyre::Result;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};
use tracing::{Level, instrument, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};

use crate::{settings::Settings, state::AppState};

pub fn init_tracing() -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}

#[instrument(skip(state, session_layer))]
pub async fn init_axum(
    state: AppState,
    session_layer: SessionManagerLayer<RedisStore<Pool>>,
) -> Result<Router> {
    let router = crate::routes::routes();

    let (router, api) = router.with_state(state.clone()).split_for_parts();

    let openapi_prefix = "/apidoc";
    let spec_name = "/openapi.json";

    let docs = Router::new()
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new(format!("{openapi_prefix}{spec_name}")).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api.clone()))
        .route(spec_name, get(|| async move { Json(api) }));

    let router = router
        .nest(openapi_prefix, docs)
        .layer(Extension(state))
        .layer(session_layer)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() })
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG)),
        );

    Ok(router)
}

pub async fn init_listener(settings: &Settings) -> Result<TcpListener> {
    let addr: Vec<SocketAddr> = settings.general.listen_address.clone().into();

    Ok(TcpListener::bind(addr.as_slice()).await?)
}
