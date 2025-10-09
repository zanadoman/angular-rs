use axum_login::tracing::Level;
use utoipa_axum::router::OpenApiRouter;

use crate::{AppState, routers::authentication};

#[tracing::instrument(level = Level::TRACE)]
pub fn router() -> OpenApiRouter<AppState> {
  OpenApiRouter::new().merge(authentication::router())
}
