use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{
    CampaignDraft, DestinationType, FunnelDraft, FunnelPath, FunnelSequence, OfferDraft,
    OfferSourceDraft, SequenceType, TrafficSourceDraft, UrlToken, WeightedReference,
};
use axum::Router;
use campaign_server::config::{ServerConfig, UpdateConfig};
use campaign_server::storage::database::connect_database;
use campaign_server::storage::entities::{
    create_campaign, create_funnel, create_offer, create_offer_source, create_traffic_source,
};
use campaign_server::web::router::build_router as build_campaign_router;
use fake_landing_page_server::build_router as build_fake_lander_router;
use fake_landing_page_server::cli::Cli;
use fake_landing_page_server::config::RunConfig;
use sqlx::SqlitePool;
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

struct FlowFixture {
    _tempdir: tempfile::TempDir,
    _campaign_server: RunningServer,
    _fake_lander_server: RunningServer,
    pool: SqlitePool,
    client: reqwest::Client,
    campaign_base_url: String,
    fake_lander_base_url: String,
    fake_offer_base_url: String,
    traffic_source_id: String,
    offer_id: String,
}

impl FlowFixture {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let tempdir = tempdir()?;
        let (campaign_listener, _campaign_address, campaign_base_url) = bind_server().await?;
        let (fake_lander_listener, fake_lander_address, fake_lander_base_url) =
            bind_server().await?;
        let dashboard_dist = tempdir.path().join("dist");
        fs::create_dir(&dashboard_dist)?;
        fs::write(dashboard_dist.join("index.html"), "<main>dashboard</main>")?;

        let config = test_config(
            &tempdir.path().join("ad_buy_engine.sqlite3"),
            dashboard_dist,
            tempdir.path(),
            &campaign_base_url,
            &fake_lander_base_url,
        );
        let pool = connect_database(&config).await?;
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
        let offer_source = create_offer_source(
            &pool,
            OfferSourceDraft {
                name: "Fake Affiliate Style Source".to_string(),
                tokens: default_tokens(),
                tracking_domain: "127.0.0.1".to_string(),
                tracking_method: "postback".to_string(),
                payout_currency: "USD".to_string(),
                postback_url: String::new(),
                append_click_id: true,
                accept_duplicate_postbacks: false,
                whitelist_postback_ips: Vec::new(),
                referrer_handling: "do_nothing".to_string(),
                notes: String::new(),
            },
        )
        .await?;
        let fake_offer_base_url = "http://127.0.0.1:8090".to_string();
        let offer = create_offer(
            &pool,
            OfferDraft {
                offer_source_id: offer_source.id,
                country: "Global".to_string(),
                name: "Fake Affiliate Style Offer".to_string(),
                tags: vec!["fake-affiliate-network".to_string()],
                url: format!("{fake_offer_base_url}/click/fake-flow-offer?subid={{clickid}}"),
                url_tokens: default_tokens(),
                payout_model: "fixed".to_string(),
                payout_value: 1.0,
                currency: "USD".to_string(),
                language: "en".to_string(),
                vertical: "fake-flow".to_string(),
                weight: 100,
                notes: String::new(),
            },
        )
        .await?;

        let campaign_router = build_campaign_router(config, pool.clone()).await?;
        let fake_lander_router = build_fake_lander_router(fake_lander_config(fake_lander_address)?);
        let campaign_server = spawn_bound_server(campaign_listener, campaign_router);
        let fake_lander_server = spawn_bound_server(fake_lander_listener, fake_lander_router);
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        Ok(Self {
            _tempdir: tempdir,
            _campaign_server: campaign_server,
            _fake_lander_server: fake_lander_server,
            pool,
            client,
            campaign_base_url,
            fake_lander_base_url,
            fake_offer_base_url,
            traffic_source_id: traffic_source.id,
            offer_id: offer.id,
        })
    }

    async fn create_campaign_for_paths(
        &self,
        name: &str,
        paths: Vec<FunnelPath>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let funnel = create_funnel(
            &self.pool,
            FunnelDraft {
                country: "Global".to_string(),
                name: format!("{name} Funnel"),
                redirect_handling: "default".to_string(),
                referrer_handling: "do_nothing".to_string(),
                conditional_sequences: Vec::new(),
                default_sequences: vec![FunnelSequence {
                    id: "default".to_string(),
                    name: "Default".to_string(),
                    active: true,
                    weight: 100,
                    sequence_type: SequenceType::Matrix,
                    conditions: Vec::new(),
                    paths,
                }],
                notes: String::new(),
            },
        )
        .await?;
        let campaign = create_campaign(
            &self.pool,
            &self.campaign_base_url,
            CampaignDraft {
                traffic_source_id: self.traffic_source_id.clone(),
                destination_type: DestinationType::Funnel,
                funnel_id: Some(funnel.id),
                direct_sequence: None,
                cost_model: "CPC".to_string(),
                cost_value: 0.0,
                country: "Global".to_string(),
                name: name.to_string(),
                notes: String::new(),
            },
        )
        .await?;
        Ok(campaign.id)
    }
}

#[tokio::test]
async fn advertorial_page_click_records_lander_click_and_routes_to_fake_offer()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = FlowFixture::new().await?;
    let campaign_id = fixture
        .create_campaign_for_paths(
            "Fake Advertorial Campaign",
            vec![path_to_offer(
                "advertorial",
                "fake-lander-advertorial",
                &fixture.offer_id,
            )],
        )
        .await?;

    let lander_url =
        campaign_redirect(&fixture.client, &fixture.campaign_base_url, &campaign_id).await?;
    assert!(lander_url.contains("/lander/fake-lander-advertorial?next="));
    let body = fixture.client.get(&lander_url).send().await?.text().await?;
    let cta_url = html_attr_after(
        &body,
        "class=\"cta primary\" data-continuation=\"next\" href=\"",
    )?;
    assert!(cta_url.starts_with(&format!("{}/go/", fixture.campaign_base_url)));

    let offer_redirect = temporary_redirect(&fixture.client, &cta_url).await?;

    assert!(offer_redirect.starts_with(&fixture.fake_offer_base_url));
    assert_lander_click_count(&fixture.pool, 1).await?;
    Ok(())
}

#[tokio::test]
async fn lead_capture_submit_follows_go_without_creating_conversion()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = FlowFixture::new().await?;
    let campaign_id = fixture
        .create_campaign_for_paths(
            "Fake Lead Capture Campaign",
            vec![path_to_offer(
                "lead",
                "fake-lander-lead-capture",
                &fixture.offer_id,
            )],
        )
        .await?;

    let lander_url =
        campaign_redirect(&fixture.client, &fixture.campaign_base_url, &campaign_id).await?;
    assert!(lander_url.contains("/lander/fake-lander-lead-capture?next="));
    let body = fixture.client.get(&lander_url).send().await?.text().await?;
    let next = html_attr_after(&body, "name=\"next\" type=\"hidden\" value=\"")?;
    let submit_url = format!(
        "{}/lander/fake-lander-lead-capture/opt-in",
        fixture.fake_lander_base_url
    );
    let opt_in_redirect = fixture
        .client
        .post(submit_url)
        .form(&[("email", "person@example.test"), ("next", next.as_str())])
        .send()
        .await?;

    assert_eq!(opt_in_redirect.status(), reqwest::StatusCode::SEE_OTHER);
    let go_url = response_location(&opt_in_redirect)?;
    assert_eq!(go_url, next);
    let final_redirect = temporary_redirect(&fixture.client, &go_url).await?;

    assert!(final_redirect.starts_with(&fixture.fake_offer_base_url));
    assert_lander_click_count(&fixture.pool, 1).await?;
    assert_eq!(conversion_count(&fixture.pool).await?, 0);
    Ok(())
}

#[tokio::test]
async fn multi_cta_second_choice_uses_second_continuation_slot()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = FlowFixture::new().await?;
    let campaign_id = fixture
        .create_campaign_for_paths(
            "Fake Multi CTA Campaign",
            vec![path_to_offer(
                "multi",
                "fake-lander-multi-cta",
                &fixture.offer_id,
            )],
        )
        .await?;

    let lander_url =
        campaign_redirect(&fixture.client, &fixture.campaign_base_url, &campaign_id).await?;
    let body = fixture.client.get(&lander_url).send().await?.text().await?;
    let cta2_url = html_attr_after(&body, "data-continuation=\"cta2\" href=\"")?;
    let (_visit_id, slot) = route_parts(&cta2_url)?;

    let final_redirect = temporary_redirect(&fixture.client, &cta2_url).await?;

    assert_eq!(slot, 2);
    assert!(final_redirect.starts_with(&fixture.fake_offer_base_url));
    assert_lander_click_slot(&fixture.pool, 2).await?;
    Ok(())
}

#[tokio::test]
async fn nested_fake_lead_to_advertorial_flow_preserves_visit_id()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = FlowFixture::new().await?;
    let campaign_id = fixture
        .create_campaign_for_paths(
            "Nested Fake Lander Campaign",
            vec![FunnelPath {
                id: "lead".to_string(),
                weight: 100,
                landing_page_id: Some("fake-lander-lead-capture".to_string()),
                offers: Vec::new(),
                children: vec![FunnelPath {
                    id: "advertorial".to_string(),
                    weight: 100,
                    landing_page_id: Some("fake-lander-advertorial".to_string()),
                    offers: Vec::new(),
                    children: vec![path_to_offer("offer", "", &fixture.offer_id)],
                }],
            }],
        )
        .await?;

    let lead_url =
        campaign_redirect(&fixture.client, &fixture.campaign_base_url, &campaign_id).await?;
    let lead_body = fixture.client.get(&lead_url).send().await?.text().await?;
    let first_go = html_attr_after(&lead_body, "name=\"next\" type=\"hidden\" value=\"")?;
    let (visit_id, first_slot) = route_parts(&first_go)?;

    let advertorial_url = temporary_redirect(&fixture.client, &first_go).await?;
    let advertorial_body = fixture
        .client
        .get(&advertorial_url)
        .send()
        .await?
        .text()
        .await?;
    let second_go = html_attr_after(
        &advertorial_body,
        "class=\"cta primary\" data-continuation=\"next\" href=\"",
    )?;
    let (second_visit_id, second_slot) = route_parts(&second_go)?;
    let final_redirect = temporary_redirect(&fixture.client, &second_go).await?;

    assert_eq!(visit_id, second_visit_id);
    assert_ne!(first_slot, second_slot);
    assert!(first_slot > 0);
    assert!(second_slot > 0);
    assert!(advertorial_url.contains("/lander/fake-lander-advertorial?next="));
    assert!(final_redirect.starts_with(&fixture.fake_offer_base_url));
    assert_lander_click_count(&fixture.pool, 2).await?;
    Ok(())
}

#[tokio::test]
async fn unsafe_fake_page_continuation_does_not_create_lander_click()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = FlowFixture::new().await?;
    let response = fixture
        .client
        .get(format!(
            "{}/lander/fake-lander-standard-click-through?next=https%3A%2F%2Fpublic.example%2Fgo",
            fixture.fake_lander_base_url
        ))
        .send()
        .await?;

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
    assert_lander_click_count(&fixture.pool, 0).await?;
    Ok(())
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

fn fake_lander_config(
    listen_address: SocketAddr,
) -> Result<RunConfig, fake_landing_page_server::config::ConfigError> {
    RunConfig::try_from(Cli {
        listen_address: listen_address.to_string(),
        allow_host: Vec::new(),
        allow_private_network: false,
    })
}

fn test_config(
    database_path: &Path,
    dashboard_dist: PathBuf,
    data_dir: &Path,
    tracking_base_url: &str,
    fake_landing_page_base_url: &str,
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
        demo_seed_fake_affiliate_network: false,
        fake_affiliate_network_base_url: "http://127.0.0.1:8090".to_string(),
        demo_seed_fake_landing_pages: true,
        fake_landing_page_base_url: fake_landing_page_base_url.to_string(),
        updates: UpdateConfig {
            enabled: false,
            control_dir: data_dir.join("update_control"),
            repo: "john-says-hi/ad_buy_engine".to_string(),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            active_slot: None,
        },
    }
}

fn path_to_offer(path_id: &str, landing_page_id: &str, offer_id: &str) -> FunnelPath {
    FunnelPath {
        id: path_id.to_string(),
        weight: 100,
        landing_page_id: if landing_page_id.is_empty() {
            None
        } else {
            Some(landing_page_id.to_string())
        },
        offers: vec![WeightedReference {
            id: offer_id.to_string(),
            weight: 100,
        }],
        children: Vec::new(),
    }
}

fn default_tokens() -> Vec<UrlToken> {
    vec![UrlToken {
        name: "clickid".to_string(),
        token: "{clickid}".to_string(),
    }]
}

async fn campaign_redirect(
    client: &reqwest::Client,
    campaign_base_url: &str,
    campaign_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    temporary_redirect(
        client,
        &format!("{campaign_base_url}/c/{campaign_id}?subid=fake-user"),
    )
    .await
}

async fn temporary_redirect(
    client: &reqwest::Client,
    url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;
    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    response_location(&response)
}

fn response_location(response: &reqwest::Response) -> Result<String, Box<dyn std::error::Error>> {
    Ok(response
        .headers()
        .get(reqwest::header::LOCATION)
        .ok_or_else(|| std::io::Error::other("redirect location missing"))?
        .to_str()?
        .to_string())
}

fn html_attr_after(body: &str, marker: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tail = body
        .split(marker)
        .nth(1)
        .ok_or_else(|| std::io::Error::other("expected HTML attribute marker"))?;
    let value = tail
        .split('"')
        .next()
        .ok_or_else(|| std::io::Error::other("expected HTML attribute value"))?;
    Ok(unescape_html_attr(value))
}

fn unescape_html_attr(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn route_parts(go_url: &str) -> Result<(String, u8), Box<dyn std::error::Error>> {
    let tail = go_url
        .split("/go/")
        .nth(1)
        .ok_or_else(|| std::io::Error::other("visit id missing from click URL"))?;
    let mut parts = tail.split('/');
    let visit_id = parts
        .next()
        .ok_or_else(|| std::io::Error::other("visit id missing from click URL"))?
        .to_string();
    let slot = parts
        .next()
        .and_then(|slot| slot.parse::<u8>().ok())
        .ok_or_else(|| std::io::Error::other("slot missing from click URL"))?;
    Ok((visit_id, slot))
}

async fn assert_lander_click_count(
    pool: &SqlitePool,
    expected: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM visit_events WHERE event_type = 'lander_click'")
            .fetch_one(pool)
            .await?;
    assert_eq!(count, expected);
    Ok(())
}

async fn assert_lander_click_slot(
    pool: &SqlitePool,
    expected_slot: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_data: String = sqlx::query_scalar(
        "SELECT event_data_json FROM visit_events WHERE event_type = 'lander_click'",
    )
    .fetch_one(pool)
    .await?;
    let value: serde_json::Value = serde_json::from_str(&event_data)?;
    assert_eq!(
        value.get("slot").and_then(|slot| slot.as_i64()),
        Some(expected_slot)
    );
    Ok(())
}

async fn conversion_count(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM conversion_events")
        .fetch_one(pool)
        .await
}
