use std::fs;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{
    HealthResponse, SessionResponse, UPDATE_REQUEST_FILE, UpdateSlot, UpdateStartRequest,
    UpdateStatusResponse,
};
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::header::{CONTENT_TYPE, COOKIE, SET_COOKIE};
use axum::http::{Request, StatusCode};
use campaign_server::config::{ServerConfig, UpdateConfig};
use campaign_server::storage::database::connect_database;
use campaign_server::web::router::build_router;
use tempfile::{TempDir, tempdir};
use tower::ServiceExt;

#[tokio::test]
async fn unauthenticated_update_requests_fail() -> Result<(), Box<dyn std::error::Error>> {
    let harness = ApiHarness::new(true).await?;

    let response = harness
        .app
        .oneshot(
            Request::builder()
                .uri("/api/updates/status")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

#[tokio::test]
async fn unauthenticated_session_reports_first_run_credential_state()
-> Result<(), Box<dyn std::error::Error>> {
    let fresh_harness = ApiHarness::new_with_credentials_reset(false, false).await?;
    let fresh_response = fresh_harness
        .app
        .oneshot(
            Request::builder()
                .uri("/api/auth/session")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(fresh_response.status(), StatusCode::OK);
    let fresh_session: SessionResponse = response_json(fresh_response).await?;
    assert!(!fresh_session.authenticated);
    assert!(fresh_session.must_change_credentials);

    let reset_harness = ApiHarness::new_with_credentials_reset(false, true).await?;
    let reset_response = reset_harness
        .app
        .oneshot(
            Request::builder()
                .uri("/api/auth/session")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(reset_response.status(), StatusCode::OK);
    let reset_session: SessionResponse = response_json(reset_response).await?;
    assert!(!reset_session.authenticated);
    assert!(!reset_session.must_change_credentials);
    Ok(())
}

#[tokio::test]
async fn disabled_updates_report_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let harness = ApiHarness::new(false).await?;
    let cookie = harness.login_cookie().await?;

    let response = harness
        .app
        .oneshot(
            Request::builder()
                .uri("/api/updates/status")
                .header(COOKIE, cookie)
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let status: UpdateStatusResponse = response_json(response).await?;
    assert!(!status.enabled);
    assert_eq!(status.phase, ad_buy_engine_domain::UpdatePhase::Disabled);
    Ok(())
}

#[tokio::test]
async fn confirmed_install_writes_one_request() -> Result<(), Box<dyn std::error::Error>> {
    let harness = ApiHarness::new(true).await?;
    let cookie = harness.login_cookie().await?;
    let request = UpdateStartRequest {
        current_password: "admin".to_string(),
        confirmation: "INSTALL".to_string(),
        requested_version: Some("v0.2.0".to_string()),
    };

    let first = harness
        .app
        .clone()
        .oneshot(json_request("/api/updates/start", &cookie, &request)?)
        .await?;
    let second = harness
        .app
        .clone()
        .oneshot(json_request("/api/updates/start", &cookie, &request)?)
        .await?;

    assert_eq!(first.status(), StatusCode::OK);
    assert_eq!(second.status(), StatusCode::CONFLICT);
    assert!(harness.control_dir().join(UPDATE_REQUEST_FILE).exists());
    Ok(())
}

#[tokio::test]
async fn health_reports_version_schema_and_slot() -> Result<(), Box<dyn std::error::Error>> {
    let harness = ApiHarness::new(true).await?;

    let response = harness
        .app
        .oneshot(Request::builder().uri("/api/health").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let health: HealthResponse = response_json(response).await?;
    assert!(health.ok);
    assert!(health.ready);
    assert_eq!(health.app_version, "test");
    assert_eq!(health.active_slot, Some(UpdateSlot::Blue));
    assert!(health.schema_version >= 1);
    Ok(())
}

struct ApiHarness {
    app: Router,
    tempdir: TempDir,
}

impl ApiHarness {
    async fn new(update_enabled: bool) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_credentials_reset(update_enabled, true).await
    }

    async fn new_with_credentials_reset(
        update_enabled: bool,
        credentials_reset: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tempdir = tempdir()?;
        let dashboard_dist = tempdir.path().join("dist");
        fs::create_dir(&dashboard_dist)?;
        fs::write(dashboard_dist.join("index.html"), "<main>dashboard</main>")?;
        let config = test_config(
            &tempdir.path().join("ad_buy_engine.sqlite3"),
            dashboard_dist,
            tempdir.path(),
            update_enabled,
        );
        let pool = connect_database(&config).await?;
        if credentials_reset {
            sqlx::query("UPDATE operator_credentials SET must_change_credentials = 0 WHERE id = 1")
                .execute(&pool)
                .await?;
        }
        let app = build_router(config, pool).await?;
        Ok(Self { app, tempdir })
    }

    async fn login_cookie(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self
            .app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/login")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"username":"admin","password":"admin"}"#))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let cookie = response
            .headers()
            .get(SET_COOKIE)
            .ok_or("login response did not set a cookie")?
            .to_str()?
            .split(';')
            .next()
            .ok_or("cookie header was empty")?
            .to_string();
        Ok(cookie)
    }

    fn control_dir(&self) -> PathBuf {
        self.tempdir.path().join("update_control")
    }
}

fn json_request<T: serde::Serialize>(
    uri: &str,
    cookie: &str,
    value: &T,
) -> Result<Request<Body>, Box<dyn std::error::Error>> {
    Ok(Request::builder()
        .method("POST")
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(COOKIE, cookie)
        .body(Body::from(serde_json::to_vec(value)?))?)
}

async fn response_json<T: serde::de::DeserializeOwned>(
    response: axum::response::Response,
) -> Result<T, Box<dyn std::error::Error>> {
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn test_config(
    database_path: &Path,
    dashboard_dist: PathBuf,
    data_dir: &Path,
    update_enabled: bool,
) -> ServerConfig {
    ServerConfig {
        database_url: format!("sqlite://{}", database_path.display()),
        tracking_base_url: "https://track.test".to_string(),
        tracking_base_url_override: None,
        admin_dashboard_base_url: "https://admin.test".to_string(),
        admin_dashboard_base_url_override: None,
        public_base_url: "http://127.0.0.1:8088".to_string(),
        listen_address: "127.0.0.1:0".to_string(),
        dashboard_dist,
        app_version: "test".to_string(),
        maxmind_account_id: String::new(),
        maxmind_license_key: String::new(),
        geolite_city_database_path: data_dir.join("GeoLite2-City.mmdb").display().to_string(),
        geolite_country_database_path: data_dir.join("GeoLite2-Country.mmdb").display().to_string(),
        geolite_asn_database_path: data_dir.join("GeoLite2-ASN.mmdb").display().to_string(),
        demo_seed_fake_affiliate_network: false,
        fake_affiliate_network_base_url: "http://127.0.0.1:8090".to_string(),
        updates: UpdateConfig {
            enabled: update_enabled,
            control_dir: data_dir.join("update_control"),
            repo: "john-says-hi/ad_buy_engine".to_string(),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            active_slot: Some(UpdateSlot::Blue),
        },
    }
}
