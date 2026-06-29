use clap::Parser;
use fake_traffic_generator::cli::Cli;
use fake_traffic_generator::config::RunConfig;

fn config_from(args: &[&str]) -> Result<RunConfig, Box<dyn std::error::Error>> {
    let cli = Cli::try_parse_from(std::iter::once("abe-fake-traffic").chain(args.iter().copied()))?;
    Ok(RunConfig::try_from(cli)?)
}

#[test]
fn loopback_campaign_urls_are_allowed_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let config = config_from(&["--campaign-url", "http://127.0.0.1:8088/c/campaign-1"])?;

    assert_eq!(config.campaign_url.host_str(), Some("127.0.0.1"));
    assert!(config.safety_policy.summary().default_loopback_only);
    Ok(())
}

#[test]
fn public_campaign_urls_are_blocked_without_allow_host() -> Result<(), Box<dyn std::error::Error>> {
    let error = config_from(&["--campaign-url", "https://example.com/c/campaign-1"])
        .err()
        .map(|error| error.to_string())
        .unwrap_or_default();

    assert!(error.contains("requires --allow-host"));
    Ok(())
}

#[test]
fn public_campaign_urls_are_allowed_when_host_matches() -> Result<(), Box<dyn std::error::Error>> {
    let config = config_from(&[
        "--campaign-url",
        "https://example.com/c/campaign-1",
        "--allow-host",
        "example.com",
    ])?;

    assert_eq!(config.campaign_url.host_str(), Some("example.com"));
    assert_eq!(
        config.safety_policy.summary().allow_hosts,
        vec!["example.com".to_string()]
    );
    Ok(())
}

#[test]
fn private_network_urls_require_explicit_flag() -> Result<(), Box<dyn std::error::Error>> {
    let blocked = config_from(&["--campaign-url", "http://192.168.1.10/c/campaign-1"])
        .err()
        .map(|error| error.to_string())
        .unwrap_or_default();
    assert!(blocked.contains("--allow-private-network"));

    let allowed = config_from(&[
        "--campaign-url",
        "http://192.168.1.10/c/campaign-1",
        "--allow-private-network",
    ])?;
    assert!(allowed.safety_policy.summary().allow_private_network);
    Ok(())
}

#[test]
fn invalid_numeric_ranges_are_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let invalid_users = config_from(&[
        "--campaign-url",
        "http://localhost:8088/c/campaign-1",
        "--users",
        "0",
    ])
    .err()
    .map(|error| error.to_string())
    .unwrap_or_default();

    assert!(invalid_users.contains("users must be between"));

    let invalid_rate = config_from(&[
        "--campaign-url",
        "http://localhost:8088/c/campaign-1",
        "--conversion-rate",
        "1.5",
    ])
    .err()
    .map(|error| error.to_string())
    .unwrap_or_default();

    assert!(invalid_rate.contains("conversion-rate must be between"));
    Ok(())
}

#[test]
fn conversion_rate_requires_a_conversion_type() -> Result<(), Box<dyn std::error::Error>> {
    let error = config_from(&[
        "--campaign-url",
        "http://localhost:8088/c/campaign-1",
        "--conversion-rate",
        "0.5",
    ])
    .err()
    .map(|error| error.to_string())
    .unwrap_or_default();

    assert!(error.contains("--conversion-type lead or sale"));
    Ok(())
}
