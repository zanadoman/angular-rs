use std::sync::Arc;

use notify::RecommendedWatcher;
use tower_sessions_sqlx_store::sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    _watcher: Arc<RecommendedWatcher>,
    pool: PgPool,
}

impl AppState {
    #[tracing::instrument(level = "trace")]
    pub fn new(watcher: RecommendedWatcher, pool: PgPool) -> Self {
        Self {
            _watcher: watcher.into(),
            pool,
        }
    }

    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }
}
