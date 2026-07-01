use clap::Parser;
use fake_landing_page_server::cli::Cli;
use fake_landing_page_server::config::RunConfig;

fn config_from(args: &[&str]) -> Result<RunConfig, Box<dyn std::error::Error>> {
    let cli = Cli::try_parse_from(
        std::iter::once("abe-fake-landing-page-server").chain(args.iter().copied()),
    )?;
    Ok(RunConfig::try_from(cli)?)
}

#[test]
fn config_defaults_to_loopback_fake_landing_page_server() -> Result<(), Box<dyn std::error::Error>>
{
    let config = config_from(&[])?;

    assert_eq!(config.listen_address.to_string(), "127.0.0.1:8091");
    assert_eq!(config.server_base_url(), "http://127.0.0.1:8091");
    assert!(config.safety_policy.summary().default_loopback_only);
    Ok(())
}

#[test]
fn invalid_or_unsafe_listen_addresses_are_rejected() {
    assert!(config_from(&["--listen-address", "not-a-socket"]).is_err());
    assert!(config_from(&["--listen-address", "0.0.0.0:8091"]).is_err());
    assert!(
        config_from(&[
            "--listen-address",
            "0.0.0.0:8091",
            "--allow-private-network"
        ])
        .is_ok()
    );
}

#[test]
fn allowlisted_hosts_are_reflected_in_safety_summary() -> Result<(), Box<dyn std::error::Error>> {
    let config = config_from(&["--allow-host", "staging.example,LOCALHOST:8088"])?;
    let summary = config.safety_policy.summary();

    assert_eq!(
        summary.allow_hosts,
        vec!["staging.example".to_string(), "localhost:8088".to_string()]
    );
    assert!(!summary.default_loopback_only);
    Ok(())
}
