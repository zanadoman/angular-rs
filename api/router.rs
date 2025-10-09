use std::{error::Error, path::Path};

use axum_login::{axum::Router, tracing::Level};
use notify::{Event, RecursiveMode, Watcher};
use tokio::time::Duration;
use tower_http::{
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tower_livereload::LiveReloadLayer;
use tower_sessions_sqlx_store::{PostgresStore, sqlx::PgPool};

use crate::{AppState, Authenticator};

#[tracing::instrument(level = "info")]
pub fn new(
    pool: PgPool,
    store: PostgresStore,
    expiry: i64,
    assets: &str,
) -> Result<Router, Box<dyn Error>> {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let mut watcher =
        notify::recommended_watcher(move |event: Result<_, _>| {
            if event.is_ok_and(|evt: Event| !evt.kind.is_access()) {
                reloader.reload();
            }
        })?;
    if cfg!(debug_assertions) {
        watcher.watch(Path::new(assets), RecursiveMode::Recursive)?;
    }
    let router =
        Router::new()
            .nest(
                "/api",
                authentication::router()
                    .layer(Authenticator::new(pool.clone(), store, expiry)?)
                    .with_state(AppState::new(watcher, pool)),
            )
            .layer((
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
                TimeoutLayer::new(Duration::from_mins(1)),
            ))
            .fallback_service(ServeDir::new(assets).not_found_service(
                ServeFile::new(format!("{assets}/index.html")),
            ));
    if cfg!(debug_assertions) {
        Ok(router.layer(livereload))
    } else {
        Ok(router)
    }
}

mod authentication;
