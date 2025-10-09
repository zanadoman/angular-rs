#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::{
    env,
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};

use axum_login::{tower_sessions::ExpiredDeletion, tracing::Level};
use tokio::{net::TcpListener, signal, task, time::Duration};
use tower_sessions_sqlx_store::{PostgresStore, sqlx::PgPool};
use tracing_appender::rolling;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
#[tracing::instrument(level = Level::INFO, err)]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv()?;
    let (writer, _guard) =
        tracing_appender::non_blocking(rolling::daily("logs", "log"));
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(writer).with_ansi(false))
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(EnvFilter::try_from_default_env()?)
        .init();
    serve().await?;
    Ok(())
}

#[tracing::instrument(level = Level::INFO, err)]
async fn serve() -> Result<(), Box<dyn Error + Send + Sync>> {
    let port = env::var("APP_PORT").map_or(Ok(8080), |p| p.parse())?;
    let listener = TcpListener::bind(SocketAddr::from((
        env::var("APP_ADDRESS")?.parse::<Ipv4Addr>()?,
        port,
    )))
    .await?;
    tracing::info!("{listener:?}");
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    tracing::info!("{:?}", pool.connect_options());
    let store = PostgresStore::new(pool.clone())
        .with_schema_name("public")?
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
    task.await.ok().transpose()?;
    Ok(())
}
