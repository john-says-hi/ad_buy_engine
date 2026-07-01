pub mod cli;
pub mod config;
pub mod render;
pub mod routes;
pub mod safety;

use axum::Router;
use config::RunConfig;
use thiserror::Error;
use tokio::net::TcpListener;

#[derive(Debug, Error)]
pub enum FakeLandingPageServerError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn build_router(config: RunConfig) -> Router {
    routes::build_router(config)
}

pub async fn run(config: RunConfig) -> Result<(), FakeLandingPageServerError> {
    let listen_address = config.listen_address;
    let router = build_router(config);
    let listener = TcpListener::bind(listen_address).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
