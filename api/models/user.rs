use std::fmt::{self, Debug, Formatter};

use axum_login::{AuthUser, tracing::Level};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tower_sessions_sqlx_store::sqlx::PgPool;

use crate::{FieldError, HandlerError};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(
        default = "OffsetDateTime::now_utc",
        with = "time::serde::rfc3339"
    )]
    pub created_at: OffsetDateTime,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

impl User {
    #[tracing::instrument(level = Level::TRACE, skip(pool))]
    pub async fn where_username(
        pool: &PgPool,
        username: &str,
        deleted: bool,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
SELECT *
FROM "users"
WHERE ($1 OR "deleted_at" IS NULL) AND "username" = $2
LIMIT 1
            "#,
            deleted,
            username
        )
        .fetch_optional(pool)
        .await
    }

    #[tracing::instrument(level = Level::TRACE, skip(pool))]
    pub async fn create(&mut self, pool: &PgPool) -> Result<(), HandlerError> {
        let mut errs = Vec::new();
        if self.username.len() < 3 {
            errs.push(invalid_field!(self, username, FieldError::TooShort));
        } else if 50 < self.username.len() {
            errs.push(invalid_field!(self, username, FieldError::TooLong));
        } else if Self::where_username(pool, &self.username, true)
            .await?
            .is_some()
        {
            errs.push(invalid_field!(self, username, FieldError::Duplicate));
        }
        if self.password.len() < 8 {
            errs.push(invalid_field!(self, password, FieldError::TooShort));
        }
        errs.is_empty().ok_or(errs)?;
        *self = sqlx::query_as!(
            Self,
            r#"
INSERT INTO "users" ("username", "password", "created_at")
VALUES ($1, $2, now())
RETURNING *
            "#,
            self.username,
            password_auth::generate_hash(&self.password)
        )
        .fetch_one(pool)
        .await?;
        Ok(())
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"********")
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("deleted_at", &self.deleted_at)
            .finish()
    }
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.username.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}
