use axum_login::{
    AuthSession,
    axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
};

use crate::{AppState, Authenticator, Credentials, HandlerError, models::User};

#[axum::debug_handler]
#[tracing::instrument(level = "debug", skip(state))]
pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<User>,
) -> Result<impl IntoResponse, HandlerError> {
    user.create(state.pool()).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[axum::debug_handler]
#[tracing::instrument(level = "debug", skip(authenticator))]
pub async fn login(
    mut authenticator: AuthSession<Authenticator>,
    Json(creds): Json<Credentials>,
) -> Result<impl IntoResponse, HandlerError> {
    let user = authenticator
        .authenticate(creds)
        .await?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    authenticator.login(&user).await?;
    Ok((StatusCode::OK, Json(user)))
}

#[axum::debug_handler]
#[tracing::instrument(level = "debug", skip(authenticator))]
pub async fn logout(
    mut authenticator: AuthSession<Authenticator>,
) -> Result<impl IntoResponse, HandlerError> {
    authenticator.logout().await?;
    Ok(StatusCode::NO_CONTENT)
}
