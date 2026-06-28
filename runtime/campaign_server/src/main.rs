use std::net::SocketAddr;

use anyhow::Context;
use campaign_server::config::ServerConfig;
use campaign_server::storage::database::connect_database;
use campaign_server::web::router::build_router;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .json()
        .init();

    let config = ServerConfig::from_environment()?;
    let pool = connect_database(&config).await?;
    let app = build_router(config.clone(), pool).await?;
    let address: SocketAddr = config
        .listen_address
        .parse()
        .with_context(|| format!("invalid listen address {}", config.listen_address))?;
    let listener = TcpListener::bind(address)
        .await
        .with_context(|| format!("failed to bind {}", config.listen_address))?;

    tracing::info!(listen_address = %config.listen_address, "campaign server listening");
    axum::serve(listener, app)
        .await
        .context("campaign server failed")?;
    Ok(())
}
