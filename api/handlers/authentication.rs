use axum_login::{
    AuthSession,
    axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
    tracing::Level,
};

use crate::{AppState, Authenticator, Credentials, HandlerError, models::User};

#[utoipa::path(
    post,
    path = "/register",
    tag = module_path!(),
    request_body = User,
    responses((status = CREATED, body = User))
)]
#[axum::debug_handler]
#[tracing::instrument(level = Level::DEBUG, skip(state), err)]
pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<User>,
) -> Result<impl IntoResponse, HandlerError> {
    let mut tx = state.pool().begin().await?;
    user.create(&mut tx).await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    post,
    path = "/login",
    tag = module_path!(),
    request_body = Credentials,
    responses((status = OK, body = User))
)]
#[axum::debug_handler]
#[tracing::instrument(level = Level::DEBUG, skip(auth), err)]
pub async fn login(
    mut auth: AuthSession<Authenticator>,
    Json(creds): Json<Credentials>,
) -> Result<impl IntoResponse, HandlerError> {
    if let Some(user) = auth.authenticate(creds).await? {
        auth.login(&user).await?;
        Ok((StatusCode::OK, Json(user)))
    } else {
        auth.logout().await?;
        Err(StatusCode::UNAUTHORIZED.into())
    }
}

#[utoipa::path(
    post,
    path = "/logout",
    tag = module_path!(),
    responses((status = NO_CONTENT))
)]
#[axum::debug_handler]
#[tracing::instrument(level = Level::DEBUG, skip(auth), err)]
pub async fn logout(
    mut auth: AuthSession<Authenticator>,
) -> Result<impl IntoResponse, HandlerError> {
    auth.logout().await?;
    Ok(StatusCode::NO_CONTENT)
}
