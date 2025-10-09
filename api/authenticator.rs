use std::fmt::{self, Debug, Formatter};

use axum_login::{
    AuthManagerLayer, AuthManagerLayerBuilder, AuthnBackend, UserId,
    tower_sessions::{Expiry, SessionManagerLayer},
    tracing::Level,
};
use serde::Deserialize;
use time::Duration;
use tower_sessions_sqlx_store::{PostgresStore, sqlx::PgPool};
use utoipa::ToSchema;

use crate::models::User;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    #[schema(write_only)]
    pub username: String,
    #[schema(write_only)]
    pub password: String,
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &"********")
            .finish()
    }
}

#[derive(Clone)]
pub struct Authenticator(PgPool);

impl Authenticator {
    #[tracing::instrument(level = Level::TRACE, err)]
    pub fn new(
        pool: PgPool,
        store: PostgresStore,
        expiry: i64,
    ) -> Result<AuthManagerLayer<Self, PostgresStore>, sqlx::Error> {
        Ok(AuthManagerLayerBuilder::new(
            Self(pool),
            SessionManagerLayer::new(store)
                .with_expiry(if 0 < expiry {
                    Expiry::OnInactivity(Duration::hours(expiry))
                } else {
                    Expiry::OnSessionEnd
                })
                .with_secure(!cfg!(debug_assertions)),
        )
        .build())
    }
}

impl AuthnBackend for Authenticator {
    type Credentials = Credentials;
    type Error = sqlx::Error;
    type User = User;

    #[tracing::instrument(level = Level::TRACE, skip(self), err)]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.get_user(&creds.username).await?.filter(|u| {
            password_auth::verify_password(creds.password, &u.password).is_ok()
        }))
    }

    #[tracing::instrument(level = Level::TRACE, skip(self), err)]
    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Self::User::where_username(&self.0, user_id, false).await
    }
}
