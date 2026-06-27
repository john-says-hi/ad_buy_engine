use anyhow::Context;
use click_server::app_state::AppState;
use click_server::config::ClickServerConfig;
use click_server::database::{connect_database, run_migrations};
use click_server::http::router::build_router;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("click_server=info,tower_http=info,axum=info")),
        )
        .init();

    let config = ClickServerConfig::from_env()?;
    let pool = connect_database(&config).await?;
    run_migrations(&pool).await?;

    let bind_addr = config.bind_addr.clone();
    let app = build_router(AppState::new(pool, config));
    let listener = TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("failed to bind click server to {bind_addr}"))?;

    tracing::info!(%bind_addr, "click server listening");
    axum::serve(listener, app)
        .await
        .context("click server stopped unexpectedly")?;

    Ok(())
}
