use std::time::Duration;

use update_agent::core::agent::{UpdateAgent, UpdateAgentClients};
use update_agent::core::clients::{
    FilesystemReleaseInstaller, GithubReleaseSource, HttpHealthProbe, NginxProxySwitcher,
    SystemdSlotSupervisor,
};
use update_agent::core::config::UpdateAgentConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .json()
        .init();

    let config = UpdateAgentConfig::from_environment()?;
    let once = std::env::var("ABE_UPDATE_AGENT_ONCE")
        .map(|value| matches!(value.as_str(), "1" | "true" | "yes"))
        .unwrap_or(false);
    let clients = UpdateAgentClients {
        release_source: Box::new(GithubReleaseSource::new(config.github_token.as_deref())?),
        installer: Box::new(FilesystemReleaseInstaller),
        supervisor: Box::new(SystemdSlotSupervisor),
        proxy: Box::new(NginxProxySwitcher),
        health: Box::new(HttpHealthProbe::new()),
    };
    let agent = UpdateAgent::new(config.clone(), clients);

    loop {
        if let Err(error) = agent.run_once().await {
            tracing::error!(error = %error, "update request failed");
        }
        if once {
            break;
        }
        tokio::time::sleep(Duration::from_secs(config.poll_seconds)).await;
    }

    Ok(())
}
