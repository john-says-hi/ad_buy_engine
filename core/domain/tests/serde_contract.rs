use ad_buy_engine_domain::{
    CampaignDraft, ConditionRule, DestinationType, FunnelDraft, FunnelSequence, LandingPageDraft,
    OfferDraft, OfferSourceDraft, TrafficSourceDraft, UrlToken, ValidateDraft,
};

#[test]
fn dto_round_trips_through_json() -> Result<(), Box<dyn std::error::Error>> {
    let draft = CampaignDraft {
        traffic_source_id: "traffic-source-1".to_string(),
        destination_type: DestinationType::DirectSequence,
        funnel_id: None,
        direct_sequence: Some(FunnelSequence::default_offer("offer-1")),
        cost_model: "CPC".to_string(),
        cost_value: 1.25,
        country: "US".to_string(),
        name: "Demo".to_string(),
        notes: "Notes".to_string(),
    };

    let encoded = serde_json::to_string(&draft)?;
    let decoded: CampaignDraft = serde_json::from_str(&encoded)?;

    assert_eq!(decoded, draft);
    Ok(())
}

#[test]
fn validates_required_references() {
    let campaign = CampaignDraft {
        traffic_source_id: String::new(),
        destination_type: DestinationType::Funnel,
        funnel_id: None,
        direct_sequence: None,
        cost_model: "CPC".to_string(),
        cost_value: 0.0,
        country: "Global".to_string(),
        name: String::new(),
        notes: String::new(),
    };

    let errors = campaign.validate();

    assert!(
        errors
            .iter()
            .any(|error| error.field == "traffic_source_id")
    );
    assert!(errors.iter().any(|error| error.field == "funnel_id"));
    assert!(errors.iter().any(|error| error.field == "name"));
}

#[test]
fn every_create_dto_has_validation_and_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let token = UrlToken {
        name: "click".to_string(),
        token: "{clickid}".to_string(),
    };
    let offer_source = OfferSourceDraft {
        name: "Source".to_string(),
        tokens: vec![token.clone()],
        tracking_domain: "main".to_string(),
        tracking_method: "postback".to_string(),
        payout_currency: "USD".to_string(),
        postback_url: "https://example.com/postback".to_string(),
        append_click_id: true,
        accept_duplicate_postbacks: false,
        whitelist_postback_ips: Vec::new(),
        referrer_handling: "do_nothing".to_string(),
        notes: String::new(),
    };
    let offer = OfferDraft {
        offer_source_id: "source".to_string(),
        country: "Global".to_string(),
        name: "Offer".to_string(),
        tags: vec!["tag".to_string()],
        url: "https://example.com/offer".to_string(),
        url_tokens: vec![token.clone()],
        payout_model: "fixed".to_string(),
        payout_value: 1.0,
        currency: "USD".to_string(),
        language: "en".to_string(),
        vertical: "demo".to_string(),
        weight: 100,
        notes: String::new(),
    };
    let landing_page = LandingPageDraft {
        country: "Global".to_string(),
        name: "Lander".to_string(),
        tags: Vec::new(),
        url: "https://example.com/lander".to_string(),
        url_tokens: vec![token],
        cta_count: 1,
        language: "en".to_string(),
        vertical: "demo".to_string(),
        weight: 100,
        notes: String::new(),
    };
    let traffic_source = TrafficSourceDraft {
        name: "Traffic".to_string(),
        external_id_parameter: "clickid".to_string(),
        cost_parameter: "cost".to_string(),
        custom_parameters: Vec::new(),
        currency: "USD".to_string(),
        postback_urls: Vec::new(),
        pixel_url: String::new(),
        track_impressions: false,
        direct_tracking: true,
        notes: String::new(),
    };
    let funnel = FunnelDraft {
        country: "Global".to_string(),
        name: "Funnel".to_string(),
        redirect_handling: "default".to_string(),
        referrer_handling: "do_nothing".to_string(),
        conditional_sequences: vec![FunnelSequence {
            conditions: vec![ConditionRule::query_parameter("rule-1", "src", "paid")],
            ..FunnelSequence::default_offer("offer")
        }],
        default_sequences: vec![FunnelSequence::default_offer("offer")],
        notes: String::new(),
    };

    assert!(offer_source.validate().is_empty());
    assert!(offer.validate().is_empty());
    assert!(landing_page.validate().is_empty());
    assert!(traffic_source.validate().is_empty());
    assert!(funnel.validate().is_empty());
    serde_json::to_string(&offer_source)?;
    serde_json::to_string(&offer)?;
    serde_json::to_string(&landing_page)?;
    serde_json::to_string(&traffic_source)?;
    serde_json::to_string(&funnel)?;
    Ok(())
}
