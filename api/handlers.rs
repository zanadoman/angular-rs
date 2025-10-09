use std::num::TryFromIntError;

use axum_login::axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_sessions_sqlx_store::sqlx::error::ErrorKind;

use crate::{Authenticator, InvalidField};

pub enum HandlerError {
    AnyError(StatusCode),
    AuthenticatorError,
    DatabaseConflict(&'static str),
    DatabaseError,
    DatabaseNotFound,
    InvalidFields(Vec<InvalidField>),
    TryFromError,
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            Self::AnyError(err) => err.into_response(),
            Self::DatabaseConflict(err) => (
                StatusCode::CONFLICT,
                Json(serde_json::json!({"violation": err})),
            )
                .into_response(),
            Self::DatabaseNotFound => StatusCode::NOT_FOUND.into_response(),
            Self::InvalidFields(errs) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"errors": errs})),
            )
                .into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl From<StatusCode> for HandlerError {
    fn from(value: StatusCode) -> Self {
        Self::AnyError(value)
    }
}

impl From<axum_login::Error<Authenticator>> for HandlerError {
    fn from(_: axum_login::Error<Authenticator>) -> Self {
        Self::AuthenticatorError
    }
}

impl From<sqlx::Error> for HandlerError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(err) => match err.kind() {
                ErrorKind::UniqueViolation => Self::DatabaseConflict("unique"),
                ErrorKind::ForeignKeyViolation => Self::DatabaseConflict("key"),
                ErrorKind::NotNullViolation => Self::DatabaseConflict("null"),
                ErrorKind::CheckViolation => Self::DatabaseConflict("check"),
                _ => Self::DatabaseConflict("other"),
            },
            sqlx::Error::RowNotFound => Self::DatabaseNotFound,
            _ => Self::DatabaseError,
        }
    }
}

impl From<InvalidField> for HandlerError {
    fn from(value: InvalidField) -> Self {
        Self::InvalidFields(vec![value])
    }
}

impl From<Vec<InvalidField>> for HandlerError {
    fn from(value: Vec<InvalidField>) -> Self {
        Self::InvalidFields(value)
    }
}

impl From<TryFromIntError> for HandlerError {
    fn from(_: TryFromIntError) -> Self {
        Self::TryFromError
    }
}

pub mod authentication;
