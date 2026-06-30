use clap::{ArgAction, Parser};

#[derive(Clone, Debug, Parser)]
#[command(
    name = "abe-fake-affiliate-network",
    about = "Local browser fake CPA affiliate network for Ad Buy Engine testing"
)]
pub struct Cli {
    #[arg(
        long,
        default_value = "127.0.0.1:8090",
        help = "Loopback listen address for the fake network dashboard"
    )]
    pub listen_address: String,

    #[arg(
        long,
        default_value = "http://127.0.0.1:8088/postback?cid={click_id}&type={event_type}&payout={payout}&currency={currency}&status={status}&txid={transaction_id}",
        help = "Ad Buy Engine postback URL template with fake network macros"
    )]
    pub postback_template: String,

    #[arg(
        long,
        default_value_t = 10,
        help = "Current-run lead conversion threshold"
    )]
    pub lead_threshold: u32,

    #[arg(
        long,
        default_value_t = 100,
        help = "Current-run sale conversion threshold"
    )]
    pub sale_threshold: u32,

    #[arg(
        long,
        default_value_t = 5,
        help = "Outbound postback request timeout in seconds"
    )]
    pub request_timeout_seconds: u64,

    #[arg(long, value_delimiter = ',', action = ArgAction::Append, help = "Allow an otherwise blocked postback host")]
    pub allow_host: Vec<String>,

    #[arg(long, action = ArgAction::SetTrue, help = "Allow RFC1918/link-local postback targets")]
    pub allow_private_network: bool,
}
