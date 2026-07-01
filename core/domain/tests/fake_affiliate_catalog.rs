use ad_buy_engine_domain::{
    FakeAffiliateOfferKind, fake_affiliate_catalog, fake_affiliate_offer_by_id,
    fake_affiliate_offer_url,
};

#[test]
fn catalog_exposes_three_leads_and_two_sales() {
    let offers = fake_affiliate_catalog();

    assert_eq!(offers.len(), 5);
    assert_eq!(
        offers
            .iter()
            .filter(|offer| offer.kind == FakeAffiliateOfferKind::Lead)
            .count(),
        3
    );
    assert_eq!(
        offers
            .iter()
            .filter(|offer| offer.kind == FakeAffiliateOfferKind::Sale)
            .count(),
        2
    );
}

#[test]
fn catalog_defaults_are_public_safe_and_deterministic() {
    for offer in fake_affiliate_catalog() {
        assert!(offer.id.starts_with("fake-"));
        assert!(offer.name.starts_with("Fake "));
        assert!(offer.vertical.starts_with("fake-"));
        assert_eq!(offer.currency, "USD");
        assert!(offer.payout_value.is_finite());
        assert!(offer.payout_value > 0.0);
        match offer.kind {
            FakeAffiliateOfferKind::Lead => assert_eq!(offer.default_threshold, 10),
            FakeAffiliateOfferKind::Sale => assert_eq!(offer.default_threshold, 100),
        }
    }
}

#[test]
fn catalog_lookup_and_offer_links_are_stable() -> Result<(), Box<dyn std::error::Error>> {
    let offer = fake_affiliate_offer_by_id("fake-lead-solar-savings")
        .ok_or_else(|| std::io::Error::other("expected stable fake lead offer"))?;

    assert_eq!(offer.name, "Fake Solar Savings Lead");
    assert_eq!(
        fake_affiliate_offer_url("http://127.0.0.1:8090/", offer.id),
        "http://127.0.0.1:8090/click/fake-lead-solar-savings?subid={clickid}"
    );
    assert!(fake_affiliate_offer_by_id("real-network-offer").is_none());
    Ok(())
}
