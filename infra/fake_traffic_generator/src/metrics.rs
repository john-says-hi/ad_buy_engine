use std::collections::BTreeMap;

use serde::Serialize;

use crate::config::RunConfig;
use crate::redirects::RedirectStep;
use crate::safety::SafetySummary;

#[derive(Clone, Debug)]
pub struct RequestRecord {
    pub url: String,
    pub status: Option<u16>,
    pub latency_ms: u128,
    pub error: Option<String>,
}

impl RequestRecord {
    pub fn success(url: String, status: u16, latency_ms: u128) -> Self {
        Self {
            url,
            status: Some(status),
            latency_ms,
            error: None,
        }
    }

    pub fn failure(url: String, latency_ms: u128, error: String) -> Self {
        Self {
            url,
            status: None,
            latency_ms,
            error: Some(error),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SessionOutcome {
    pub request_records: Vec<RequestRecord>,
    pub redirect_steps: Vec<RedirectStep>,
    pub visit_id: Option<String>,
    pub conversion_attempted: bool,
    pub conversion_sent: bool,
    pub conversion_skipped_no_visit_id: bool,
    pub blocked_redirects: u64,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RunSummary {
    pub dry_run: bool,
    pub campaign_url: String,
    pub users: u32,
    pub sessions_per_user: u32,
    pub planned_sessions: u64,
    pub completed_sessions: u64,
    pub failed_sessions: u64,
    pub concurrency: usize,
    pub seed: u64,
    pub safety: SafetySummary,
    pub http: HttpSummary,
    pub redirects: RedirectSummary,
    pub conversions: ConversionSummary,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct HttpSummary {
    pub requests: u64,
    pub status_buckets: BTreeMap<String, u64>,
    pub error_buckets: BTreeMap<String, u64>,
    pub latency_ms: LatencySummary,
    #[serde(skip)]
    latency_samples: Vec<u128>,
    #[serde(skip)]
    latency_total: u128,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RedirectSummary {
    pub blocked: u64,
    pub steps: u64,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ConversionSummary {
    pub attempted: u64,
    pub sent: u64,
    pub skipped_no_visit_id: u64,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct LatencySummary {
    pub min: u128,
    pub p50: u128,
    pub p95: u128,
    pub p99: u128,
    pub max: u128,
    pub average: f64,
}

impl RunSummary {
    pub fn new(config: &RunConfig, planned_sessions: u64) -> Self {
        Self {
            dry_run: false,
            campaign_url: config.campaign_url.to_string(),
            users: config.users,
            sessions_per_user: config.sessions_per_user,
            planned_sessions,
            completed_sessions: 0,
            failed_sessions: 0,
            concurrency: config.concurrency,
            seed: config.seed,
            safety: config.safety_policy.summary(),
            http: HttpSummary::default(),
            redirects: RedirectSummary::default(),
            conversions: ConversionSummary::default(),
        }
    }

    pub fn dry_run(config: &RunConfig, planned_sessions: u64) -> Self {
        Self {
            dry_run: true,
            ..Self::new(config, planned_sessions)
        }
    }

    pub fn record_session(&mut self, outcome: &SessionOutcome) {
        self.completed_sessions += 1;
        if !outcome.errors.is_empty() {
            self.failed_sessions += 1;
            for error in &outcome.errors {
                increment(&mut self.http.error_buckets, error);
            }
        }
        for request in &outcome.request_records {
            self.record_request(request);
        }
        self.redirects.steps += outcome.redirect_steps.len() as u64;
        self.redirects.blocked += outcome.blocked_redirects;
        if outcome.conversion_attempted {
            self.conversions.attempted += 1;
        }
        if outcome.conversion_sent {
            self.conversions.sent += 1;
        }
        if outcome.conversion_skipped_no_visit_id {
            self.conversions.skipped_no_visit_id += 1;
        }
    }

    pub fn record_request(&mut self, request: &RequestRecord) {
        self.http.requests += 1;
        match request.status {
            Some(status) => increment(&mut self.http.status_buckets, &status.to_string()),
            None => {
                let error = request.error.as_deref().unwrap_or("request_error");
                increment(&mut self.http.error_buckets, error);
            }
        }
        self.http.record_latency(request.latency_ms);
    }

    pub fn record_internal_error(&mut self, error: String) {
        increment(&mut self.http.error_buckets, &error);
    }

    pub fn finalize(&mut self) {
        self.http.finalize_latency();
    }
}

impl HttpSummary {
    fn record_latency(&mut self, sample: u128) {
        self.latency_samples.push(sample);
        self.latency_total = self.latency_total.saturating_add(sample);
        self.latency_ms.min = if self.latency_samples.len() == 1 {
            sample
        } else {
            self.latency_ms.min.min(sample)
        };
        self.latency_ms.max = self.latency_ms.max.max(sample);
        self.latency_ms.average = self.latency_total as f64 / self.latency_samples.len() as f64;
    }

    fn finalize_latency(&mut self) {
        if self.latency_samples.is_empty() {
            self.latency_ms = LatencySummary::default();
            return;
        }
        self.latency_ms = latency_summary(&self.latency_samples);
    }
}

fn latency_summary(samples: &[u128]) -> LatencySummary {
    if samples.is_empty() {
        return LatencySummary::default();
    }
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let total: u128 = sorted.iter().copied().sum();
    let average = total as f64 / sorted.len() as f64;
    LatencySummary {
        min,
        p50: percentile(&sorted, 50),
        p95: percentile(&sorted, 95),
        p99: percentile(&sorted, 99),
        max,
        average,
    }
}

fn percentile(sorted_samples: &[u128], percentile: usize) -> u128 {
    if sorted_samples.is_empty() {
        return 0;
    }
    let last_index = sorted_samples.len() - 1;
    let index = (last_index * percentile).div_ceil(100);
    sorted_samples[index.min(last_index)]
}

fn increment(buckets: &mut BTreeMap<String, u64>, key: &str) {
    let count = buckets.entry(key.to_string()).or_default();
    *count += 1;
}
