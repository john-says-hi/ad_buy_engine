use clap::Parser;
use fake_traffic_generator::cli::Cli;
use fake_traffic_generator::config::RunConfig;
use fake_traffic_generator::output::format_summary;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = RunConfig::try_from(cli)?;
    let output_format = config.output_format;
    let summary = fake_traffic_generator::run(config).await?;
    println!("{}", format_summary(&summary, output_format)?);
    Ok(())
}
