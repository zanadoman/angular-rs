use std::fmt::{self, Debug, Formatter};

use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use tower_sessions_sqlx_store::sqlx::MySqlPool;

use crate::{FieldError, HandlerError, InvalidField};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl User {
    #[tracing::instrument(level = "trace", skip(pool))]
    pub async fn where_name(
        pool: &MySqlPool,
        name: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"SELECT * FROM `user` WHERE `name` = ? LIMIT 1;"#,
            name
        )
        .fetch_optional(pool)
        .await
    }

    #[tracing::instrument(level = "trace", skip(pool))]
    pub async fn create(
        &mut self,
        pool: &MySqlPool,
    ) -> Result<(), HandlerError> {
        let mut errs = Vec::new();
        if self.name.len() < 3 {
            errs.push(invalid_field!(self, name, FieldError::TooShort));
        } else if 50 < self.name.len() {
            errs.push(invalid_field!(self, name, FieldError::TooLong));
        } else if Self::where_name(pool, &self.name).await?.is_some() {
            errs.push(invalid_field!(self, name, FieldError::Duplicate));
        }
        if self.password.len() < 8 {
            errs.push(invalid_field!(self, password, FieldError::TooShort));
        }
        if !errs.is_empty() {
            return Err(errs.into());
        }
        self.id = Some(
            sqlx::query!(
                r#"INSERT INTO `user` (`name`, `password`) VALUES (?, ?);"#,
                self.name,
                password_auth::generate_hash(&self.password)
            )
            .execute(pool)
            .await?
            .last_insert_id()
            .try_into()?,
        );
        Ok(())
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("password", &"********")
            .finish()
    }
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.name.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}
