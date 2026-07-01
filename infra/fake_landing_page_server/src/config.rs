use std::net::SocketAddr;

use thiserror::Error;

use crate::cli::Cli;
use crate::safety::SafetyPolicy;

pub const DEFAULT_LISTEN_ADDRESS: &str = "127.0.0.1:8091";

#[derive(Clone, Debug)]
pub struct RunConfig {
    pub listen_address: SocketAddr,
    pub safety_policy: SafetyPolicy,
}

impl RunConfig {
    pub fn server_base_url(&self) -> String {
        format!("http://{}", self.listen_address)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("listen address must be a socket address like 127.0.0.1:8091: {0}")]
    InvalidListenAddress(String),
    #[error("listen address must be loopback unless --allow-private-network is set")]
    UnsafeListenAddress,
}

impl TryFrom<Cli> for RunConfig {
    type Error = ConfigError;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        let listen_address = cli
            .listen_address
            .parse::<SocketAddr>()
            .map_err(|error| ConfigError::InvalidListenAddress(error.to_string()))?;
        if !listen_address.ip().is_loopback() && !cli.allow_private_network {
            return Err(ConfigError::UnsafeListenAddress);
        }
        Ok(Self {
            listen_address,
            safety_policy: SafetyPolicy::new(cli.allow_host, cli.allow_private_network),
        })
    }
}
