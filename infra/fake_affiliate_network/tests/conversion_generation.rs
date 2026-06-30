use std::collections::HashMap;

use ad_buy_engine_domain::fake_affiliate_offer_by_id;
use fake_affiliate_network::config::RunConfig;
use fake_affiliate_network::state::{NetworkState, RuntimeSettings};

fn state_with_default_settings() -> Result<NetworkState, Box<dyn std::error::Error>> {
    let cli = fake_affiliate_network::cli::Cli {
        listen_address: "127.0.0.1:8090".to_string(),
        postback_template: "http://127.0.0.1:8088/postback?cid={click_id}&type={event_type}&payout={payout}&currency={currency}&status={status}&txid={transaction_id}".to_string(),
        lead_threshold: 10,
        sale_threshold: 100,
        request_timeout_seconds: 5,
        allow_host: Vec::new(),
        allow_private_network: false,
    };
    let config = RunConfig::try_from(cli)?;
    Ok(NetworkState::new(RuntimeSettings::from_config(&config)))
}

fn params(tracking_identifier: &str) -> HashMap<String, String> {
    HashMap::from([("subid".to_string(), tracking_identifier.to_string())])
}

#[test]
fn ten_qualifying_lead_clicks_generate_one_lead_conversion()
-> Result<(), Box<dyn std::error::Error>> {
    let state = state_with_default_settings()?;
    let offer = fake_affiliate_offer_by_id("fake-lead-solar-savings")
        .ok_or_else(|| std::io::Error::other("lead offer"))?;

    for index in 1..10 {
        let outcome = state.record_click(offer, &params(&format!("visit-{index}")))?;
        assert!(outcome.generated_conversion.is_none());
    }
    let outcome = state.record_click(offer, &params("visit-10"))?;

    let conversion = outcome
        .generated_conversion
        .ok_or_else(|| std::io::Error::other("tenth lead click should convert"))?;
    assert_eq!(conversion.event_type, "Lead");
    assert_eq!(conversion.tracking_identifier, "visit-10");
    assert_eq!(
        conversion.transaction_id,
        "fan-fake-lead-solar-savings-lead-10"
    );
    assert!(conversion.callback_url.contains("cid=visit-10"));
    assert!(conversion.callback_url.contains("type=Lead"));
    Ok(())
}

#[test]
fn one_hundred_sale_clicks_generate_one_sale_conversion_with_revenue()
-> Result<(), Box<dyn std::error::Error>> {
    let state = state_with_default_settings()?;
    let offer = fake_affiliate_offer_by_id("fake-sale-course-bundle")
        .ok_or_else(|| std::io::Error::other("sale offer"))?;

    for index in 1..100 {
        let outcome = state.record_click(offer, &params(&format!("sale-visit-{index}")))?;
        assert!(outcome.generated_conversion.is_none());
    }
    let outcome = state.record_click(offer, &params("sale-visit-100"))?;

    let conversion = outcome
        .generated_conversion
        .ok_or_else(|| std::io::Error::other("hundredth sale click should convert"))?;
    assert_eq!(conversion.event_type, "Sale");
    assert_eq!(conversion.payout, "49.00");
    assert_eq!(conversion.currency, "USD");
    assert_eq!(
        conversion.transaction_id,
        "fan-fake-sale-course-bundle-sale-100"
    );
    Ok(())
}

#[test]
fn fresh_runs_use_the_same_threshold_positions_and_transaction_pattern()
-> Result<(), Box<dyn std::error::Error>> {
    let offer = fake_affiliate_offer_by_id("fake-lead-solar-savings")
        .ok_or_else(|| std::io::Error::other("lead offer"))?;
    let first = state_with_default_settings()?;
    let second = state_with_default_settings()?;

    for index in 1..=10 {
        let tracking_identifier = format!("visit-{index}");
        let first_outcome = first.record_click(offer, &params(&tracking_identifier))?;
        let second_outcome = second.record_click(offer, &params(&tracking_identifier))?;
        assert_eq!(
            first_outcome
                .generated_conversion
                .as_ref()
                .map(|conversion| conversion.transaction_id.clone()),
            second_outcome
                .generated_conversion
                .as_ref()
                .map(|conversion| conversion.transaction_id.clone())
        );
    }
    Ok(())
}
