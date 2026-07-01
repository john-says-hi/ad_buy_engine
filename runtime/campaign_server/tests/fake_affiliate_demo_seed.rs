use std::fs;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{FAKE_AFFILIATE_OFFER_SOURCE_ID, fake_affiliate_catalog};
use campaign_server::config::{ServerConfig, UpdateConfig};
use campaign_server::storage::database::connect_database;
use campaign_server::storage::demo::seed_fake_affiliate_network_catalog;
use sqlx::SqlitePool;
use tempfile::tempdir;

#[tokio::test]
async fn default_database_initialization_does_not_seed_fake_network()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(false, "http://127.0.0.1:8090")?;
    let pool = connect_database(&fixture.config).await?;

    assert_eq!(fake_source_count(&pool).await?, 0);
    assert_eq!(fake_offer_count(&pool).await?, 0);
    Ok(())
}

#[tokio::test]
async fn enabled_demo_seed_creates_exactly_one_source_and_five_offers()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(true, "http://127.0.0.1:8099")?;
    let pool = connect_database(&fixture.config).await?;

    assert_eq!(fake_source_count(&pool).await?, 1);
    assert_eq!(fake_offer_count(&pool).await?, 5);
    let seeded_urls: Vec<String> =
        sqlx::query_scalar("SELECT url FROM offers WHERE offer_source_id = ? ORDER BY id")
            .bind(FAKE_AFFILIATE_OFFER_SOURCE_ID)
            .fetch_all(&pool)
            .await?;
    assert_eq!(seeded_urls.len(), fake_affiliate_catalog().len());
    assert!(
        seeded_urls
            .iter()
            .all(|url| url.starts_with("http://127.0.0.1:8099/click/"))
    );
    assert!(
        seeded_urls
            .iter()
            .all(|url| url.ends_with("?subid={clickid}"))
    );
    Ok(())
}

#[tokio::test]
async fn demo_seed_is_idempotent_and_refreshes_only_stable_records()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(true, "http://127.0.0.1:8090")?;
    let pool = connect_database(&fixture.config).await?;
    sqlx::query("UPDATE offers SET name = 'Stale Name' WHERE id = 'fake-lead-solar-savings'")
        .execute(&pool)
        .await?;

    seed_fake_affiliate_network_catalog(&pool, &fixture.config).await?;

    assert_eq!(fake_source_count(&pool).await?, 1);
    assert_eq!(fake_offer_count(&pool).await?, 5);
    let refreshed_name: String =
        sqlx::query_scalar("SELECT name FROM offers WHERE id = 'fake-lead-solar-savings'")
            .fetch_one(&pool)
            .await?;
    assert_eq!(refreshed_name, "Fake Solar Savings Lead");
    Ok(())
}

#[tokio::test]
async fn invalid_fake_network_base_url_fails_before_partial_seed()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(false, "https://public.example")?;
    let pool = connect_database(&fixture.config).await?;

    let result = seed_fake_affiliate_network_catalog(&pool, &fixture.config).await;

    assert!(result.is_err());
    assert_eq!(fake_source_count(&pool).await?, 0);
    assert_eq!(fake_offer_count(&pool).await?, 0);
    Ok(())
}

struct DatabaseFixture {
    _tempdir: tempfile::TempDir,
    config: ServerConfig,
}

impl DatabaseFixture {
    fn new(
        seed_fake_affiliate_network: bool,
        fake_affiliate_network_base_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tempdir = tempdir()?;
        let database_path = tempdir.path().join("ad_buy_engine.sqlite3");
        let dashboard_dist = tempdir.path().join("dist");
        fs::create_dir(&dashboard_dist)?;
        fs::write(dashboard_dist.join("index.html"), "<main>dashboard</main>")?;
        let config = test_config(
            &database_path,
            dashboard_dist,
            tempdir.path(),
            seed_fake_affiliate_network,
            fake_affiliate_network_base_url,
        );
        Ok(Self {
            _tempdir: tempdir,
            config,
        })
    }
}

fn test_config(
    database_path: &Path,
    dashboard_dist: PathBuf,
    data_dir: &Path,
    seed_fake_affiliate_network: bool,
    fake_affiliate_network_base_url: &str,
) -> ServerConfig {
    ServerConfig {
        database_url: format!("sqlite://{}", database_path.display()),
        tracking_base_url: "http://127.0.0.1:8088".to_string(),
        tracking_base_url_override: None,
        admin_dashboard_base_url: "http://127.0.0.1:8088".to_string(),
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
        demo_seed_fake_affiliate_network: seed_fake_affiliate_network,
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

async fn fake_source_count(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM offer_sources WHERE id = ?")
        .bind(FAKE_AFFILIATE_OFFER_SOURCE_ID)
        .fetch_one(pool)
        .await
}

async fn fake_offer_count(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM offers WHERE offer_source_id = ?")
        .bind(FAKE_AFFILIATE_OFFER_SOURCE_ID)
        .fetch_one(pool)
        .await
}
