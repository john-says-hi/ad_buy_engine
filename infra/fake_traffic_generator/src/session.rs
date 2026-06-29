use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use reqwest::header::LOCATION;
use url::Url;

use crate::config::RunConfig;
use crate::conversions::{postback_url, should_send_conversion};
use crate::metrics::{RequestRecord, SessionOutcome};
use crate::profiles::{VirtualUserProfile, profile_for};
use crate::redirects::{
    RedirectStep, extract_go_url, extract_visit_id, is_redirect_status, resolve_location,
};
use crate::safety::health_url_for_local_campaign;
use crate::scheduler::ScheduledSession;

pub async fn preflight_health(
    client: &reqwest::Client,
    config: &RunConfig,
) -> Option<RequestRecord> {
    let health_url = health_url_for_local_campaign(&config.campaign_url)?;
    let (_, record) = send_get(client, &health_url, None).await;
    Some(record)
}

pub async fn run_session(
    client: &Arc<reqwest::Client>,
    config: &RunConfig,
    scheduled_session: ScheduledSession,
) -> SessionOutcome {
    let profile = profile_for(config.seed, scheduled_session.user_index);
    let mut outcome = SessionOutcome::default();
    let campaign_url = campaign_url_for_session(config, &profile, scheduled_session.session_index);
    let mut seen_urls = HashSet::new();
    seen_urls.insert(campaign_url.to_string());

    let (mut response, record) = send_get(client, &campaign_url, Some(&profile)).await;
    outcome.request_records.push(record);
    let mut current_url = campaign_url;
    let mut redirect_hops = 0;

    while let Some(next_response) = response {
        if !is_redirect_status(next_response.status()) {
            break;
        }
        let Some(location) = redirect_location(&next_response) else {
            outcome.errors.push("missing_redirect_location".to_string());
            break;
        };
        if redirect_hops >= config.max_redirect_hops {
            outcome.errors.push("max_redirect_hops".to_string());
            break;
        }
        redirect_hops += 1;

        let target_url = match resolve_location(&current_url, &location) {
            Ok(url) => url,
            Err(error) => {
                outcome
                    .errors
                    .push(format!("invalid_redirect_location:{error}"));
                break;
            }
        };
        let follow_url = extract_go_url(&current_url, &target_url).unwrap_or(target_url);
        if let Some(visit_id) = extract_visit_id(&follow_url) {
            outcome.visit_id.get_or_insert(visit_id);
        }

        let mut redirect_step = RedirectStep {
            from_url: current_url.to_string(),
            status: next_response.status().as_u16(),
            location: Some(location),
            followed_url: None,
            blocked_reason: None,
        };

        if let Err(error) = config.safety_policy.ensure_url_allowed(&follow_url) {
            redirect_step.blocked_reason = Some(error.to_string());
            outcome.blocked_redirects += 1;
            outcome.redirect_steps.push(redirect_step);
            break;
        }

        let follow_url_text = follow_url.to_string();
        if !seen_urls.insert(follow_url_text.clone()) {
            redirect_step.blocked_reason = Some("redirect_loop".to_string());
            outcome.errors.push("redirect_loop".to_string());
            outcome.redirect_steps.push(redirect_step);
            break;
        }

        redirect_step.followed_url = Some(follow_url_text);
        outcome.redirect_steps.push(redirect_step);
        let (next, record) = send_get(client, &follow_url, Some(&profile)).await;
        outcome.request_records.push(record);
        current_url = follow_url;
        response = next;
    }

    maybe_send_conversion(
        client,
        config,
        &profile,
        scheduled_session.session_index,
        &mut outcome,
    )
    .await;

    outcome
}

fn campaign_url_for_session(
    config: &RunConfig,
    profile: &VirtualUserProfile,
    session_index: u64,
) -> Url {
    let mut url = config.campaign_url.clone();
    {
        let mut pairs = url.query_pairs_mut();
        for (key, value) in profile.query_pairs(session_index) {
            pairs.append_pair(&key, &value);
        }
    }
    url
}

async fn maybe_send_conversion(
    client: &Arc<reqwest::Client>,
    config: &RunConfig,
    profile: &VirtualUserProfile,
    session_index: u64,
    outcome: &mut SessionOutcome,
) {
    if !should_send_conversion(config, session_index) {
        return;
    }
    outcome.conversion_attempted = true;
    let Some(visit_id) = outcome.visit_id.as_deref() else {
        outcome.conversion_skipped_no_visit_id = true;
        return;
    };
    let Some(url) = postback_url(config, visit_id, session_index) else {
        outcome
            .errors
            .push("conversion_url_unavailable".to_string());
        return;
    };

    tokio::time::sleep(config.conversion_delay).await;
    let (response, record) = send_get(client, &url, Some(profile)).await;
    outcome.conversion_sent = record
        .status
        .is_some_and(|status| (200..400).contains(&status));
    if response.is_none() || !outcome.conversion_sent {
        outcome.errors.push("conversion_failed".to_string());
    }
    outcome.request_records.push(record);
}

async fn send_get(
    client: &reqwest::Client,
    url: &Url,
    profile: Option<&VirtualUserProfile>,
) -> (Option<reqwest::Response>, RequestRecord) {
    let started = Instant::now();
    let mut request = client.get(url.clone());
    if let Some(profile) = profile {
        for (name, value) in profile.header_pairs() {
            request = request.header(name, value);
        }
    }

    match request.send().await {
        Ok(response) => {
            let latency_ms = started.elapsed().as_millis();
            let status = response.status().as_u16();
            (
                Some(response),
                RequestRecord::success(url.to_string(), status, latency_ms),
            )
        }
        Err(error) => {
            let latency_ms = started.elapsed().as_millis();
            (
                None,
                RequestRecord::failure(
                    url.to_string(),
                    latency_ms,
                    classify_request_error(&error).to_string(),
                ),
            )
        }
    }
}

fn redirect_location(response: &reqwest::Response) -> Option<String> {
    response
        .headers()
        .get(LOCATION)
        .and_then(|value| value.to_str().ok())
        .map(ToString::to_string)
}

fn classify_request_error(error: &reqwest::Error) -> &'static str {
    if error.is_timeout() {
        "timeout"
    } else if error.is_connect() {
        "connect"
    } else if error.is_redirect() {
        "redirect"
    } else {
        "request_error"
    }
}
