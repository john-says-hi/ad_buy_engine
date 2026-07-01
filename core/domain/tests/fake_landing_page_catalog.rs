use ad_buy_engine_domain::{
    LandingPageRole, fake_landing_page_by_id, fake_landing_page_catalog, fake_landing_page_url,
};

#[test]
fn catalog_exposes_five_stable_fake_landers() {
    let landers = fake_landing_page_catalog();

    assert_eq!(landers.len(), 5);
    assert!(
        landers
            .iter()
            .all(|lander| lander.id.starts_with("fake-lander-"))
    );
    assert!(
        landers
            .iter()
            .all(|lander| lander.name.starts_with("Fake "))
    );
    assert!(
        landers
            .iter()
            .all(|lander| lander.vertical.starts_with("fake-"))
    );
    assert!(
        landers
            .iter()
            .all(|lander| lander.tags.contains(&"fake-landing-page-server"))
    );
}

#[test]
fn catalog_maps_lander_roles_and_cta_counts() {
    let standard = fake_landing_page_by_id("fake-lander-standard-click-through")
        .expect("standard fake lander should exist");
    let lead_capture =
        fake_landing_page_by_id("fake-lander-lead-capture").expect("lead fake lander should exist");
    let advertorial =
        fake_landing_page_by_id("fake-lander-advertorial").expect("advertorial should exist");
    let after_optin =
        fake_landing_page_by_id("fake-lander-after-optin").expect("after opt-in should exist");
    let multi_cta = fake_landing_page_by_id("fake-lander-multi-cta")
        .expect("multi CTA fake lander should exist");

    assert_eq!(standard.role, LandingPageRole::Standard);
    assert_eq!(lead_capture.role, LandingPageRole::LeadCapture);
    assert_eq!(advertorial.role, LandingPageRole::Advertorial);
    assert_eq!(after_optin.role, LandingPageRole::AfterOptin);
    assert_eq!(multi_cta.role, LandingPageRole::Standard);
    assert_eq!(multi_cta.cta_count, 3);
    assert!(
        fake_landing_page_catalog()
            .iter()
            .filter(|lander| lander.id != "fake-lander-multi-cta")
            .all(|lander| lander.cta_count == 1)
    );
}

#[test]
fn generated_seed_urls_preserve_continuation_tokens() {
    let standard = fake_landing_page_by_id("fake-lander-standard-click-through")
        .expect("standard fake lander should exist");
    let multi_cta = fake_landing_page_by_id("fake-lander-multi-cta")
        .expect("multi CTA fake lander should exist");

    assert_eq!(
        fake_landing_page_url("http://127.0.0.1:8091/", standard),
        "http://127.0.0.1:8091/lander/fake-lander-standard-click-through?next={click_url_1}"
    );
    assert_eq!(
        fake_landing_page_url("http://127.0.0.1:8091", multi_cta),
        "http://127.0.0.1:8091/lander/fake-lander-multi-cta?cta1={click_url_1}&cta2={click_url_2}&cta3={click_url_3}"
    );
}

#[test]
fn catalog_copy_is_public_safe_and_fake() {
    for lander in fake_landing_page_catalog() {
        assert!(lander.display_copy.contains("Public-safe"));
        assert!(!lander.display_copy.to_ascii_lowercase().contains("real "));
        assert!(!lander.name.contains("Facebook"));
        assert!(!lander.name.contains("Google"));
    }

    assert!(fake_landing_page_by_id("real-client-lander").is_none());
}
