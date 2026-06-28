use admin_dashboard::route::{NAVIGATION_ITEMS, Route};
use admin_dashboard::state::create_form::CreateFormDefinition;
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
fn offer_sources_has_expected_button_and_report_state() -> Result<(), &'static str> {
    let route = Route::OfferSources;
    let report = ReportState::for_route(route);
    let Some(form) = CreateFormDefinition::for_route(route) else {
        return Err("offer sources should have a form");
    };

    assert_eq!(route.label(), "Offer Sources");
    assert_eq!(route.create_button_label(), Some("New Offer Source"));
    assert_eq!(form.modal_id, "offer-sources");
    assert_eq!(form.title, "New Offer Source");
    assert!(form.contains_field_label("Name of Source:"));
    assert!(form.contains_field_label("Click ID"));
    assert_eq!(report.first_grouping, "Offer Sources");
    assert_eq!(report.second_grouping, "Drill Down");
    assert_eq!(report.third_grouping, "Drill Down");
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
    for route in [
        Route::Conversions,
        Route::Connection,
        Route::Browsers,
        Route::Device,
        Route::Os,
        Route::Date,
        Route::DayParting,
    ] {
        assert_eq!(route.create_button_label(), None);
        assert_eq!(CreateFormDefinition::for_route(route), None);
    }
}
