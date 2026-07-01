use std::collections::HashMap;

use ad_buy_engine_domain::{FakeLandingPage, fake_landing_page_by_id};
use axum::Router;
use axum::extract::{Form, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use url::Url;

use crate::config::RunConfig;
use crate::render::{self, ActionState, ContinuationAction};
use crate::safety::SafetyPolicy;

#[derive(Clone)]
struct AppState {
    base_url: String,
    safety_policy: SafetyPolicy,
}

impl AppState {
    fn new(config: RunConfig) -> Self {
        Self {
            base_url: config.server_base_url(),
            safety_policy: config.safety_policy,
        }
    }
}

#[derive(Debug, Deserialize)]
struct OptInForm {
    next: Option<String>,
    #[serde(flatten)]
    _ignored_fields: HashMap<String, String>,
}

pub fn build_router(config: RunConfig) -> Router {
    let state = AppState::new(config);
    Router::new()
        .route("/", get(dashboard))
        .route("/health", get(health))
        .route("/lander/{lander_id}", get(lander_page))
        .route("/lander/{lander_id}/opt-in", post(submit_opt_in))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn dashboard(State(state): State<AppState>) -> Html<String> {
    Html(render::dashboard(&state.base_url))
}

async fn lander_page(
    State(state): State<AppState>,
    Path(lander_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let Some(lander) = fake_landing_page_by_id(&lander_id) else {
        return (StatusCode::NOT_FOUND, Html(render::not_found(&lander_id))).into_response();
    };

    match continuation_actions(lander, &params, &state.safety_policy) {
        Ok(actions) => {
            Html(render::lander_page(lander, ActionState::Ready(actions))).into_response()
        }
        Err(problem) => (
            StatusCode::BAD_REQUEST,
            Html(render::lander_page(
                lander,
                ActionState::Problem(problem.message()),
            )),
        )
            .into_response(),
    }
}

async fn submit_opt_in(
    State(state): State<AppState>,
    Path(lander_id): Path<String>,
    Form(form): Form<OptInForm>,
) -> Response {
    let Some(lander) = fake_landing_page_by_id(&lander_id) else {
        return (StatusCode::NOT_FOUND, Html(render::not_found(&lander_id))).into_response();
    };
    if lander.id != "fake-lander-lead-capture" {
        return local_bad_request(
            "Opt-in submit is only available for the fake lead-capture lander",
        );
    }
    let Some(next) = form.next.as_deref() else {
        return local_bad_request("Missing continuation target: next");
    };
    match validate_continuation("next", next, &state.safety_policy) {
        Ok(url) => Redirect::to(url.as_str()).into_response(),
        Err(problem) => local_bad_request(&problem.message()),
    }
}

fn continuation_actions(
    lander: FakeLandingPage,
    params: &HashMap<String, String>,
    safety_policy: &SafetyPolicy,
) -> Result<Vec<ContinuationAction>, ContinuationProblem> {
    let expected_parameters = lander.continuation_parameters();
    let missing_parameters = expected_parameters
        .iter()
        .filter(|parameter| {
            params
                .get(**parameter)
                .map(|value| value.trim().is_empty())
                .unwrap_or(true)
        })
        .map(|parameter| (*parameter).to_string())
        .collect::<Vec<_>>();
    if !missing_parameters.is_empty() {
        return Err(ContinuationProblem::Missing(missing_parameters));
    }

    expected_parameters
        .iter()
        .copied()
        .map(|parameter| {
            let value = params
                .get(parameter)
                .ok_or_else(|| ContinuationProblem::Missing(vec![parameter.to_string()]))?;
            let url = validate_continuation(parameter, value, safety_policy)?;
            Ok(ContinuationAction {
                parameter: parameter.to_string(),
                label: action_label(lander, parameter).to_string(),
                url: url.to_string(),
            })
        })
        .collect()
}

fn validate_continuation(
    parameter: &str,
    value: &str,
    safety_policy: &SafetyPolicy,
) -> Result<Url, ContinuationProblem> {
    let url = Url::parse(value).map_err(|error| ContinuationProblem::Invalid {
        parameter: parameter.to_string(),
        reason: error.to_string(),
    })?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(ContinuationProblem::Invalid {
            parameter: parameter.to_string(),
            reason: "continuation URL must use http or https".to_string(),
        });
    }
    safety_policy
        .ensure_url_allowed(&url)
        .map_err(|error| ContinuationProblem::Blocked {
            parameter: parameter.to_string(),
            reason: error.to_string(),
        })?;
    Ok(url)
}

fn action_label(lander: FakeLandingPage, parameter: &str) -> &'static str {
    match (lander.id, parameter) {
        ("fake-lander-lead-capture", _) => "Submit Fake Opt-In",
        ("fake-lander-advertorial", _) => "Read The Fake Offer Details",
        ("fake-lander-after-optin", _) => "Continue After Fake Opt-In",
        ("fake-lander-multi-cta", "cta1") => "Choose Fake Option A",
        ("fake-lander-multi-cta", "cta2") => "Choose Fake Option B",
        ("fake-lander-multi-cta", "cta3") => "Choose Fake Option C",
        _ => "Continue To Fake Offer",
    }
}

fn local_bad_request(message: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Html(render::local_error(
            "Fake landing page action blocked",
            message,
        )),
    )
        .into_response()
}

enum ContinuationProblem {
    Missing(Vec<String>),
    Invalid { parameter: String, reason: String },
    Blocked { parameter: String, reason: String },
}

impl ContinuationProblem {
    fn message(&self) -> String {
        match self {
            Self::Missing(parameters) => {
                format!("Missing continuation target: {}", parameters.join(", "))
            }
            Self::Invalid { parameter, reason } => {
                format!("Invalid continuation target in {parameter}: {reason}")
            }
            Self::Blocked { parameter, reason } => {
                format!("Blocked continuation target in {parameter}: {reason}")
            }
        }
    }
}
