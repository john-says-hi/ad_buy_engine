use std::net::SocketAddr;
use std::time::Duration;

use thiserror::Error;
use url::Url;

use crate::cli::Cli;
use crate::macros::{PostbackMacros, render_postback_url};
use crate::safety::{SafetyError, SafetyPolicy};

pub const DEFAULT_LISTEN_ADDRESS: &str = "127.0.0.1:8090";
pub const DEFAULT_REQUEST_TIMEOUT_SECONDS: u64 = 5;
pub const MIN_THRESHOLD: u32 = 1;
pub const MAX_THRESHOLD: u32 = 100_000;
pub const MAX_REQUEST_TIMEOUT_SECONDS: u64 = 60;

#[derive(Clone, Debug)]
pub struct RunConfig {
    pub listen_address: SocketAddr,
    pub postback_template: String,
    pub lead_threshold: u32,
    pub sale_threshold: u32,
    pub request_timeout: Duration,
    pub safety_policy: SafetyPolicy,
}

impl RunConfig {
    pub fn dashboard_base_url(&self) -> String {
        format!("http://{}", self.listen_address)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("listen address must be a socket address like 127.0.0.1:8090: {0}")]
    InvalidListenAddress(String),
    #[error("listen address must be loopback unless --allow-private-network is set")]
    UnsafeListenAddress,
    #[error("{field} must be between {min} and {max}, got {actual}")]
    NumericRange {
        field: &'static str,
        min: String,
        max: String,
        actual: String,
    },
    #[error("postback template is invalid: {0}")]
    InvalidPostbackTemplate(String),
    #[error("postback template target is not allowed: {0}")]
    UnsafePostbackTemplate(#[from] SafetyError),
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
        validate_u32(
            "lead-threshold",
            cli.lead_threshold,
            MIN_THRESHOLD,
            MAX_THRESHOLD,
        )?;
        validate_u32(
            "sale-threshold",
            cli.sale_threshold,
            MIN_THRESHOLD,
            MAX_THRESHOLD,
        )?;
        validate_u64(
            "request-timeout-seconds",
            cli.request_timeout_seconds,
            1,
            MAX_REQUEST_TIMEOUT_SECONDS,
        )?;

        let safety_policy = SafetyPolicy::new(cli.allow_host, cli.allow_private_network);
        let rendered = validate_postback_template(&cli.postback_template)?;
        safety_policy.ensure_url_allowed(&rendered)?;

        Ok(Self {
            listen_address,
            postback_template: cli.postback_template,
            lead_threshold: cli.lead_threshold,
            sale_threshold: cli.sale_threshold,
            request_timeout: Duration::from_secs(cli.request_timeout_seconds),
            safety_policy,
        })
    }
}

pub fn validate_postback_template(template: &str) -> Result<Url, ConfigError> {
    let sample = PostbackMacros {
        click_id: "sample-click",
        event_type: "Lead",
        payout: "1.00",
        currency: "USD",
        status: "approved",
        transaction_id: "sample-transaction",
    };
    render_postback_url(template, &sample)
        .map_err(|error| ConfigError::InvalidPostbackTemplate(error.to_string()))
}

fn validate_u32(field: &'static str, value: u32, min: u32, max: u32) -> Result<(), ConfigError> {
    if value < min || value > max {
        return Err(ConfigError::NumericRange {
            field,
            min: min.to_string(),
            max: max.to_string(),
            actual: value.to_string(),
        });
    }
    Ok(())
}

fn validate_u64(field: &'static str, value: u64, min: u64, max: u64) -> Result<(), ConfigError> {
    if value < min || value > max {
        return Err(ConfigError::NumericRange {
            field,
            min: min.to_string(),
            max: max.to_string(),
            actual: value.to_string(),
        });
    }
    Ok(())
}
