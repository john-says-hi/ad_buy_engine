use ad_buy_engine_domain::{FakeAffiliateOffer, FakeAffiliateOfferKind};

pub const APPROVED_STATUS: &str = "approved";

pub fn threshold_for_offer(
    offer: FakeAffiliateOffer,
    lead_threshold: u32,
    sale_threshold: u32,
) -> u32 {
    match offer.kind {
        FakeAffiliateOfferKind::Lead => lead_threshold,
        FakeAffiliateOfferKind::Sale => sale_threshold,
    }
}

pub fn transaction_id_for_threshold(
    offer: FakeAffiliateOffer,
    qualifying_click_count: u64,
) -> String {
    format!(
        "fan-{}-{}-{}",
        offer.id,
        offer.event_type().to_ascii_lowercase(),
        qualifying_click_count
    )
}

pub fn payout_string(offer: FakeAffiliateOffer) -> String {
    format!("{:.2}", offer.payout_value)
}
