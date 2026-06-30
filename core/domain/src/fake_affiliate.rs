#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FakeAffiliateOfferKind {
    Lead,
    Sale,
}

impl FakeAffiliateOfferKind {
    pub fn event_type(self) -> &'static str {
        match self {
            Self::Lead => "Lead",
            Self::Sale => "Sale",
        }
    }

    pub fn payout_model(self) -> &'static str {
        match self {
            Self::Lead => "CPA Lead",
            Self::Sale => "CPA Sale",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FakeAffiliateOffer {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: FakeAffiliateOfferKind,
    pub payout_value: f64,
    pub currency: &'static str,
    pub vertical: &'static str,
    pub default_threshold: u32,
    pub display_copy: &'static str,
}

impl FakeAffiliateOffer {
    pub fn event_type(self) -> &'static str {
        self.kind.event_type()
    }

    pub fn payout_model(self) -> &'static str {
        self.kind.payout_model()
    }
}

pub const FAKE_AFFILIATE_OFFER_SOURCE_ID: &str = "fake-affiliate-network";
pub const FAKE_AFFILIATE_OFFER_SOURCE_NAME: &str = "Fake Affiliate Network";
pub const FAKE_AFFILIATE_DEFAULT_BASE_URL: &str = "http://127.0.0.1:8090";
pub const FAKE_AFFILIATE_CLICK_ID_TOKEN: &str = "{clickid}";

pub const FAKE_AFFILIATE_OFFERS: &[FakeAffiliateOffer] = &[
    FakeAffiliateOffer {
        id: "fake-lead-solar-savings",
        name: "Fake Solar Savings Lead",
        kind: FakeAffiliateOfferKind::Lead,
        payout_value: 4.50,
        currency: "USD",
        vertical: "fake-home-services",
        default_threshold: 10,
        display_copy: "Public-safe lead form for a fictional home solar savings estimate.",
    },
    FakeAffiliateOffer {
        id: "fake-lead-fitness-trial",
        name: "Fake Fitness Trial Lead",
        kind: FakeAffiliateOfferKind::Lead,
        payout_value: 5.25,
        currency: "USD",
        vertical: "fake-health-fitness",
        default_threshold: 10,
        display_copy: "Public-safe lead form for a fictional fitness starter trial.",
    },
    FakeAffiliateOffer {
        id: "fake-lead-credit-checkup",
        name: "Fake Credit Checkup Lead",
        kind: FakeAffiliateOfferKind::Lead,
        payout_value: 7.00,
        currency: "USD",
        vertical: "fake-personal-finance",
        default_threshold: 10,
        display_copy: "Public-safe lead form for a fictional credit score checkup.",
    },
    FakeAffiliateOffer {
        id: "fake-sale-course-bundle",
        name: "Fake Course Bundle Sale",
        kind: FakeAffiliateOfferKind::Sale,
        payout_value: 49.00,
        currency: "USD",
        vertical: "fake-digital-education",
        default_threshold: 100,
        display_copy: "Public-safe checkout for a fictional digital course bundle.",
    },
    FakeAffiliateOffer {
        id: "fake-sale-garden-kit",
        name: "Fake Smart Garden Kit Sale",
        kind: FakeAffiliateOfferKind::Sale,
        payout_value: 89.00,
        currency: "USD",
        vertical: "fake-home-garden",
        default_threshold: 100,
        display_copy: "Public-safe checkout for a fictional smart garden kit.",
    },
];

pub fn fake_affiliate_catalog() -> &'static [FakeAffiliateOffer] {
    FAKE_AFFILIATE_OFFERS
}

pub fn fake_affiliate_offer_by_id(id: &str) -> Option<FakeAffiliateOffer> {
    fake_affiliate_catalog()
        .iter()
        .copied()
        .find(|offer| offer.id == id)
}

pub fn fake_affiliate_offer_url(base_url: &str, offer_id: &str) -> String {
    let base_url = base_url.trim().trim_end_matches('/');
    format!("{base_url}/click/{offer_id}?subid={FAKE_AFFILIATE_CLICK_ID_TOKEN}")
}
