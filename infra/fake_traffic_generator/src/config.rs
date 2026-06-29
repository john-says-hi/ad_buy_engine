use std::fmt;
use std::time::Duration;

use clap::ValueEnum;
use serde::Serialize;
use thiserror::Error;
use url::Url;

use crate::cli::Cli;
use crate::safety::{SafetyError, SafetyPolicy};

pub const MAX_USERS: u32 = 1_000;
pub const MAX_SESSIONS_PER_USER: u32 = 1_000;
pub const MAX_TOTAL_SESSIONS: u64 = 50_000;
pub const MAX_DURATION_SECONDS: u64 = 3_600;
pub const MAX_INTERVAL_MS: u64 = 60_000;
pub const MAX_CONCURRENCY: usize = 256;
pub const MAX_REDIRECT_HOPS: usize = 8;
pub const REQUEST_TIMEOUT_SECONDS: u64 = 5;
pub const CONVERSION_DELAY_MS: u64 = 25;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Table,
    Json,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Table => formatter.write_str("table"),
            Self::Json => formatter.write_str("json"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ConversionType {
    Lead,
    Sale,
    None,
}

impl ConversionType {
    pub fn is_enabled(self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn postback_value(self) -> Option<&'static str> {
        match self {
            Self::Lead => Some("Lead"),
            Self::Sale => Some("Sale"),
            Self::None => None,
        }
    }
}

impl fmt::Display for ConversionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lead => formatter.write_str("lead"),
            Self::Sale => formatter.write_str("sale"),
            Self::None => formatter.write_str("none"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RunConfig {
    pub campaign_url: Url,
    pub users: u32,
    pub sessions_per_user: u32,
    pub duration_limit: Option<Duration>,
    pub interval: Duration,
    pub jitter_percent: u8,
    pub concurrency: usize,
    pub seed: u64,
    pub conversion_rate: f64,
    pub conversion_type: ConversionType,
    pub output_format: OutputFormat,
    pub dry_run: bool,
    pub safety_policy: SafetyPolicy,
    pub max_redirect_hops: usize,
    pub request_timeout: Duration,
    pub conversion_delay: Duration,
}

impl RunConfig {
    pub fn total_requested_sessions(&self) -> u64 {
        u64::from(self.users) * u64::from(self.sessions_per_user)
    }

    pub fn conversions_enabled(&self) -> bool {
        self.conversion_type.is_enabled() && self.conversion_rate > 0.0
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("campaign URL must be a valid HTTP or HTTPS URL: {0}")]
    InvalidCampaignUrl(String),
    #[error("campaign URL is not allowed: {0}")]
    UnsafeCampaignUrl(#[from] SafetyError),
    #[error("{field} must be between {min} and {max}, got {actual}")]
    NumericRange {
        field: &'static str,
        min: String,
        max: String,
        actual: String,
    },
    #[error("users * sessions must be at most {max}, got {actual}")]
    TooManySessions { max: u64, actual: u64 },
    #[error("--conversion-rate requires --conversion-type lead or sale")]
    ConversionRateWithoutType,
}

impl TryFrom<Cli> for RunConfig {
    type Error = ConfigError;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        let campaign_url = Url::parse(&cli.campaign_url)
            .map_err(|error| ConfigError::InvalidCampaignUrl(error.to_string()))?;
        if !matches!(campaign_url.scheme(), "http" | "https") {
            return Err(ConfigError::InvalidCampaignUrl(
                "scheme must be http or https".to_string(),
            ));
        }

        validate_u32("users", cli.users, 1, MAX_USERS)?;
        validate_u32("sessions", cli.sessions, 1, MAX_SESSIONS_PER_USER)?;
        validate_u64(
            "duration-seconds",
            cli.duration_seconds,
            0,
            MAX_DURATION_SECONDS,
        )?;
        validate_u64("interval-ms", cli.interval_ms, 0, MAX_INTERVAL_MS)?;
        validate_u8("jitter-percent", cli.jitter_percent, 0, 100)?;
        validate_usize("concurrency", cli.concurrency, 1, MAX_CONCURRENCY)?;
        validate_f64("conversion-rate", cli.conversion_rate, 0.0, 1.0)?;

        let total_sessions = u64::from(cli.users) * u64::from(cli.sessions);
        if total_sessions > MAX_TOTAL_SESSIONS {
            return Err(ConfigError::TooManySessions {
                max: MAX_TOTAL_SESSIONS,
                actual: total_sessions,
            });
        }

        if cli.conversion_rate > 0.0 && !cli.conversion_type.is_enabled() {
            return Err(ConfigError::ConversionRateWithoutType);
        }

        let safety_policy = SafetyPolicy::new(cli.allow_host, cli.allow_private_network);
        safety_policy.ensure_url_allowed(&campaign_url)?;

        Ok(Self {
            campaign_url,
            users: cli.users,
            sessions_per_user: cli.sessions,
            duration_limit: duration_limit(cli.duration_seconds),
            interval: Duration::from_millis(cli.interval_ms),
            jitter_percent: cli.jitter_percent,
            concurrency: cli.concurrency,
            seed: cli.seed,
            conversion_rate: cli.conversion_rate,
            conversion_type: cli.conversion_type,
            output_format: cli.output,
            dry_run: cli.dry_run,
            safety_policy,
            max_redirect_hops: MAX_REDIRECT_HOPS,
            request_timeout: Duration::from_secs(REQUEST_TIMEOUT_SECONDS),
            conversion_delay: Duration::from_millis(CONVERSION_DELAY_MS),
        })
    }
}

fn duration_limit(seconds: u64) -> Option<Duration> {
    if seconds == 0 {
        None
    } else {
        Some(Duration::from_secs(seconds))
    }
}

fn validate_u32(field: &'static str, value: u32, min: u32, max: u32) -> Result<(), ConfigError> {
    if value < min || value > max {
        return Err(range_error(field, min, max, value));
    }
    Ok(())
}

fn validate_u64(field: &'static str, value: u64, min: u64, max: u64) -> Result<(), ConfigError> {
    if value < min || value > max {
        return Err(range_error(field, min, max, value));
    }
    Ok(())
}

fn validate_u8(field: &'static str, value: u8, min: u8, max: u8) -> Result<(), ConfigError> {
    if value < min || value > max {
        return Err(range_error(field, min, max, value));
    }
    Ok(())
}

fn validate_usize(
    field: &'static str,
    value: usize,
    min: usize,
    max: usize,
) -> Result<(), ConfigError> {
    if value < min || value > max {
        return Err(range_error(field, min, max, value));
    }
    Ok(())
}

fn validate_f64(field: &'static str, value: f64, min: f64, max: f64) -> Result<(), ConfigError> {
    if !value.is_finite() || value < min || value > max {
        return Err(range_error(field, min, max, value));
    }
    Ok(())
}

fn range_error<T: fmt::Display>(field: &'static str, min: T, max: T, actual: T) -> ConfigError {
    ConfigError::NumericRange {
        field,
        min: min.to_string(),
        max: max.to_string(),
        actual: actual.to_string(),
    }
}
