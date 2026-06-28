use ad_buy_engine_domain::{
    CampaignDraft, ConditionRule, DashboardConversionPathStep, DashboardDateWindow,
    DashboardDecision, DashboardKpi, DashboardMetricUnit, DashboardPerformancePoint,
    DashboardRecentEvent, DashboardSetupHealthItem, DashboardSummaryResponse, DashboardTone,
    DashboardTopMover, DashboardTrafficMix, DashboardTrafficSegment, DestinationType,
    DomainSettingsResponse, DomainSettingsUpdate, DomainSetupStatus, FunnelDraft, FunnelSequence,
    LandingPageDraft, LandingPageRole, OfferDraft, OfferSourceDraft, TrafficSourceDraft, UrlToken,
    ValidateDraft,
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
        role: LandingPageRole::Standard,
        expected_conversion_event_type_ids: Vec::new(),
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

#[test]
fn domain_settings_accepts_hostname_and_normalizes_base_urls() {
    let update = DomainSettingsUpdate::from_primary_domain("Track.Example.Com".to_string());

    assert!(update.validate().is_empty());
    assert_eq!(
        update.normalized_primary_tracking_domain(),
        "track.example.com"
    );
    assert_eq!(update.tracking_base_url(), "https://track.example.com");
    assert_eq!(
        update.admin_dashboard_base_url(),
        "https://track.example.com"
    );
}

#[test]
fn domain_settings_rejects_scheme_path_and_port() {
    let update =
        DomainSettingsUpdate::from_primary_domain("https://track.example.com/path".to_string());

    let errors = update.validate();

    assert!(
        errors
            .iter()
            .any(|error| error.field == "primary_tracking_domain")
    );
    assert!(
        errors
            .iter()
            .any(|error| error.field == "admin_dashboard_domain")
    );
}

#[test]
fn domain_settings_rejects_empty_or_space_containing_domains() {
    let empty = DomainSettingsUpdate::from_primary_domain(String::new());
    let spaced = DomainSettingsUpdate::from_primary_domain("track example.com".to_string());

    assert_eq!(empty.validate().len(), 2);
    assert_eq!(spaced.validate().len(), 2);
}

#[test]
fn domain_settings_response_serializes_admin_and_tracking_roles_separately()
-> Result<(), Box<dyn std::error::Error>> {
    let response = DomainSettingsResponse {
        primary_tracking_domain: "track.example.com".to_string(),
        tracking_base_url: "https://track.example.com".to_string(),
        admin_dashboard_domain: "admin.example.com".to_string(),
        admin_dashboard_base_url: "https://admin.example.com".to_string(),
        domain_setup_status: DomainSetupStatus::Configured,
        updated_at_millis: 42,
    };

    let json = serde_json::to_value(response)?;

    assert_eq!(json["primary_tracking_domain"], "track.example.com");
    assert_eq!(json["tracking_base_url"], "https://track.example.com");
    assert_eq!(json["admin_dashboard_domain"], "admin.example.com");
    assert_eq!(
        json["admin_dashboard_base_url"],
        "https://admin.example.com"
    );
    assert_eq!(json["domain_setup_status"], "configured");
    Ok(())
}

#[test]
fn dashboard_summary_contract_serializes_full_overview() -> Result<(), Box<dyn std::error::Error>> {
    let response = DashboardSummaryResponse {
        generated_at_millis: 100,
        current_window: DashboardDateWindow {
            label: "Today".to_string(),
            start_at_millis: Some(10),
            end_at_millis: Some(20),
        },
        comparison_window: Some(DashboardDateWindow {
            label: "Previous period".to_string(),
            start_at_millis: Some(0),
            end_at_millis: Some(10),
        }),
        kpis: vec![DashboardKpi {
            key: "profit".to_string(),
            label: "Profit".to_string(),
            value: 42.5,
            previous_value: Some(30.0),
            delta_percent: Some(41.66666666666667),
            unit: DashboardMetricUnit::Currency,
            tone: DashboardTone::Positive,
            estimated: true,
        }],
        performance: vec![DashboardPerformancePoint {
            label: "2026-06-28".to_string(),
            start_at_millis: 10,
            visits: 12,
            revenue: 70.0,
            cost: 27.5,
            profit: 42.5,
        }],
        decision_feed: vec![DashboardDecision {
            title: "Scale winner".to_string(),
            detail: "Campaign is profitable".to_string(),
            tone: DashboardTone::Positive,
            action_label: "Open campaign".to_string(),
            route_path: Some("/campaigns".to_string()),
        }],
        top_movers: vec![DashboardTopMover {
            category: "Campaign".to_string(),
            name: "Demo".to_string(),
            detail: "Traffic Source".to_string(),
            route_path: Some("/campaigns".to_string()),
            visits: 12,
            conversions: 3,
            revenue: 70.0,
            cost: 27.5,
            profit: 42.5,
            roi: 154.54545454545453,
        }],
        conversion_path: vec![DashboardConversionPathStep {
            label: "Visits".to_string(),
            count: 12,
            rate_from_previous: None,
        }],
        traffic_mix: vec![DashboardTrafficMix {
            dimension: "Device".to_string(),
            segments: vec![DashboardTrafficSegment {
                label: "Desktop".to_string(),
                visits: 9,
                share_percent: 75.0,
            }],
        }],
        recent_events: vec![DashboardRecentEvent {
            label: "Conversion".to_string(),
            detail: "Lead tracked".to_string(),
            occurred_at_millis: 18,
            tone: DashboardTone::Positive,
        }],
        setup_health: vec![DashboardSetupHealthItem {
            label: "Tracking domain".to_string(),
            detail: "Configured".to_string(),
            tone: DashboardTone::Positive,
            route_path: Some("/settings/domain".to_string()),
        }],
    };

    let json = serde_json::to_value(response)?;

    assert_eq!(json["kpis"][0]["unit"], "currency");
    assert_eq!(json["kpis"][0]["tone"], "positive");
    assert_eq!(json["decision_feed"][0]["route_path"], "/campaigns");
    assert_eq!(json["traffic_mix"][0]["segments"][0]["share_percent"], 75.0);
    assert_eq!(json["setup_health"][0]["tone"], "positive");
    Ok(())
}
