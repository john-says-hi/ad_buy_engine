use clap::Parser;
use fake_traffic_generator::cli::Cli;
use fake_traffic_generator::config::RunConfig;
use fake_traffic_generator::metrics::{RequestRecord, RunSummary};
use fake_traffic_generator::output::format_summary;
use fake_traffic_generator::profiles::profile_for;
use fake_traffic_generator::scheduler::build_schedule;

fn config_from(args: &[&str]) -> Result<RunConfig, Box<dyn std::error::Error>> {
    let cli = Cli::try_parse_from(std::iter::once("abe-fake-traffic").chain(args.iter().copied()))?;
    Ok(RunConfig::try_from(cli)?)
}

#[test]
fn virtual_user_generation_is_deterministic() {
    let first = profile_for(42, 7);
    let second = profile_for(42, 7);
    let different_user = profile_for(42, 8);

    assert_eq!(first, second);
    assert_ne!(first, different_user);
}

#[test]
fn scheduler_applies_interval_jitter_and_keeps_configured_concurrency()
-> Result<(), Box<dyn std::error::Error>> {
    let config = config_from(&[
        "--campaign-url",
        "http://127.0.0.1:8088/c/campaign-1",
        "--users",
        "2",
        "--sessions",
        "3",
        "--interval-ms",
        "100",
        "--jitter-percent",
        "10",
        "--concurrency",
        "2",
        "--seed",
        "99",
    ])?;

    let schedule = build_schedule(&config);

    assert_eq!(schedule.len(), 6);
    assert_eq!(config.concurrency, 2);
    for scheduled in &schedule {
        assert!(scheduled.user_index < 2);
        let base = scheduled.session_index * 100;
        let start_ms = scheduled.start_after.as_millis() as u64;
        let lower_bound = base.saturating_sub(10);
        let upper_bound = base + 10;
        assert!(
            (lower_bound..=upper_bound).contains(&start_ms),
            "session {} start {} outside {}..={}",
            scheduled.session_index,
            start_ms,
            lower_bound,
            upper_bound
        );
    }
    Ok(())
}

#[test]
fn json_output_has_stable_top_level_fields() -> Result<(), Box<dyn std::error::Error>> {
    let config = config_from(&[
        "--campaign-url",
        "http://127.0.0.1:8088/c/campaign-1",
        "--output",
        "json",
    ])?;
    let mut summary = RunSummary::new(&config, 1);
    summary.record_request(&RequestRecord::success(
        "http://127.0.0.1:8088/api/health".to_string(),
        200,
        12,
    ));

    let output = format_summary(&summary, config.output_format)?;
    let value: serde_json::Value = serde_json::from_str(&output)?;

    assert_eq!(value["dry_run"], false);
    assert_eq!(value["campaign_url"], "http://127.0.0.1:8088/c/campaign-1");
    assert_eq!(value["http"]["requests"], 1);
    assert_eq!(value["http"]["status_buckets"]["200"], 1);
    assert!(value.get("conversions").is_some());
    assert!(value.get("redirects").is_some());
    Ok(())
}
