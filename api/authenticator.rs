use std::fmt::{self, Debug, Formatter};

use axum_login::{
    AuthManagerLayer, AuthManagerLayerBuilder, AuthnBackend, UserId,
    tower_sessions::{Expiry, SessionManagerLayer},
};
use serde::Deserialize;
use time::Duration;
use tower_sessions_sqlx_store::{MySqlStore, sqlx::MySqlPool};

use crate::models::User;

#[derive(Deserialize)]
pub struct Credentials {
    name: String,
    password: String,
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("name", &self.name)
            .field("password", &"********")
            .finish()
    }
}

#[derive(Clone)]
pub struct Authenticator(MySqlPool);

impl Authenticator {
    #[tracing::instrument(level = "trace")]
    pub fn new(
        pool: MySqlPool,
        store: MySqlStore,
        expiry: i64,
    ) -> Result<AuthManagerLayer<Self, MySqlStore>, sqlx::Error> {
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

    #[tracing::instrument(level = "trace", skip(self))]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.get_user(&creds.name).await?.filter(|user| {
            password_auth::verify_password(creds.password, &user.password)
                .is_ok()
        }))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Self::User::where_name(&self.0, user_id).await
    }
}
