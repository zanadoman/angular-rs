use std::fmt::{self, Debug, Formatter};

use axum_login::{AuthUser, tracing::Level};
use serde::{Deserialize, Serialize};
use tower_sessions_sqlx_store::sqlx::PgPool;

use crate::{FieldError, HandlerError};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl User {
    #[tracing::instrument(level = Level::TRACE, skip(pool))]
    pub async fn where_username(
        pool: &PgPool,
        name: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"SELECT * FROM "users" WHERE "username" = $1 LIMIT 1"#,
            name
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
        } else if Self::where_username(pool, &self.username).await?.is_some() {
            errs.push(invalid_field!(self, username, FieldError::Duplicate));
        }
        if self.password.len() < 8 {
            errs.push(invalid_field!(self, password, FieldError::TooShort));
        }
        errs.is_empty().ok_or(errs)?;
        *self = sqlx::query_as!(
            Self,
            r#"INSERT INTO "users" ("username", "password") VALUES ($1, $2) RETURNING *"#,
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
