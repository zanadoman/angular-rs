use std::{
  collections::HashMap,
  fmt::{self, Debug, Display, Formatter},
};

use axum_login::axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use tower_sessions_sqlx_store::sqlx::error::ErrorKind;

use crate::{Authenticator, Validation};

#[derive(Debug)]
pub enum AppError {
  Response(Response),
  Panic(Box<dyn std::error::Error>),
}

impl std::error::Error for AppError {}

impl From<StatusCode> for AppError {
  fn from(value: StatusCode) -> Self {
    Self::Response(value.into_response())
  }
}

impl From<serde_json::Error> for AppError {
  fn from(value: serde_json::Error) -> Self {
    Self::Response(
      (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(serde_json::json!({ "message": value.to_string() })),
      )
        .into_response(),
    )
  }
}

impl From<(&'static str, Validation)> for AppError {
  fn from(value: (&'static str, Validation)) -> Self {
    Self::Response(
      (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(serde_json::json!({ "validation": { value.0: value.1 } })),
      )
        .into_response(),
    )
  }
}

impl From<Vec<(&'static str, Validation)>> for AppError {
  fn from(value: Vec<(&'static str, Validation)>) -> Self {
    Self::Response(
      (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(serde_json::json!({
          "validation": value.into_iter().collect::<HashMap<&'static str, Validation>>()
        })),
      )
        .into_response(),
    )
  }
}

impl From<sqlx::Error> for AppError {
  fn from(value: sqlx::Error) -> Self {
    match value {
      sqlx::Error::Database(err) => Self::Response(
        (
          StatusCode::CONFLICT,
          Json(serde_json::json!({
            "validation": match err.kind() {
              ErrorKind::UniqueViolation => "unique",
              ErrorKind::ForeignKeyViolation => "key",
              ErrorKind::NotNullViolation => "null",
              ErrorKind::CheckViolation => "check",
              _ => "other",
            }
          })),
        )
          .into_response(),
      ),
      sqlx::Error::RowNotFound => Self::Response(StatusCode::NOT_FOUND.into_response()),
      _ => Self::Response(StatusCode::SERVICE_UNAVAILABLE.into_response()),
    }
  }
}

macro_rules! impl_panic_from {
  ($($err:ty),*$(,)?) => {
    $(
      impl From<$err> for AppError {
        fn from(value: $err) -> Self {
          Self::Panic(Box::new(value))
        }
      }
    )*
  };
}

impl_panic_from!(
  askama::Error,
  axum_login::Error<Authenticator>,
  lettre::address::AddressError,
  lettre::error::Error,
  lettre::message::header::ContentTypeErr,
  lettre::transport::smtp::Error,
  std::io::Error,
);

impl Display for AppError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{self:?}")
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    match self {
      Self::Response(err) => err,
      Self::Panic(err) => panic!("{err}"),
    }
  }
}
