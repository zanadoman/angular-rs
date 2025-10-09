use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use axum_login::axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_sessions_sqlx_store::sqlx::error::ErrorKind;

use crate::{Authenticator, FieldError};

#[derive(Debug)]
pub enum HandlerError {
    Any(StatusCode),
    Validation(HashMap<&'static str, FieldError>),
    Violation(&'static str),
}

impl From<StatusCode> for HandlerError {
    fn from(value: StatusCode) -> Self {
        Self::Any(value)
    }
}

impl From<axum_login::Error<Authenticator>> for HandlerError {
    fn from(_: axum_login::Error<Authenticator>) -> Self {
        Self::Any(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<(&'static str, FieldError)> for HandlerError {
    fn from(value: (&'static str, FieldError)) -> Self {
        Self::Validation([value].into())
    }
}

impl From<Vec<(&'static str, FieldError)>> for HandlerError {
    fn from(value: Vec<(&'static str, FieldError)>) -> Self {
        Self::Validation(value.into_iter().collect())
    }
}

impl From<sqlx::Error> for HandlerError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(err) => match err.kind() {
                ErrorKind::UniqueViolation => Self::Violation("unique"),
                ErrorKind::ForeignKeyViolation => Self::Violation("key"),
                ErrorKind::NotNullViolation => Self::Violation("null"),
                ErrorKind::CheckViolation => Self::Violation("check"),
                _ => Self::Violation("other"),
            },
            sqlx::Error::RowNotFound => Self::Any(StatusCode::NOT_FOUND),
            _ => Self::Any(StatusCode::SERVICE_UNAVAILABLE),
        }
    }
}

impl Display for HandlerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            Self::Any(err) => err.into_response(),
            Self::Validation(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({"validation": err})),
            )
                .into_response(),
            Self::Violation(err) => (
                StatusCode::CONFLICT,
                Json(serde_json::json!({"violation": err})),
            )
                .into_response(),
        }
    }
}

pub mod authentication;
