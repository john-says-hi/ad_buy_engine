use ad_buy_engine_domain::{EntityRow, ReportDimensionKey};
use admin_dashboard::route::{NAVIGATION_ITEMS, Route};
use admin_dashboard::state::create_form::CreateFormDefinition;
use admin_dashboard::state::entity_form::{EntityKind, FieldType, form_fields};
use admin_dashboard::state::report::{
    DATE_RANGE_OPTIONS, ReportDateRange, ReportState, ReportTotals, filter_rows_by_search,
};

#[test]
fn default_route_is_dashboard() {
    assert_eq!(Route::default(), Route::Dashboard);
    assert_eq!(Route::default().label(), "Dashboard");
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
            "Geo Settings",
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
    assert_eq!(Route::Conversions.create_button_label(), None);
    assert_eq!(CreateFormDefinition::for_route(Route::Conversions), None);
    assert_eq!(Route::Conversions.report_rows_endpoint(), None);
    assert_eq!(Route::GeolocationSettings.report_rows_endpoint(), None);
    assert!(!Route::GeolocationSettings.is_report());
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
        field_type(EntityKind::Offer, "weight"),
        Some(FieldType::Number)
    );
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
