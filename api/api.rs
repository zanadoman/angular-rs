#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::{
    env,
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};

use axum_login::tower_sessions::ExpiredDeletion;
use tokio::{net::TcpListener, signal, task, time::Duration};
use tower_sessions_sqlx_store::{MySqlStore, sqlx::MySqlPool};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    tracing_subscriber::registry()
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(EnvFilter::try_from_default_env()?)
        .init();
    let port = env::var("APP_PORT").map_or(Ok(8080), |p| p.parse())?;
    let listener = TcpListener::bind(SocketAddr::from((
        if cfg!(debug_assertions) {
            Ipv4Addr::LOCALHOST
        } else {
            Ipv4Addr::UNSPECIFIED
        },
        port,
    )))
    .await?;
    tracing::info!("{listener:?}");
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    let store = MySqlStore::new(pool.clone())
        .with_schema_name(pool.connect_options().get_database().unwrap())?
        .with_table_name("_tower_sessions")?;
    store.migrate().await?;
    let task = task::spawn(
        store
            .clone()
            .continuously_delete_expired(Duration::from_hours(1)),
    );
    let handle = task.abort_handle();
    axum::serve(
        listener,
        angular_rs::new(
            pool,
            store,
            env::var("SESSION_EXPIRY").map_or(Ok(0), |e| e.parse())?,
            "dist/angular-rs/browser",
        )?,
    )
    .with_graceful_shutdown(async move {
        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .unwrap()
                .recv()
                .await
        };
        #[cfg(not(unix))]
        let terminate = std::future::pending();
        tokio::select! {
            () = async { signal::ctrl_c().await.unwrap() } => {}
            _ = terminate => {}
        }
        handle.abort();
    })
    .await?;
    task.await??;
    Ok(())
}
