use axum::body::{to_bytes, Body};
use axum::http::header::{COOKIE, LOCATION, SET_COOKIE, USER_AGENT};
use axum::http::{Method, Request, StatusCode};
use click_server::app_state::AppState;
use click_server::config::ClickServerConfig;
use click_server::database::{connect_database, run_migrations};
use click_server::http::router::build_router;
use serde_json::{json, Value};
use std::path::Path;
use tempfile::TempDir;
use tower::ServiceExt;

const SETUP_SECRET: &str = "test-setup-secret";

async fn test_app() -> (axum::Router, TempDir) {
    let temp_dir = tempfile::tempdir().expect("temp dir should be created");
    let database_url = sqlite_url(temp_dir.path().join("click_server.sqlite3").as_path());
    let config = ClickServerConfig {
        bind_addr: "127.0.0.1:0".to_string(),
        database_url,
        setup_secret: SETUP_SECRET.to_string(),
        admin_dist_dir: temp_dir.path().join("dist"),
        cookie_secure: false,
        session_ttl_seconds: 604_800,
        version: "test".to_string(),
    };

    let pool = connect_database(&config)
        .await
        .expect("database should connect");
    run_migrations(&pool).await.expect("migrations should run");

    (build_router(AppState::new(pool, config)), temp_dir)
}

fn sqlite_url(path: &Path) -> String {
    format!("sqlite://{}", path.display())
}

async fn json_request(
    app: axum::Router,
    method: Method,
    uri: &str,
    body: Value,
    cookie: Option<&str>,
) -> (StatusCode, axum::http::HeaderMap, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");

    if let Some(cookie) = cookie {
        builder = builder.header(COOKIE, cookie);
    }

    let response = app
        .oneshot(
            builder
                .body(Body::from(body.to_string()))
                .expect("request should build"),
        )
        .await
        .expect("response should be returned");
    let status = response.status();
    let headers = response.headers().clone();
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body should read");
    let value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));

    (status, headers, value)
}

async fn get_json(
    app: axum::Router,
    uri: &str,
    cookie: Option<&str>,
) -> (StatusCode, axum::http::HeaderMap, Value) {
    let mut builder = Request::builder().method(Method::GET).uri(uri);
    if let Some(cookie) = cookie {
        builder = builder.header(COOKIE, cookie);
    }

    let response = app
        .oneshot(builder.body(Body::empty()).expect("request should build"))
        .await
        .expect("response should be returned");
    let status = response.status();
    let headers = response.headers().clone();
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body should read");
    let value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));

    (status, headers, value)
}

async fn complete_setup(app: axum::Router) {
    let (status, _, body) = json_request(
        app,
        Method::POST,
        "/api/setup/complete",
        json!({
            "setup_secret": SETUP_SECRET,
            "username": "admin",
            "password": "correct horse battery staple",
            "tracking_domain": "track.example.com"
        }),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body:?}");
}

async fn login(app: axum::Router) -> String {
    let (status, headers, body) = json_request(
        app,
        Method::POST,
        "/api/auth/login",
        json!({
            "username": "admin",
            "password": "correct horse battery staple"
        }),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body:?}");
    headers
        .get(SET_COOKIE)
        .expect("set-cookie header should exist")
        .to_str()
        .expect("set-cookie should be valid text")
        .split(';')
        .next()
        .expect("cookie pair should exist")
        .to_string()
}

#[tokio::test]
async fn setup_secret_is_required_and_setup_locks_after_completion() {
    let (app, _temp_dir) = test_app().await;

    let (status, _, _) = get_json(app.clone(), "/api/setup/status", None).await;
    assert_eq!(status, StatusCode::OK);

    let (status, _, _) = json_request(
        app.clone(),
        Method::POST,
        "/api/setup/complete",
        json!({
            "setup_secret": "wrong-secret",
            "username": "admin",
            "password": "correct horse battery staple",
            "tracking_domain": null
        }),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    complete_setup(app.clone()).await;

    let (status, _, body) = get_json(app.clone(), "/api/setup/status", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["setup_complete"], true);

    let (status, _, _) = json_request(
        app,
        Method::POST,
        "/api/setup/complete",
        json!({
            "setup_secret": SETUP_SECRET,
            "username": "admin2",
            "password": "correct horse battery staple",
            "tracking_domain": null
        }),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn admin_can_create_campaign_track_click_and_view_stats() {
    let (app, _temp_dir) = test_app().await;
    complete_setup(app.clone()).await;
    let cookie = login(app.clone()).await;

    let (status, _, campaign) = json_request(
        app.clone(),
        Method::POST,
        "/api/campaigns",
        json!({
            "name": "Summer Search",
            "slug": "summer-search",
            "destination_url": "https://example.com/offer",
            "is_active": true
        }),
        Some(&cookie),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{campaign:?}");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/c/summer-search?kw=blue")
                .header(USER_AGENT, "integration-test")
                .header("x-forwarded-for", "203.0.113.10")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("response should be returned");

    assert_eq!(response.status(), StatusCode::FOUND);
    assert_eq!(
        response.headers().get(LOCATION).expect("location exists"),
        "https://example.com/offer"
    );

    let (status, _, stats) = get_json(app.clone(), "/api/stats/summary", Some(&cookie)).await;
    assert_eq!(status, StatusCode::OK, "{stats:?}");
    assert_eq!(stats["total_clicks"], 1);
    assert_eq!(stats["active_campaigns"], 1);

    let campaign_id = campaign["id"].as_str().expect("campaign id should exist");
    let (status, _, campaign_stats) = get_json(
        app,
        &format!("/api/stats/campaign/{campaign_id}"),
        Some(&cookie),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{campaign_stats:?}");
    assert_eq!(campaign_stats["total_clicks"], 1);
}

#[tokio::test]
async fn campaign_endpoints_require_an_admin_session() {
    let (app, _temp_dir) = test_app().await;
    complete_setup(app.clone()).await;

    let (status, _, _) = get_json(app, "/api/campaigns", None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
