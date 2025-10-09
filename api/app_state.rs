use std::sync::Arc;

use axum_login::tracing::Level;
use notify::RecommendedWatcher;
use tower_sessions_sqlx_store::sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    _watcher: Arc<RecommendedWatcher>,
}

impl AppState {
    #[tracing::instrument(level = Level::TRACE)]
    pub fn new(pool: PgPool, watcher: RecommendedWatcher) -> Self {
        Self {
            pool,
            _watcher: watcher.into(),
        }
    }

    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }
}
