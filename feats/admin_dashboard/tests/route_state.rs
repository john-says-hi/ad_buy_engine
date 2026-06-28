use admin_dashboard::route::{NAVIGATION_ITEMS, Route};
use admin_dashboard::state::report::ReportState;

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
        ]
    );
}

#[test]
fn offer_sources_has_expected_button_and_report_state() {
    let route = Route::OfferSources;
    let report = ReportState::for_route(route);

    assert_eq!(route.label(), "Offer Sources");
    assert_eq!(route.create_button_label(), Some("New Offer Source"));
    assert_eq!(report.first_grouping, "Offer Sources");
    assert_eq!(report.second_grouping, "Drill Down");
    assert_eq!(report.third_grouping, "Drill Down");
    assert_eq!(report.visit_total, 0);
    assert_eq!(report.unique_total, 0);
}

#[test]
fn dashboard_is_not_a_report_page() {
    assert!(Route::Dashboard.is_dashboard());
    assert!(!Route::Dashboard.is_report());
    assert_eq!(Route::Dashboard.create_button_label(), None);
}
