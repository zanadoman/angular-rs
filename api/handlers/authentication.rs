use askama::Template;
use axum_login::{
  AuthSession,
  axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
  tracing::Level,
};
use lettre::{
  AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
  message::{Attachment, MultiPart, SinglePart},
};
use time::OffsetDateTime;
use tokio::fs;

use crate::{AppError, AppState, Authenticator, Credentials, models::User, templates::ReportMail};

#[utoipa::path(
    post,
    path = "/register",
    tag = module_path!().split("::").last().unwrap(),
    request_body = User,
    responses((status = CREATED, body = User))
)]
#[axum::debug_handler]
#[tracing::instrument(level = Level::DEBUG, skip(state), err)]
pub async fn register(
  State(state): State<AppState>,
  Json(mut user): Json<User>,
) -> Result<impl IntoResponse, AppError> {
  let mut tx = state.pool().begin().await?;
  user.create(&mut tx).await?;
  tx.commit().await?;
  send_report(state.mailer(), &user.username, "registration").await?;
  Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    post,
    path = "/login",
    tag = module_path!().split("::").last().unwrap(),
    request_body = Credentials,
    responses((status = OK, body = User))
)]
#[axum::debug_handler]
#[tracing::instrument(level = Level::DEBUG, skip(state, auth), err)]
pub async fn login(
  State(state): State<AppState>,
  mut auth: AuthSession<Authenticator>,
  Json(creds): Json<Credentials>,
) -> Result<impl IntoResponse, AppError> {
  if let Some(user) = auth.authenticate(creds).await? {
    auth.login(&user).await?;
    send_report(state.mailer(), &user.username, "login").await?;
    Ok((StatusCode::OK, Json(user)))
  } else {
    auth.logout().await?;
    Err(StatusCode::UNAUTHORIZED.into())
  }
}

#[utoipa::path(
    post,
    path = "/logout",
    tag = module_path!().split("::").last().unwrap(),
    responses((status = NO_CONTENT))
)]
#[axum::debug_handler]
#[tracing::instrument(level = Level::DEBUG, skip(state, auth), err)]
pub async fn logout(
  State(state): State<AppState>,
  mut auth: AuthSession<Authenticator>,
) -> Result<impl IntoResponse, AppError> {
  let username = auth
    .user
    .as_ref()
    .ok_or(StatusCode::UNAUTHORIZED)?
    .username
    .clone();
  auth.logout().await?;
  send_report(state.mailer(), &username, "logout").await?;
  Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(level = Level::TRACE, skip(mailer), err)]
async fn send_report(
  mailer: &AsyncSmtpTransport<Tokio1Executor>,
  username: &str,
  action: &str,
) -> Result<(), AppError> {
  mailer
    .send(
      Message::builder()
        .from("angular-rs <no-reply@angular-rs.org>".parse()?)
        .to(format!("{username} <{username}@example.org>").parse()?)
        .subject(format!("angular-rs {action}"))
        .multipart(
          MultiPart::related()
            .singlepart(SinglePart::html(
              ReportMail {
                username,
                action,
                timestamp: &OffsetDateTime::now_utc(),
              }
              .render()?,
            ))
            .singlepart(Attachment::new_inline("favicon.ico".to_owned()).body(
              fs::read("public/favicon.ico").await?,
              "image/vnd.microsoft.icon".parse()?,
            )),
        )?,
    )
    .await?;
  Ok(())
}
