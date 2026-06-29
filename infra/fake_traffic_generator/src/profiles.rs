#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualUserProfile {
    pub user_index: u32,
    pub user_agent: &'static str,
    pub accept_language: &'static str,
    pub referrer: &'static str,
    pub test_ip: String,
    pub source: &'static str,
    pub keyword: &'static str,
}

#[derive(Clone, Debug)]
pub struct DeterministicRng {
    state: u64,
}

const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
];
const ACCEPT_LANGUAGES: &[&str] = &[
    "en-US,en;q=0.9",
    "en-CA,en;q=0.9",
    "en-GB,en;q=0.8",
    "es-US,es;q=0.8,en;q=0.6",
    "fr-CA,fr;q=0.8,en;q=0.6",
];
const REFERRERS: &[&str] = &[
    "https://www.google.com/search?q=ad+buy+engine",
    "https://www.bing.com/search?q=campaign+tracking",
    "https://duckduckgo.com/?q=performance+tracking",
    "https://news.ycombinator.com/",
    "https://example.test/newsletter",
];
const SOURCES: &[&str] = &["google", "bing", "newsletter", "native", "social"];
const KEYWORDS: &[&str] = &[
    "campaign tracking",
    "performance marketing",
    "affiliate analytics",
    "postback testing",
    "click routing",
];

impl VirtualUserProfile {
    pub fn header_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("User-Agent", self.user_agent.to_string()),
            ("Accept-Language", self.accept_language.to_string()),
            ("Referer", self.referrer.to_string()),
            ("X-Forwarded-For", self.test_ip.clone()),
            ("CF-Connecting-IP", self.test_ip.clone()),
            ("X-ABE-Fake-Traffic", "1".to_string()),
        ]
    }

    pub fn query_pairs(&self, session_index: u64) -> Vec<(String, String)> {
        vec![
            ("utm_source".to_string(), self.source.to_string()),
            ("utm_medium".to_string(), "fake_traffic".to_string()),
            ("utm_campaign".to_string(), "abe_local_test".to_string()),
            ("keyword".to_string(), self.keyword.to_string()),
            ("abe_fake_user".to_string(), self.user_index.to_string()),
            ("abe_fake_session".to_string(), session_index.to_string()),
        ]
    }
}

impl DeterministicRng {
    pub fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            0x9e37_79b9_7f4a_7c15
        } else {
            seed
        };
        Self { state }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    pub fn index(&mut self, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        (self.next_u64() as usize) % len
    }

    pub fn bounded_u64(&mut self, upper_exclusive: u64) -> u64 {
        if upper_exclusive == 0 {
            return 0;
        }
        self.next_u64() % upper_exclusive
    }

    pub fn chance(&mut self, probability: f64) -> bool {
        if probability <= 0.0 {
            return false;
        }
        if probability >= 1.0 {
            return true;
        }
        let sample = self.next_u64() as f64 / u64::MAX as f64;
        sample < probability
    }
}

pub fn profile_for(seed: u64, user_index: u32) -> VirtualUserProfile {
    let mut rng = DeterministicRng::new(seed ^ u64::from(user_index).rotate_left(17));
    let last_octet = 1 + rng.bounded_u64(253);
    VirtualUserProfile {
        user_index,
        user_agent: USER_AGENTS[rng.index(USER_AGENTS.len())],
        accept_language: ACCEPT_LANGUAGES[rng.index(ACCEPT_LANGUAGES.len())],
        referrer: REFERRERS[rng.index(REFERRERS.len())],
        test_ip: format!("203.0.113.{last_octet}"),
        source: SOURCES[rng.index(SOURCES.len())],
        keyword: KEYWORDS[rng.index(KEYWORDS.len())],
    }
}

pub fn conversion_rng(seed: u64, session_index: u64) -> DeterministicRng {
    DeterministicRng::new(seed ^ 0xa5a5_5a5a_d3c3_b4b4 ^ session_index.rotate_left(23))
}
