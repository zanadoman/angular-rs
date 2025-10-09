use axum_login::{
    axum::{Router, routing},
    tracing::Level,
};

use crate::{AppState, Authenticator, handlers::authentication};

#[tracing::instrument(level = Level::TRACE)]
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/logout", routing::post(authentication::logout))
        .route_layer(axum_login::login_required!(Authenticator))
        .route("/login", routing::post(authentication::login))
        .route("/register", routing::post(authentication::register))
}
