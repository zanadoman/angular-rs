#![feature(bool_to_result)]
#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::{error::Error, path::Path};

use axum_login::{
  axum::{Router, http::StatusCode},
  tracing::Level,
};
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use notify::{Event, RecursiveMode, Watcher};
use tokio::{fs, time::Duration};
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

use self::utils::{
  app_error::AppError,
  app_state::AppState,
  authenticator::{Authenticator, Credentials},
  validation::Validation,
};

#[tracing::instrument(level = Level::INFO, err)]
pub async fn new(
  pool: PgPool,
  store: PostgresStore,
  mailer: AsyncSmtpTransport<Tokio1Executor>,
  expiry: i64,
  assets: &Path,
) -> Result<Router, Box<dyn Error + Send + Sync>> {
  let livereload = LiveReloadLayer::new();
  let reloader = livereload.reloader();
  let mut watcher = notify::recommended_watcher(move |e: Result<_, _>| {
    if e.is_ok_and(|e: Event| !e.kind.is_access()) {
      reloader.reload();
    }
  })?;
  if cfg!(debug_assertions) {
    fs::create_dir_all(assets).await?;
    watcher.watch(assets, RecursiveMode::Recursive)?;
  }
  let (mut router, mut api) = OpenApiRouter::new()
    .nest(
      "/api",
      router::router()
        .fallback(async || StatusCode::NOT_FOUND)
        .layer(Authenticator::new(pool.clone(), store, expiry)?)
        .with_state(AppState::new(pool, mailer, watcher)),
    )
    .split_for_parts();
  if cfg!(debug_assertions) {
    api.info = Info::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    router =
      router.merge(SwaggerUi::new("/api/docs/swagger-ui").url("/api/docs/openapi.json", api));
  }
  router = router.fallback_service(
    ServeDir::new(assets)
      .not_found_service(ServeFile::new(format!("{}/index.html", assets.display()))),
  );
  if cfg!(debug_assertions) {
    router = router.layer(livereload);
  }
  Ok(router.layer((
    TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
    CatchPanicLayer::new(),
    TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_mins(1)),
  )))
}

mod handlers;
mod models;
mod router;
mod routers;
mod templates;
mod utils;
