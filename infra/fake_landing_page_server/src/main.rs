use clap::Parser;
use fake_landing_page_server::cli::Cli;
use fake_landing_page_server::config::RunConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = RunConfig::try_from(cli)?;
    println!("Fake Landing Page Server: {}", config.server_base_url());
    fake_landing_page_server::run(config).await?;
    Ok(())
}
