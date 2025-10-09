use std::fmt::{self, Debug, Formatter};

use axum_login::{AuthUser, tracing::Level};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tower_sessions_sqlx_store::sqlx::{Acquire, Postgres};
use utoipa::ToSchema;

use crate::{FieldError, HandlerError, models};

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(default)]
    #[schema(read_only)]
    pub id: i64,
    pub username: String,
    #[serde(serialize_with = "models::default_serializer")]
    #[schema(write_only)]
    pub password: String,
    #[serde(
        default = "OffsetDateTime::now_utc",
        with = "time::serde::rfc3339"
    )]
    #[schema(read_only)]
    pub created_at: OffsetDateTime,
    #[serde(default, with = "time::serde::rfc3339::option")]
    #[schema(read_only)]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(default, with = "time::serde::rfc3339::option")]
    #[schema(read_only)]
    pub deleted_at: Option<OffsetDateTime>,
}

impl User {
    #[tracing::instrument(level = Level::TRACE, skip(conn), err)]
    pub fn where_username<'a, 'c, A>(
        conn: A,
        username: &'a str,
        deleted: bool,
    ) -> impl Future<Output = Result<Option<Self>, sqlx::Error>> + Send + 'a
    where
        A: Acquire<'c, Database = Postgres> + Send + 'a,
    {
        async move {
            let mut conn = conn.acquire().await?;
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
            .fetch_optional(&mut *conn)
            .await
        }
    }

    #[tracing::instrument(level = Level::TRACE, skip(conn), err)]
    pub fn create<'a, 'c, A>(
        &'a mut self,
        conn: A,
    ) -> impl Future<Output = Result<(), HandlerError>> + Send + 'a
    where
        A: Acquire<'c, Database = Postgres> + Send + 'a,
    {
        async move {
            let mut conn = conn.acquire().await?;
            let mut errs = Vec::new();
            if self.username.len() < 3 {
                errs.push(invalid_field!(self, username, FieldError::TooShort));
            } else if 50 < self.username.len() {
                errs.push(invalid_field!(self, username, FieldError::TooLong));
            } else if Self::where_username(&mut *conn, &self.username, true)
                .await?
                .is_some()
            {
                errs.push(invalid_field!(
                    self,
                    username,
                    FieldError::Duplicate
                ));
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
            .fetch_one(&mut *conn)
            .await?;
            Ok(())
        }
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
