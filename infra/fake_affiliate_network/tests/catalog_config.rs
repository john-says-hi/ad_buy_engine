use clap::Parser;
use fake_affiliate_network::cli::Cli;
use fake_affiliate_network::config::RunConfig;
use fake_affiliate_network::macros::{PostbackMacros, render_postback_url};

fn config_from(args: &[&str]) -> Result<RunConfig, Box<dyn std::error::Error>> {
    let cli = Cli::try_parse_from(
        std::iter::once("abe-fake-affiliate-network").chain(args.iter().copied()),
    )?;
    Ok(RunConfig::try_from(cli)?)
}

#[test]
fn config_defaults_to_loopback_dashboard_and_ad_buy_engine_postback()
-> Result<(), Box<dyn std::error::Error>> {
    let config = config_from(&[])?;

    assert_eq!(config.listen_address.to_string(), "127.0.0.1:8090");
    assert_eq!(config.dashboard_base_url(), "http://127.0.0.1:8090");
    assert_eq!(config.lead_threshold, 10);
    assert_eq!(config.sale_threshold, 100);
    assert!(config.postback_template.contains("127.0.0.1:8088/postback"));
    Ok(())
}

#[test]
fn invalid_listen_address_or_thresholds_are_rejected() {
    assert!(config_from(&["--listen-address", "not-a-socket"]).is_err());
    assert!(config_from(&["--listen-address", "0.0.0.0:8090"]).is_err());
    assert!(config_from(&["--lead-threshold", "0"]).is_err());
    assert!(config_from(&["--sale-threshold", "0"]).is_err());
}

#[test]
fn public_postback_templates_are_blocked_without_allowlist() {
    let template = "https://public.example/postback?cid={click_id}";

    assert!(config_from(&["--postback-template", template]).is_err());
    assert!(
        config_from(&[
            "--postback-template",
            template,
            "--allow-host",
            "public.example"
        ])
        .is_ok()
    );
}

#[test]
fn macro_rendering_maps_to_ad_buy_engine_aliases_and_encodes_values()
-> Result<(), Box<dyn std::error::Error>> {
    let url = render_postback_url(
        "http://127.0.0.1:8088/postback?cid={click_id}&type={event_type}&payout={payout}&currency={currency}&status={status}&txid={transaction_id}",
        &PostbackMacros {
            click_id: "visit 123",
            event_type: "Sale",
            payout: "49.00",
            currency: "USD",
            status: "approved",
            transaction_id: "tx/sale 1",
        },
    )?;

    assert_eq!(
        url.as_str(),
        "http://127.0.0.1:8088/postback?cid=visit%20123&type=Sale&payout=49.00&currency=USD&status=approved&txid=tx%2Fsale%201"
    );
    Ok(())
}
