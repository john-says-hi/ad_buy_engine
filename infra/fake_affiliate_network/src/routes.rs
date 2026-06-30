use std::collections::HashMap;

use axum::Router;
use axum::extract::{Form, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use url::Url;

use crate::config::RunConfig;
use crate::postback::{PostbackClient, PostbackDelivery};
use crate::render;
use crate::safety::SafetyPolicy;
use crate::state::{NetworkState, RuntimeSettings};

#[derive(Clone)]
struct AppState {
    base_url: String,
    safety_policy: SafetyPolicy,
    postback_client: PostbackClient,
    network_state: NetworkState,
}

impl AppState {
    fn try_new(config: RunConfig) -> Result<Self, reqwest::Error> {
        let postback_client =
            PostbackClient::new(config.safety_policy.clone(), config.request_timeout)?;
        let runtime_settings = RuntimeSettings::from_config(&config);
        Ok(Self {
            base_url: config.dashboard_base_url(),
            safety_policy: config.safety_policy,
            postback_client,
            network_state: NetworkState::new(runtime_settings),
        })
    }
}

#[derive(Debug, Deserialize)]
struct SettingsForm {
    postback_template: String,
    lead_threshold: u32,
    sale_threshold: u32,
}

#[derive(Debug, Deserialize)]
struct SampleForm {
    event_type: String,
    tracking_identifier: String,
}

pub fn build_router(config: RunConfig) -> Result<Router, reqwest::Error> {
    let state = AppState::try_new(config)?;
    Ok(Router::new()
        .route("/", get(dashboard))
        .route("/health", get(health))
        .route("/offers/{offer_id}", get(offer_detail))
        .route("/click/{offer_id}", get(click_offer))
        .route("/settings", post(update_settings))
        .route("/sample", post(sample_postback))
        .with_state(state))
}

async fn health() -> &'static str {
    "ok"
}

async fn dashboard(State(state): State<AppState>) -> Response {
    match state.network_state.snapshot() {
        Ok(snapshot) => Html(render::dashboard(&state.base_url, &snapshot, None)).into_response(),
        Err(error) => server_error(error.to_string()),
    }
}

async fn offer_detail(State(state): State<AppState>, Path(offer_id): Path<String>) -> Response {
    if ad_buy_engine_domain::fake_affiliate_offer_by_id(&offer_id).is_none() {
        return (
            StatusCode::NOT_FOUND,
            Html(render::offer_detail(&state.base_url, &offer_id)),
        )
            .into_response();
    }
    Html(render::offer_detail(&state.base_url, &offer_id)).into_response()
}

async fn click_offer(
    State(state): State<AppState>,
    Path(offer_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let Some(offer) = ad_buy_engine_domain::fake_affiliate_offer_by_id(&offer_id) else {
        return (
            StatusCode::NOT_FOUND,
            Html(render::offer_detail(&state.base_url, &offer_id)),
        )
            .into_response();
    };
    let outcome = match state.network_state.record_click(offer, &params) {
        Ok(outcome) => outcome,
        Err(error) => return server_error(error.to_string()),
    };
    if let Some(conversion) = outcome.generated_conversion.clone() {
        deliver_conversion(&state, conversion.id, &conversion.callback_url).await;
    }
    Html(render::click_landing(&offer_id, &outcome)).into_response()
}

async fn update_settings(
    State(state): State<AppState>,
    Form(form): Form<SettingsForm>,
) -> Response {
    let settings = match RuntimeSettings::try_new(
        form.postback_template,
        form.lead_threshold,
        form.sale_threshold,
    ) {
        Ok(settings) => settings,
        Err(error) => return settings_error_response(&state, &error.to_string()),
    };
    let rendered = match crate::config::validate_postback_template(&settings.postback_template) {
        Ok(url) => url,
        Err(error) => return settings_error_response(&state, &error.to_string()),
    };
    if let Err(error) = state.safety_policy.ensure_url_allowed(&rendered) {
        return settings_error_response(&state, &error.to_string());
    }
    if let Err(error) = state.network_state.update_settings(settings) {
        return server_error(error.to_string());
    }
    Redirect::to("/").into_response()
}

async fn sample_postback(State(state): State<AppState>, Form(form): Form<SampleForm>) -> Response {
    let Some(kind) = render::event_kind_from_value(&form.event_type) else {
        return settings_error_response(&state, "sample event type must be Lead or Sale");
    };
    let tracking_identifier = if form.tracking_identifier.trim().is_empty() {
        "sample-click-1"
    } else {
        form.tracking_identifier.trim()
    };
    let conversion = match state
        .network_state
        .record_sample_conversion(kind, tracking_identifier)
    {
        Ok(conversion) => conversion,
        Err(error) => return server_error(error.to_string()),
    };
    deliver_conversion(&state, conversion.id, &conversion.callback_url).await;
    Redirect::to("/").into_response()
}

async fn deliver_conversion(state: &AppState, conversion_id: u64, callback_url: &str) {
    let delivery = match Url::parse(callback_url) {
        Ok(url) => state.postback_client.deliver(&url).await,
        Err(error) => PostbackDelivery::failed(error.to_string()),
    };
    let _result = state
        .network_state
        .update_conversion_delivery(conversion_id, delivery);
}

fn settings_error_response(state: &AppState, error: &str) -> Response {
    match state.network_state.snapshot() {
        Ok(snapshot) => {
            Html(render::settings_error(&state.base_url, &snapshot, error)).into_response()
        }
        Err(snapshot_error) => server_error(snapshot_error.to_string()),
    }
}

fn server_error(message: String) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(format!(
            "<!doctype html><title>Fake Affiliate Network Error</title><h1>Error</h1><p>{message}</p>"
        )),
    )
        .into_response()
}
