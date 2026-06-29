use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::Router;
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::get;
use clap::Parser;
use fake_traffic_generator::cli::Cli;
use fake_traffic_generator::config::RunConfig;

#[derive(Clone)]
struct FlowState {
    base_url: String,
    postbacks: Arc<Mutex<Vec<HashMap<String, String>>>>,
}

#[derive(Clone)]
struct SlowState {
    active: Arc<AtomicUsize>,
    max_active: Arc<AtomicUsize>,
}

struct TestServer {
    base_url: String,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
    }
}

async fn spawn_server(
    build_router: impl FnOnce(String) -> Router + Send + 'static,
) -> Result<TestServer, Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let base_url = format!("http://{address}");
    let router = build_router(base_url.clone());
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(async move {
        let server = axum::serve(listener, router).with_graceful_shutdown(async {
            let _ = shutdown_rx.await;
        });
        let _ = server.await;
    });

    Ok(TestServer {
        base_url,
        shutdown: Some(shutdown_tx),
    })
}

fn config_from(args: &[&str]) -> Result<RunConfig, Box<dyn std::error::Error>> {
    let cli = Cli::try_parse_from(std::iter::once("abe-fake-traffic").chain(args.iter().copied()))?;
    Ok(RunConfig::try_from(cli)?)
}

#[tokio::test]
async fn redirect_walker_follows_local_go_and_sends_conversion()
-> Result<(), Box<dyn std::error::Error>> {
    let postbacks = Arc::new(Mutex::new(Vec::new()));
    let server_postbacks = Arc::clone(&postbacks);
    let server = spawn_server(move |base_url| {
        let state = FlowState {
            base_url,
            postbacks: server_postbacks,
        };
        Router::new()
            .route("/api/health", get(|| async { "ok" }))
            .route("/c/campaign-1", get(campaign_redirect_with_go))
            .route("/go/visit-1/1", get(go_redirect_to_external_offer))
            .route("/postback", get(record_postback))
            .with_state(state)
    })
    .await?;
    let campaign_url = format!("{}/c/campaign-1", server.base_url);
    let config = config_from(&[
        "--campaign-url",
        &campaign_url,
        "--interval-ms",
        "0",
        "--conversion-rate",
        "1",
        "--conversion-type",
        "lead",
    ])?;

    let summary = fake_traffic_generator::run(config).await?;

    assert_eq!(summary.completed_sessions, 1);
    assert_eq!(summary.conversions.attempted, 1);
    assert_eq!(summary.conversions.sent, 1);
    assert_eq!(summary.conversions.skipped_no_visit_id, 0);
    assert_eq!(summary.redirects.blocked, 1);
    assert_eq!(summary.http.requests, 4);
    let captured_postbacks = postbacks
        .lock()
        .map(|items| items.clone())
        .unwrap_or_default();
    assert_eq!(captured_postbacks.len(), 1);
    assert_eq!(
        captured_postbacks
            .first()
            .and_then(|params| params.get("cid")),
        Some(&"visit-1".to_string())
    );
    Ok(())
}

#[tokio::test]
async fn redirect_loops_stop_before_repeating_request() -> Result<(), Box<dyn std::error::Error>> {
    let server = spawn_server(|base_url| {
        Router::new()
            .route("/api/health", get(|| async { "ok" }))
            .route(
                "/c/loop",
                get(move || async move {
                    Redirect::temporary(&format!("{base_url}/go/visit-loop/1"))
                }),
            )
            .route(
                "/go/visit-loop/1",
                get(|| async { Redirect::temporary("/go/visit-loop/1") }),
            )
    })
    .await?;
    let campaign_url = format!("{}/c/loop", server.base_url);
    let config = config_from(&["--campaign-url", &campaign_url, "--interval-ms", "0"])?;

    let summary = fake_traffic_generator::run(config).await?;

    assert_eq!(summary.completed_sessions, 1);
    assert_eq!(summary.failed_sessions, 1);
    assert_eq!(summary.http.error_buckets.get("redirect_loop"), Some(&1));
    Ok(())
}

#[tokio::test]
async fn conversion_is_skipped_when_no_visit_id_was_found() -> Result<(), Box<dyn std::error::Error>>
{
    let postbacks = Arc::new(Mutex::new(Vec::new()));
    let server_postbacks = Arc::clone(&postbacks);
    let server = spawn_server(move |_base_url| {
        let state = FlowState {
            base_url: String::new(),
            postbacks: server_postbacks,
        };
        Router::new()
            .route("/api/health", get(|| async { "ok" }))
            .route(
                "/c/no-go",
                get(|| async { Redirect::temporary("https://offer.example/final") }),
            )
            .route("/postback", get(record_postback))
            .with_state(state)
    })
    .await?;
    let campaign_url = format!("{}/c/no-go", server.base_url);
    let config = config_from(&[
        "--campaign-url",
        &campaign_url,
        "--interval-ms",
        "0",
        "--conversion-rate",
        "1",
        "--conversion-type",
        "sale",
    ])?;

    let summary = fake_traffic_generator::run(config).await?;

    assert_eq!(summary.conversions.attempted, 1);
    assert_eq!(summary.conversions.sent, 0);
    assert_eq!(summary.conversions.skipped_no_visit_id, 1);
    assert_eq!(summary.redirects.blocked, 1);
    let captured_postbacks = postbacks
        .lock()
        .map(|items| items.clone())
        .unwrap_or_default();
    assert!(captured_postbacks.is_empty());
    Ok(())
}

#[tokio::test]
async fn run_respects_configured_concurrency() -> Result<(), Box<dyn std::error::Error>> {
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    let server_active = Arc::clone(&active);
    let server_max_active = Arc::clone(&max_active);
    let server = spawn_server(move |_base_url| {
        let state = SlowState {
            active: server_active,
            max_active: server_max_active,
        };
        Router::new()
            .route("/api/health", get(|| async { "ok" }))
            .route("/c/slow", get(slow_campaign))
            .with_state(state)
    })
    .await?;
    let campaign_url = format!("{}/c/slow", server.base_url);
    let config = config_from(&[
        "--campaign-url",
        &campaign_url,
        "--users",
        "4",
        "--sessions",
        "1",
        "--interval-ms",
        "0",
        "--concurrency",
        "2",
    ])?;

    let summary = fake_traffic_generator::run(config).await?;

    assert_eq!(summary.completed_sessions, 4);
    assert!(max_active.load(Ordering::SeqCst) <= 2);
    Ok(())
}

async fn campaign_redirect_with_go(State(state): State<FlowState>) -> Redirect {
    let go_url = format!("{}/go/visit-1/1", state.base_url);
    let location = format!(
        "https://lander.example/path?go={}",
        urlencoding::encode(&go_url)
    );
    Redirect::temporary(&location)
}

async fn go_redirect_to_external_offer() -> Redirect {
    Redirect::temporary("https://offer.example/final?cid=visit-1")
}

async fn record_postback(
    State(state): State<FlowState>,
    Query(params): Query<HashMap<String, String>>,
) -> &'static str {
    if let Ok(mut postbacks) = state.postbacks.lock() {
        postbacks.push(params);
    }
    "ok"
}

async fn slow_campaign(State(state): State<SlowState>) -> &'static str {
    let current = state.active.fetch_add(1, Ordering::SeqCst) + 1;
    update_max_active(&state.max_active, current);
    tokio::time::sleep(Duration::from_millis(50)).await;
    state.active.fetch_sub(1, Ordering::SeqCst);
    "ok"
}

fn update_max_active(max_active: &AtomicUsize, current: usize) {
    let mut observed = max_active.load(Ordering::SeqCst);
    while current > observed {
        match max_active.compare_exchange(observed, current, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => break,
            Err(next_observed) => observed = next_observed,
        }
    }
}
