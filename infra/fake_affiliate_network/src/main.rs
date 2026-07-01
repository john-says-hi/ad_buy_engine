use clap::Parser;
use fake_affiliate_network::cli::Cli;
use fake_affiliate_network::config::RunConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = RunConfig::try_from(cli)?;
    println!(
        "Fake Affiliate Network dashboard: {}",
        config.dashboard_base_url()
    );
    fake_affiliate_network::run(config).await?;
    Ok(())
}
