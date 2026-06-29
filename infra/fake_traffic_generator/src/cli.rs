use clap::{ArgAction, Parser};

use crate::config::{ConversionType, OutputFormat};

#[derive(Clone, Debug, Parser)]
#[command(
    name = "abe-fake-traffic",
    about = "Local-first HTTP traffic simulator for Ad Buy Engine campaign tracking"
)]
pub struct Cli {
    #[arg(long, value_name = "URL", help = "Campaign click URL to request")]
    pub campaign_url: String,

    #[arg(long, default_value_t = 1, help = "Number of virtual users")]
    pub users: u32,

    #[arg(long, default_value_t = 1, help = "Sessions to run per virtual user")]
    pub sessions: u32,

    #[arg(
        long,
        default_value_t = 0,
        help = "Maximum run duration; 0 means no duration cap"
    )]
    pub duration_seconds: u64,

    #[arg(
        long,
        default_value_t = 100,
        help = "Base delay between scheduled session starts"
    )]
    pub interval_ms: u64,

    #[arg(
        long,
        default_value_t = 20,
        help = "Percent jitter applied around the base interval"
    )]
    pub jitter_percent: u8,

    #[arg(long, default_value_t = 10, help = "Maximum simultaneous sessions")]
    pub concurrency: usize,

    #[arg(
        long,
        default_value_t = 1,
        help = "Deterministic seed for profiles and timing"
    )]
    pub seed: u64,

    #[arg(
        long,
        default_value_t = 0.0,
        help = "Chance that a session sends a conversion"
    )]
    pub conversion_rate: f64,

    #[arg(long, value_enum, default_value_t = ConversionType::None, help = "Conversion event type to emit")]
    pub conversion_type: ConversionType,

    #[arg(long, value_enum, default_value_t = OutputFormat::Table, help = "Summary output format")]
    pub output: OutputFormat,

    #[arg(long, action = ArgAction::SetTrue, help = "Validate the run without sending requests")]
    pub dry_run: bool,

    #[arg(long, value_delimiter = ',', action = ArgAction::Append, help = "Allow an otherwise blocked host")]
    pub allow_host: Vec<String>,

    #[arg(long, action = ArgAction::SetTrue, help = "Allow RFC1918/link-local IP targets")]
    pub allow_private_network: bool,
}
