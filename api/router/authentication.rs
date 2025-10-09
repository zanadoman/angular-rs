use axum_login::tracing::Level;
use utoipa_axum::router::OpenApiRouter;

use crate::{AppState, Authenticator, handlers::authentication};

#[tracing::instrument(level = Level::TRACE)]
pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(utoipa_axum::routes!(authentication::logout))
        .route_layer(axum_login::login_required!(Authenticator))
        .routes(utoipa_axum::routes!(authentication::login))
        .routes(utoipa_axum::routes!(authentication::register))
}
