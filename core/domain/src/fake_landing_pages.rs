use crate::LandingPageRole;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FakeLandingPage {
    pub id: &'static str,
    pub name: &'static str,
    pub role: LandingPageRole,
    pub cta_count: u8,
    pub route_path: &'static str,
    pub vertical: &'static str,
    pub tags: &'static [&'static str],
    pub display_copy: &'static str,
}

impl FakeLandingPage {
    pub fn continuation_parameters(self) -> &'static [&'static str] {
        match self.cta_count {
            1 => SINGLE_CTA_CONTINUATION_PARAMETERS,
            2 => TWO_CTA_CONTINUATION_PARAMETERS,
            3 => THREE_CTA_CONTINUATION_PARAMETERS,
            _ => &[],
        }
    }

    pub fn continuation_tokens(self) -> Vec<(&'static str, String)> {
        self.continuation_parameters()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, parameter)| (parameter, format!("{{click_url_{}}}", index + 1)))
            .collect()
    }
}

const SINGLE_CTA_CONTINUATION_PARAMETERS: &[&str] = &["next"];
const TWO_CTA_CONTINUATION_PARAMETERS: &[&str] = &["cta1", "cta2"];
const THREE_CTA_CONTINUATION_PARAMETERS: &[&str] = &["cta1", "cta2", "cta3"];

pub const FAKE_LANDING_PAGE_DEFAULT_BASE_URL: &str = "http://127.0.0.1:8091";

pub const FAKE_LANDING_PAGES: &[FakeLandingPage] = &[
    FakeLandingPage {
        id: "fake-lander-standard-click-through",
        name: "Fake Standard Click-Through Lander",
        role: LandingPageRole::Standard,
        cta_count: 1,
        route_path: "/lander/fake-lander-standard-click-through",
        vertical: "fake-general-offers",
        tags: &["fake-landing-page-server", "standard", "click-through"],
        display_copy: "Public-safe click-through page for validating one tracked CTA before a fake offer.",
    },
    FakeLandingPage {
        id: "fake-lander-lead-capture",
        name: "Fake Lead Capture Lander",
        role: LandingPageRole::LeadCapture,
        cta_count: 1,
        route_path: "/lander/fake-lander-lead-capture",
        vertical: "fake-lead-generation",
        tags: &["fake-landing-page-server", "lead-capture", "opt-in"],
        display_copy: "Public-safe fake opt-in page that discards submitted values and continues the funnel.",
    },
    FakeLandingPage {
        id: "fake-lander-advertorial",
        name: "Fake Advertorial Lander",
        role: LandingPageRole::Advertorial,
        cta_count: 1,
        route_path: "/lander/fake-lander-advertorial",
        vertical: "fake-editorial-review",
        tags: &["fake-landing-page-server", "advertorial", "article"],
        display_copy: "Public-safe article-style page for validating tracked advertorial click-throughs.",
    },
    FakeLandingPage {
        id: "fake-lander-after-optin",
        name: "Fake After-Opt-In Thank You Lander",
        role: LandingPageRole::AfterOptin,
        cta_count: 1,
        route_path: "/lander/fake-lander-after-optin",
        vertical: "fake-thank-you-flow",
        tags: &["fake-landing-page-server", "after-optin", "thank-you"],
        display_copy: "Public-safe thank-you page for validating continuation after a fake opt-in step.",
    },
    FakeLandingPage {
        id: "fake-lander-multi-cta",
        name: "Fake Multi-CTA Split-Test Lander",
        role: LandingPageRole::Standard,
        cta_count: 3,
        route_path: "/lander/fake-lander-multi-cta",
        vertical: "fake-split-test",
        tags: &["fake-landing-page-server", "multi-cta", "split-test"],
        display_copy: "Public-safe split-test page for validating three distinct tracked CTA slots.",
    },
];

pub fn fake_landing_page_catalog() -> &'static [FakeLandingPage] {
    FAKE_LANDING_PAGES
}

pub fn fake_landing_page_by_id(id: &str) -> Option<FakeLandingPage> {
    fake_landing_page_catalog()
        .iter()
        .copied()
        .find(|landing_page| landing_page.id == id)
}

pub fn fake_landing_page_url(base_url: &str, landing_page: FakeLandingPage) -> String {
    let base_url = base_url.trim().trim_end_matches('/');
    let query = landing_page
        .continuation_tokens()
        .into_iter()
        .map(|(parameter, token)| format!("{parameter}={token}"))
        .collect::<Vec<_>>()
        .join("&");
    format!("{base_url}{}?{query}", landing_page.route_path)
}
