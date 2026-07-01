use clap::{ArgAction, Parser};

#[derive(Clone, Debug, Parser)]
#[command(
    name = "abe-fake-landing-page-server",
    about = "Local fake landing page server for Ad Buy Engine routing tests"
)]
pub struct Cli {
    #[arg(
        long,
        default_value = "127.0.0.1:8091",
        help = "Loopback listen address for the fake landing page server"
    )]
    pub listen_address: String,

    #[arg(long, value_delimiter = ',', action = ArgAction::Append, help = "Allow an otherwise blocked continuation host")]
    pub allow_host: Vec<String>,

    #[arg(long, action = ArgAction::SetTrue, help = "Allow RFC1918/link-local continuation targets")]
    pub allow_private_network: bool,
}
