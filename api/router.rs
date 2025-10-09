use std::{error::Error, path::Path};

use axum_login::{
    axum::{Router, http::StatusCode},
    tracing::Level,
};
use notify::{Event, RecursiveMode, Watcher};
use tokio::time::Duration;
use tower_http::{
    catch_panic::CatchPanicLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tower_livereload::LiveReloadLayer;
use tower_sessions_sqlx_store::{PostgresStore, sqlx::PgPool};
use utoipa::openapi::Info;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{AppState, Authenticator};

#[tracing::instrument(level = Level::INFO, err)]
pub fn new(
    pool: PgPool,
    store: PostgresStore,
    expiry: i64,
    assets: &str,
) -> Result<Router, Box<dyn Error + Send + Sync>> {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let mut watcher = notify::recommended_watcher(move |e: Result<_, _>| {
        if e.is_ok_and(|e: Event| !e.kind.is_access()) {
            reloader.reload();
        }
    })?;
    if cfg!(debug_assertions) {
        watcher.watch(Path::new(assets), RecursiveMode::Recursive)?;
    }
    let (mut router, mut api) = OpenApiRouter::new()
        .nest(
            "/api",
            authentication::router()
                .fallback(async || StatusCode::NOT_FOUND)
                .layer(Authenticator::new(pool.clone(), store, expiry)?)
                .with_state(AppState::new(pool, watcher)),
        )
        .split_for_parts();
    if cfg!(debug_assertions) {
        api.info = Info::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        router = router.merge(
            SwaggerUi::new("/api/docs/swagger-ui")
                .url("/api/docs/openapi.json", api),
        );
    }
    router = router.fallback_service(
        ServeDir::new(assets)
            .not_found_service(ServeFile::new(format!("{assets}/index.html"))),
    );
    if cfg!(debug_assertions) {
        router = router.layer(livereload);
    }
    Ok(router.layer((
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
        CatchPanicLayer::new(),
        TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_mins(1),
        ),
    )))
}

mod authentication;
