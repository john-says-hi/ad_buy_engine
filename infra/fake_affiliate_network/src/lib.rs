pub mod cli;
pub mod config;
pub mod conversions;
pub mod macros;
pub mod postback;
pub mod render;
pub mod routes;
pub mod safety;
pub mod state;

use axum::Router;
use config::RunConfig;
use thiserror::Error;
use tokio::net::TcpListener;

#[derive(Debug, Error)]
pub enum FakeAffiliateNetworkError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn build_router(config: RunConfig) -> Result<Router, reqwest::Error> {
    routes::build_router(config)
}

pub async fn run(config: RunConfig) -> Result<(), FakeAffiliateNetworkError> {
    let listen_address = config.listen_address;
    let router = build_router(config)?;
    let listener = TcpListener::bind(listen_address).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
