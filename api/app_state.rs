use std::sync::Arc;

use notify::RecommendedWatcher;
use tower_sessions_sqlx_store::sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    _watcher: Arc<RecommendedWatcher>,
    pool: MySqlPool,
}

impl AppState {
    #[tracing::instrument(level = "trace")]
    pub fn new(watcher: RecommendedWatcher, pool: MySqlPool) -> Self {
        Self {
            _watcher: watcher.into(),
            pool,
        }
    }

    pub const fn pool(&self) -> &MySqlPool {
        &self.pool
    }
}
