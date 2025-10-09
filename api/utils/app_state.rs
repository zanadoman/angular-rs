use std::sync::Arc;

use axum_login::tracing::Level;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use notify::RecommendedWatcher;
use tower_sessions_sqlx_store::sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
  pool: PgPool,
  mailer: AsyncSmtpTransport<Tokio1Executor>,
  _watcher: Arc<RecommendedWatcher>,
}

impl AppState {
  #[tracing::instrument(level = Level::TRACE)]
  pub fn new(
    pool: PgPool,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    watcher: RecommendedWatcher,
  ) -> Self {
    Self {
      pool,
      mailer,
      _watcher: watcher.into(),
    }
  }

  pub const fn pool(&self) -> &PgPool {
    &self.pool
  }

  pub const fn mailer(&self) -> &AsyncSmtpTransport<Tokio1Executor> {
    &self.mailer
  }
}
