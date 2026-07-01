use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{CampaignDraft, DestinationType, FunnelSequence, TrafficSourceDraft};
use axum::Router;
use campaign_server::config::{ServerConfig, UpdateConfig};
use campaign_server::storage::dashboard::dashboard_summary;
use campaign_server::storage::database::connect_database;
use campaign_server::storage::date_filter::VisitDateFilter;
use campaign_server::storage::entities::{create_campaign, create_traffic_source};
use campaign_server::web::router::build_router as build_campaign_router;
use fake_affiliate_network::build_router as build_fake_network_router;
use fake_affiliate_network::cli::Cli;
use fake_affiliate_network::config::RunConfig;
use sqlx::{Row, SqlitePool};
use tempfile::tempdir;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

struct RunningServer {
    shutdown: Option<oneshot::Sender<()>>,
}

impl Drop for RunningServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _send_result = shutdown.send(());
        }
    }
}

async fn bind_server() -> Result<(TcpListener, SocketAddr, String), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let base_url = format!("http://{address}");
    Ok((listener, address, base_url))
}

fn spawn_bound_server(listener: TcpListener, router: Router) -> RunningServer {
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        let server = axum::serve(listener, router).with_graceful_shutdown(async {
            let _result = shutdown_rx.await;
        });
        let _result = server.await;
    });
    RunningServer {
        shutdown: Some(shutdown_tx),
    }
}

#[tokio::test]
async fn fake_network_drives_ad_buy_engine_lead_and_sale_attribution()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let (campaign_listener, _campaign_address, campaign_base_url) = bind_server().await?;
    let (fake_listener, fake_address, fake_base_url) = bind_server().await?;
    let dashboard_dist = tempdir.path().join("dist");
    fs::create_dir(&dashboard_dist)?;
    fs::write(dashboard_dist.join("index.html"), "<main>dashboard</main>")?;

    let campaign_config = test_config(
        &tempdir.path().join("ad_buy_engine.sqlite3"),
        dashboard_dist,
        tempdir.path(),
        &campaign_base_url,
        &fake_base_url,
    );
    let pool = connect_database(&campaign_config).await?;
    let traffic_source = create_traffic_source(
        &pool,
        TrafficSourceDraft {
            name: "Local Fake Traffic".to_string(),
            external_id_parameter: "subid".to_string(),
            cost_parameter: String::new(),
            custom_parameters: Vec::new(),
            currency: "USD".to_string(),
            postback_urls: Vec::new(),
            pixel_url: String::new(),
            track_impressions: false,
            direct_tracking: true,
            notes: String::new(),
        },
    )
    .await?;
    let lead_campaign = create_campaign(
        &pool,
        &campaign_config.tracking_base_url,
        CampaignDraft {
            traffic_source_id: traffic_source.id.clone(),
            destination_type: DestinationType::DirectSequence,
            funnel_id: None,
            direct_sequence: Some(FunnelSequence::default_offer("fake-lead-solar-savings")),
            cost_model: "CPC".to_string(),
            cost_value: 0.0,
            country: "Global".to_string(),
            name: "Fake Lead Campaign".to_string(),
            notes: String::new(),
        },
    )
    .await?;
    let sale_campaign = create_campaign(
        &pool,
        &campaign_config.tracking_base_url,
        CampaignDraft {
            traffic_source_id: traffic_source.id,
            destination_type: DestinationType::DirectSequence,
            funnel_id: None,
            direct_sequence: Some(FunnelSequence::default_offer("fake-sale-course-bundle")),
            cost_model: "CPC".to_string(),
            cost_value: 0.0,
            country: "Global".to_string(),
            name: "Fake Sale Campaign".to_string(),
            notes: String::new(),
        },
    )
    .await?;

    let campaign_router = build_campaign_router(campaign_config.clone(), pool.clone()).await?;
    let fake_config = fake_network_config(fake_address, &campaign_base_url)?;
    let fake_router = build_fake_network_router(fake_config)?;
    let _campaign_server = spawn_bound_server(campaign_listener, campaign_router);
    let _fake_server = spawn_bound_server(fake_listener, fake_router);
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    request_campaign_clicks(&client, &campaign_base_url, &lead_campaign.id, 2).await?;
    request_campaign_clicks(&client, &campaign_base_url, &sale_campaign.id, 2).await?;

    let conversions = conversion_rows(&pool).await?;
    assert_eq!(conversions.len(), 2);
    assert!(conversions.iter().any(|conversion| {
        conversion.event_type_id == "lead"
            && conversion.revenue_value == 0.0
            && conversion
                .transaction_id
                .contains("fake-lead-solar-savings")
    }));
    assert!(conversions.iter().any(|conversion| {
        conversion.event_type_id == "sale"
            && conversion.revenue_value == 49.0
            && conversion
                .transaction_id
                .contains("fake-sale-course-bundle")
    }));
    assert!(
        conversions
            .iter()
            .all(|conversion| conversion.visit_id.is_some())
    );

    let summary = dashboard_summary(&pool, VisitDateFilter::default()).await?;
    assert_eq!(kpi_value(&summary.kpis, "conversions"), Some(2.0));
    assert_eq!(kpi_value(&summary.kpis, "revenue"), Some(49.0));
    Ok(())
}

async fn request_campaign_clicks(
    client: &reqwest::Client,
    campaign_base_url: &str,
    campaign_id: &str,
    count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..count {
        let response = client
            .get(format!("{campaign_base_url}/c/{campaign_id}"))
            .send()
            .await?;
        assert!(response.status().is_success());
    }
    Ok(())
}

#[derive(Debug)]
struct ConversionRow {
    event_type_id: String,
    revenue_value: f64,
    transaction_id: String,
    visit_id: Option<String>,
}

async fn conversion_rows(pool: &SqlitePool) -> Result<Vec<ConversionRow>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT event_type_id, revenue_value, transaction_id, visit_id
         FROM conversion_events
         WHERE duplicate = 0
         ORDER BY created_at_millis",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| {
            Ok(ConversionRow {
                event_type_id: row.try_get("event_type_id")?,
                revenue_value: row.try_get("revenue_value")?,
                transaction_id: row
                    .try_get::<Option<String>, _>("transaction_id")?
                    .unwrap_or_default(),
                visit_id: row.try_get("visit_id")?,
            })
        })
        .collect()
}

fn fake_network_config(
    listen_address: SocketAddr,
    campaign_base_url: &str,
) -> Result<RunConfig, fake_affiliate_network::config::ConfigError> {
    RunConfig::try_from(Cli {
        listen_address: listen_address.to_string(),
        postback_template: format!(
            "{}/postback?cid={{click_id}}&type={{event_type}}&payout={{payout}}&currency={{currency}}&status={{status}}&txid={{transaction_id}}",
            campaign_base_url.trim_end_matches('/')
        ),
        lead_threshold: 2,
        sale_threshold: 2,
        request_timeout_seconds: 5,
        allow_host: Vec::new(),
        allow_private_network: false,
    })
}

fn test_config(
    database_path: &Path,
    dashboard_dist: PathBuf,
    data_dir: &Path,
    tracking_base_url: &str,
    fake_affiliate_network_base_url: &str,
) -> ServerConfig {
    ServerConfig {
        database_url: format!("sqlite://{}", database_path.display()),
        tracking_base_url: tracking_base_url.to_string(),
        tracking_base_url_override: None,
        admin_dashboard_base_url: tracking_base_url.to_string(),
        admin_dashboard_base_url_override: None,
        public_base_url: tracking_base_url.to_string(),
        listen_address: "127.0.0.1:0".to_string(),
        dashboard_dist,
        app_version: "test".to_string(),
        maxmind_account_id: String::new(),
        maxmind_license_key: String::new(),
        geolite_city_database_path: data_dir.join("GeoLite2-City.mmdb").display().to_string(),
        geolite_country_database_path: data_dir.join("GeoLite2-Country.mmdb").display().to_string(),
        geolite_asn_database_path: data_dir.join("GeoLite2-ASN.mmdb").display().to_string(),
        demo_seed_fake_affiliate_network: true,
        fake_affiliate_network_base_url: fake_affiliate_network_base_url.to_string(),
        demo_seed_fake_landing_pages: false,
        fake_landing_page_base_url: "http://127.0.0.1:8091".to_string(),
        updates: UpdateConfig {
            enabled: false,
            control_dir: data_dir.join("update_control"),
            repo: "john-says-hi/ad_buy_engine".to_string(),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            active_slot: None,
        },
    }
}

fn kpi_value(kpis: &[ad_buy_engine_domain::DashboardKpi], key: &str) -> Option<f64> {
    kpis.iter().find(|kpi| kpi.key == key).map(|kpi| kpi.value)
}
