mod axum_error;
mod database;
mod extractors;
mod middlewares;
mod mongo_id;
mod routes;
mod settings;
mod state;
mod utils;
mod validators;
mod webauthn;

use std::{net::SocketAddr, sync::Arc};

use axum::{Router, http::StatusCode, middleware, response::IntoResponse};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use tokio::net::TcpListener;
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};

use crate::{
    database::{init_database, init_session_store},
    middlewares::require_auth::{require_auth, require_first_factor},
    routes::RouteProtectionLevel,
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

fn init_tracing() -> Result<()> {
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
async fn init_axum(
    state: AppState,
    session_layer: SessionManagerLayer<RedisStore<Pool>>,
) -> Result<Router> {
    let routes = routes::routes();

    // Create separate routers for public and protected routes
    let public_router = OpenApiRouter::with_openapi(ApiDoc::openapi());
    let before_second_factor_router = OpenApiRouter::with_openapi(ApiDoc::openapi());
    let auth_router = OpenApiRouter::with_openapi(ApiDoc::openapi());

    // Add public routes (these don't need authentication)
    let public_router = routes
        .clone()
        .into_iter()
        .filter(|(_, protected)| matches!(*protected, RouteProtectionLevel::Public))
        .fold(public_router, |router, (route, _)| router.routes(route));

    // Add routes that require only first factor of authentication
    let before_second_factor_router = routes
        .clone()
        .into_iter()
        .filter(|(_, protected)| matches!(*protected, RouteProtectionLevel::BeforeTwoFactor))
        .fold(before_second_factor_router, |router, (route, _)| {
            router.routes(route)
        })
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_first_factor,
        ));

    // Add protected routes
    let auth_router = routes
        .clone()
        .into_iter()
        .filter(|(_, protected)| matches!(*protected, RouteProtectionLevel::Authenticated))
        .fold(auth_router, |router, (route, _)| router.routes(route))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Combine the routers
    let router = public_router.merge(before_second_factor_router);

    let router = router.merge(auth_router);

    let router = router.layer(axum::extract::Extension(state.clone()));

    let (router, api) = router.with_state(state).split_for_parts();

    let openapi_prefix = "/apidoc";
    let spec_path = format!("{openapi_prefix}/openapi.json");

    let router = router
        .merge(Redoc::with_url(
            format!("{openapi_prefix}/redoc"),
            api.clone(),
        ))
        .merge(RapiDoc::new(spec_path.clone()).path(format!("{openapi_prefix}/rapidoc")))
        .merge(Scalar::with_url(
            format!("{openapi_prefix}/scalar"),
            api.clone(),
        ))
        .route(
            &spec_path,
            axum::routing::get(|| async move { axum::response::Json(api) }),
        );

    let router = router
        .layer(session_layer)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() });

    Ok(router)
}

async fn init_listener(settings: &Settings) -> Result<TcpListener> {
    let addr: Vec<SocketAddr> = settings.general.listen_address.clone().into();

    Ok(TcpListener::bind(addr.as_slice()).await?)
}
