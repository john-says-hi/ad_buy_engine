use ad_buy_engine_domain::{
    DashboardMetricUnit, DomainSettingsResponse, DomainSettingsUpdate, DomainSetupStatus,
    EntityRow, ReportDimensionKey, RollbackEligibility, SequenceType, UpdatePhase,
    UpdateStatusResponse,
};
use admin_dashboard::app::initial_login_password;
use admin_dashboard::client::{domain_update_from_primary_domain, primary_domain_from_settings};
use admin_dashboard::route::{NAVIGATION_ITEMS, Route};
use admin_dashboard::state::create_form::CreateFormDefinition;
use admin_dashboard::state::entity_form::{
    EntityKind, FieldType, SaveDraft, SelectSource, default_values, draft_from_values, form_fields,
};
use admin_dashboard::state::report::{
    DATE_RANGE_OPTIONS, ReportDateRange, ReportState, ReportTotals, filter_rows_by_search,
};
use admin_dashboard::ui::dashboard_page::{dashboard_delta_text, dashboard_metric_text};
use admin_dashboard::ui::update_settings_page::{can_install, can_rollback, phase_label};

#[test]
fn default_route_is_dashboard() {
    assert_eq!(Route::default(), Route::Dashboard);
    assert_eq!(Route::default().label(), "Dashboard");
}

#[test]
fn login_password_prefills_only_during_first_run_setup() {
    assert_eq!(initial_login_password(true), "admin");
    assert_eq!(initial_login_password(false), "");
}

#[test]
fn navigation_labels_match_initial_shell_scope() {
    let labels: Vec<&str> = NAVIGATION_ITEMS.iter().map(|item| item.label).collect();

    assert_eq!(
        labels,
        vec![
            "Dashboard",
            "Campaigns",
            "Offers",
            "Landers",
            "Conversions",
            "Funnels",
            "Traffic Sources",
            "Offer Sources",
            "Connection",
            "Browsers",
            "Device",
            "OS",
            "Date",
            "Day Parting",
        ]
    );
}

#[test]
fn offer_sources_has_expected_button_and_report_state() -> Result<(), &'static str> {
    let route = Route::OfferSources;
    let report = ReportState::for_route(route, ReportDimensionKey::OfferSources);
    let Some(form) = CreateFormDefinition::for_route(route) else {
        return Err("offer sources should have a form");
    };

    assert_eq!(route.label(), "Offer Sources");
    assert_eq!(route.create_button_label(), Some("New Offer Source"));
    assert_eq!(form.modal_id, "offer-sources");
    assert_eq!(form.title, "New Offer Source");
    assert!(form.contains_field_label("Name of Source:"));
    assert!(form.contains_field_label("Click ID"));
    assert_eq!(report.first_grouping, ReportDimensionKey::OfferSources);
    assert_eq!(report.second_grouping, "Drill Down");
    assert_eq!(report.third_grouping, "Drill Down");
    assert_eq!(report.date_range, ReportDateRange::Today);
    assert_eq!(report.date_range.label(), "Today");
    assert_eq!(report.visit_total, 0);
    assert_eq!(report.unique_total, 0);
    Ok(())
}

#[test]
fn dashboard_is_not_a_report_page() {
    assert!(Route::Dashboard.is_dashboard());
    assert!(!Route::Dashboard.is_report());
    assert_eq!(Route::Dashboard.create_button_label(), None);
    assert_eq!(CreateFormDefinition::for_route(Route::Dashboard), None);
}

#[test]
fn dashboard_metric_formatting_matches_dashboard_units() {
    assert_eq!(
        dashboard_metric_text(1234.49, DashboardMetricUnit::Count),
        "1234"
    );
    assert_eq!(
        dashboard_metric_text(42.5, DashboardMetricUnit::Currency),
        "$42.50"
    );
    assert_eq!(
        dashboard_metric_text(-42.5, DashboardMetricUnit::Currency),
        "-$42.50"
    );
    assert_eq!(
        dashboard_metric_text(-0.0, DashboardMetricUnit::Currency),
        "$0.00"
    );
    assert_eq!(
        dashboard_metric_text(17.234, DashboardMetricUnit::Percentage),
        "17.2%"
    );
    assert_eq!(
        dashboard_metric_text(1.234, DashboardMetricUnit::Ratio),
        "1.23"
    );
}

#[test]
fn dashboard_delta_formatting_labels_comparisons() {
    assert_eq!(dashboard_delta_text(Some(8.25)), "+8.2%");
    assert_eq!(dashboard_delta_text(Some(-3.0)), "-3.0%");
    assert_eq!(dashboard_delta_text(Some(0.0)), "0.0%");
    assert_eq!(dashboard_delta_text(None), "No comparison");
}

#[test]
fn creatable_routes_have_legacy_modal_metadata() -> Result<(), String> {
    let cases = [
        (
            Route::Campaigns,
            "New Campaign",
            "campaigns",
            "Create Campaign",
            "Destination Type",
        ),
        (
            Route::Offers,
            "New Offer",
            "offer",
            "New Offer",
            "Offer Name:",
        ),
        (
            Route::Landers,
            "New Lander",
            "landing-pages",
            "New Lander",
            "Lander Name:",
        ),
        (
            Route::Conversions,
            "New Conversion Event",
            "conversions",
            "New Conversion Event",
            "Event Key",
        ),
        (
            Route::Funnels,
            "New Funnel",
            "funnels",
            "Create Funnel",
            "Funnel Name",
        ),
        (
            Route::TrafficSources,
            "New Traffic Source",
            "traffic-sources",
            "New Traffic Source",
            "Traffic Source Name:",
        ),
        (
            Route::OfferSources,
            "New Offer Source",
            "offer-sources",
            "New Offer Source",
            "Tracking Parameters",
        ),
    ];

    for (route, button_label, modal_id, title, expected_field) in cases {
        let Some(form) = CreateFormDefinition::for_route(route) else {
            return Err(format!("missing create form for {}", route.label()));
        };

        assert_eq!(route.create_button_label(), Some(button_label));
        assert_eq!(form.modal_id, modal_id);
        assert_eq!(form.title, title);
        assert!(form.contains_field_label(expected_field));
    }
    Ok(())
}

#[test]
fn non_creatable_report_routes_do_not_have_forms() {
    assert_eq!(
        Route::Conversions.create_button_label(),
        Some("New Conversion Event")
    );
    assert!(CreateFormDefinition::for_route(Route::Conversions).is_some());
    assert_eq!(Route::Conversions.report_rows_endpoint(), None);
    assert_eq!(Route::Settings.report_rows_endpoint(), None);
    assert_eq!(Route::DomainSettings.report_rows_endpoint(), None);
    assert_eq!(Route::GeolocationSettings.report_rows_endpoint(), None);
    assert_eq!(Route::UpdateSettings.report_rows_endpoint(), None);
    assert!(!Route::Settings.is_report());
    assert!(!Route::DomainSettings.is_report());
    assert!(!Route::GeolocationSettings.is_report());
    assert!(!Route::UpdateSettings.is_report());
    for (route, endpoint) in [
        (Route::Connection, "/api/reports/connection"),
        (Route::Browsers, "/api/reports/browsers"),
        (Route::Device, "/api/reports/device"),
        (Route::Os, "/api/reports/os"),
        (Route::Date, "/api/reports/date"),
        (Route::DayParting, "/api/reports/day-parting"),
    ] {
        assert_eq!(route.create_button_label(), None);
        assert_eq!(CreateFormDefinition::for_route(route), None);
        assert_eq!(route.report_rows_endpoint(), Some(endpoint));
    }
}

#[test]
fn settings_routes_are_not_report_routes() {
    for route in [
        Route::Settings,
        Route::DomainSettings,
        Route::GeolocationSettings,
        Route::UpdateSettings,
    ] {
        assert!(!route.is_report());
        assert_eq!(route.create_button_label(), None);
        assert_eq!(route.report_rows_endpoint(), None);
        assert_eq!(CreateFormDefinition::for_route(route), None);
    }
}

#[test]
fn update_settings_route_and_actions_have_expected_state() {
    let idle = UpdateStatusResponse {
        enabled: true,
        current_version: "v0.1.0".to_string(),
        latest_version: Some("v0.2.0".to_string()),
        active_slot: None,
        phase: UpdatePhase::Idle,
        last_result: None,
        rollback: RollbackEligibility::allowed("v0.1.0"),
        message: None,
    };
    let running = UpdateStatusResponse {
        phase: UpdatePhase::Downloading,
        ..idle.clone()
    };
    let disabled = UpdateStatusResponse {
        enabled: false,
        rollback: RollbackEligibility::blocked("disabled"),
        ..idle.clone()
    };

    assert_eq!(Route::Settings.label(), "Settings");
    assert_eq!(Route::Settings.path(), "/settings");
    assert_eq!(Route::UpdateSettings.label(), "Updates");
    assert_eq!(Route::UpdateSettings.path(), "/settings/updates");
    assert_eq!(Route::UpdateSettings.render_route(), Route::Settings);
    assert_eq!(Route::DomainSettings.render_route(), Route::Settings);
    assert_eq!(Route::GeolocationSettings.render_route(), Route::Settings);
    assert_eq!(phase_label(UpdatePhase::Downloading), "Downloading");
    assert!(can_install(&idle));
    assert!(can_rollback(&idle));
    assert!(!can_install(&running));
    assert!(!can_rollback(&running));
    assert!(!can_install(&disabled));
    assert!(!can_rollback(&disabled));
}

#[test]
fn domain_settings_payload_maps_primary_domain_to_both_roles() {
    let update = domain_update_from_primary_domain("track.example.com".to_string());

    assert_eq!(update.primary_tracking_domain, "track.example.com");
    assert_eq!(update.admin_dashboard_domain, "track.example.com");
    assert_eq!(update.tracking_base_url(), "https://track.example.com");
    assert_eq!(
        update.admin_dashboard_base_url(),
        "https://track.example.com"
    );
}

#[test]
fn invalid_domain_settings_payload_returns_validation_error_details() {
    let update =
        DomainSettingsUpdate::from_primary_domain("https://track.example.com/path".to_string());
    let fields: Vec<String> = update
        .validate()
        .into_iter()
        .map(|error| error.field)
        .collect();

    assert!(fields.contains(&"primary_tracking_domain".to_string()));
    assert!(fields.contains(&"admin_dashboard_domain".to_string()));
}

#[test]
fn saved_domain_settings_reload_into_primary_domain_form_value() {
    let settings = DomainSettingsResponse {
        primary_tracking_domain: "track.example.com".to_string(),
        tracking_base_url: "https://track.example.com".to_string(),
        admin_dashboard_domain: "track.example.com".to_string(),
        admin_dashboard_base_url: "https://track.example.com".to_string(),
        domain_setup_status: DomainSetupStatus::Configured,
        updated_at_millis: 100,
    };

    assert_eq!(primary_domain_from_settings(&settings), "track.example.com");
}

#[test]
fn report_dimensions_include_geolocation_drilldowns() {
    let labels: Vec<&str> = ReportDimensionKey::ALL
        .iter()
        .map(|dimension| dimension.label())
        .collect();

    assert!(labels.contains(&"Countries"));
    assert!(labels.contains(&"Regions / States"));
    assert!(labels.contains(&"Cities"));
    assert!(labels.contains(&"ASN Organizations"));
    assert_eq!(
        Route::from_report_dimension(ReportDimensionKey::TrafficSources),
        Some(Route::TrafficSources)
    );
    assert_eq!(
        Route::from_report_dimension(ReportDimensionKey::Cities),
        None
    );
    assert_eq!(
        Route::Campaigns.default_report_dimension(),
        Some(ReportDimensionKey::Campaigns)
    );
    assert_eq!(
        Route::Conversions.default_report_dimension(),
        Some(ReportDimensionKey::Conversions)
    );
}

#[test]
fn money_fields_accept_decimal_values() {
    assert_eq!(
        field_type(EntityKind::Campaign, "cost_value"),
        Some(FieldType::Decimal)
    );
    assert_eq!(
        field_type(EntityKind::Offer, "payout_value"),
        Some(FieldType::Decimal)
    );
    assert_eq!(
        field_type(EntityKind::LandingPage, "cta_count"),
        Some(FieldType::Number)
    );
    assert_eq!(
        field_type(EntityKind::LandingPage, "role"),
        Some(FieldType::Select(
            admin_dashboard::state::entity_form::SelectSource::Static(&[
                "standard",
                "lead_capture",
                "advertorial",
                "after_optin"
            ])
        ))
    );
    assert_eq!(
        field_type(EntityKind::Offer, "weight"),
        Some(FieldType::Number)
    );
    assert_eq!(
        field_type(EntityKind::ConversionEventType, "default_revenue_value"),
        Some(FieldType::Decimal)
    );
}

#[test]
fn funnel_form_exposes_lead_capture_split_template() {
    assert_eq!(
        field_type(EntityKind::Funnel, "funnel_template"),
        Some(FieldType::Select(SelectSource::Static(&[
            "simple",
            "lead_capture_split"
        ])))
    );
    assert_eq!(
        field_type(EntityKind::Funnel, "lead_capture_landing_page_id"),
        Some(FieldType::Select(SelectSource::LandingPages))
    );
    assert_eq!(
        field_type(EntityKind::Funnel, "advertorial_landing_page_id"),
        Some(FieldType::Select(SelectSource::LandingPages))
    );
    assert_eq!(
        field_type(EntityKind::Funnel, "sales_offer_id"),
        Some(FieldType::Select(SelectSource::Offers))
    );
    assert_eq!(
        field_type(EntityKind::Funnel, "direct_sales_weight"),
        Some(FieldType::Number)
    );
    assert_eq!(
        field_type(EntityKind::Funnel, "advertorial_weight"),
        Some(FieldType::Number)
    );
}

#[test]
fn lead_capture_split_template_builds_nested_funnel_sequence() -> Result<(), String> {
    let values = default_values(EntityKind::Funnel)
        .with_text("name", "Email Capture Split")
        .with_text("funnel_template", "lead_capture_split")
        .with_text("lead_capture_landing_page_id", "lead-lander")
        .with_text("advertorial_landing_page_id", "advertorial")
        .with_text("sales_offer_id", "sales-offer")
        .with_text("direct_sales_weight", "50")
        .with_text("advertorial_weight", "50");

    let SaveDraft::Funnel(draft) = draft_from_values(EntityKind::Funnel, &values)? else {
        return Err("expected funnel draft".to_string());
    };

    let sequence = draft
        .default_sequences
        .first()
        .ok_or_else(|| "expected default sequence".to_string())?;
    assert_eq!(sequence.id, "lead-capture-split");
    assert_eq!(sequence.sequence_type, SequenceType::Matrix);

    let root = sequence
        .paths
        .first()
        .ok_or_else(|| "expected root path".to_string())?;
    assert_eq!(root.landing_page_id.as_deref(), Some("lead-lander"));
    assert!(root.offers.is_empty());
    assert_eq!(root.children.len(), 2);

    assert_eq!(root.children[0].id, "direct-sales");
    assert_eq!(root.children[0].weight, 50);
    assert_eq!(root.children[0].landing_page_id, None);
    assert_eq!(root.children[0].offers[0].id, "sales-offer");

    assert_eq!(root.children[1].id, "advertorial");
    assert_eq!(root.children[1].weight, 50);
    assert_eq!(
        root.children[1].landing_page_id.as_deref(),
        Some("advertorial")
    );
    assert!(root.children[1].offers.is_empty());
    assert_eq!(root.children[1].children[0].offers[0].id, "sales-offer");

    Ok(())
}

#[test]
fn report_totals_sum_loaded_rows() {
    let totals = ReportTotals::from_rows(&[entity_row("one", 3, 1), entity_row("two", 5, 2)]);

    assert_eq!(totals.name_total, 2);
    assert_eq!(totals.visit_total, 8);
    assert_eq!(totals.unique_total, 3);
}

#[test]
fn date_range_options_match_toolbar_labels() {
    let labels: Vec<&str> = DATE_RANGE_OPTIONS
        .iter()
        .map(|option| option.label())
        .collect();

    assert_eq!(
        labels,
        vec![
            "Today",
            "Yesterday",
            "Last 3 Days",
            "Last 7 Days",
            "Last 14 Days",
            "Last 30 Days",
            "Last 6 Months",
            "All of Time",
        ]
    );

    for option in DATE_RANGE_OPTIONS {
        assert_eq!(
            ReportDateRange::from_storage_key(option.storage_key()),
            Some(*option)
        );
    }
    assert_eq!(ReportDateRange::from_storage_key("not-a-range"), None);
}

#[test]
fn search_filter_matches_name_detail_id_and_tracking_url() {
    let rows = vec![
        entity_row_with_detail("offer-one", "Home Page", "JVZoo", None),
        entity_row_with_detail(
            "campaign-two",
            "Search Campaign",
            "Traffic",
            Some("http://127.0.0.1:8088/c/search".to_string()),
        ),
    ];

    assert_eq!(filter_rows_by_search(&rows, "home").len(), 1);
    assert_eq!(filter_rows_by_search(&rows, "jvzoo").len(), 1);
    assert_eq!(filter_rows_by_search(&rows, "campaign-two").len(), 1);
    assert_eq!(filter_rows_by_search(&rows, "/c/search").len(), 1);
    assert_eq!(filter_rows_by_search(&rows, "missing").len(), 0);
    assert_eq!(filter_rows_by_search(&rows, "   ").len(), 2);
}

fn field_type(kind: EntityKind, key: &str) -> Option<FieldType> {
    form_fields(kind)
        .into_iter()
        .find(|field| field.key == key)
        .map(|field| field.field_type)
}

fn entity_row(name: &str, visits: i64, unique_visits: i64) -> EntityRow {
    entity_row_with_detail(name, name, "", None).with_counts(visits, unique_visits)
}

trait EntityRowTestExt {
    fn with_counts(self, visits: i64, unique_visits: i64) -> Self;
}

impl EntityRowTestExt for EntityRow {
    fn with_counts(mut self, visits: i64, unique_visits: i64) -> Self {
        self.visits = visits;
        self.unique_visits = unique_visits;
        self
    }
}

fn entity_row_with_detail(
    id: &str,
    name: &str,
    detail: &str,
    tracking_url: Option<String>,
) -> EntityRow {
    EntityRow {
        id: id.to_string(),
        name: name.to_string(),
        detail: detail.to_string(),
        visits: 0,
        unique_visits: 0,
        updated_at_millis: 0,
        tracking_url,
    }
}
