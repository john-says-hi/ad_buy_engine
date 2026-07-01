use std::fs;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{fake_landing_page_catalog, fake_landing_page_url};
use campaign_server::config::{ServerConfig, UpdateConfig};
use campaign_server::storage::database::connect_database;
use campaign_server::storage::demo::seed_fake_landing_page_catalog;
use sqlx::{Row, SqlitePool};
use tempfile::tempdir;

#[tokio::test]
async fn default_database_initialization_does_not_seed_fake_landers()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(false, "http://127.0.0.1:8091")?;
    let pool = connect_database(&fixture.config).await?;

    assert_eq!(fake_lander_count(&pool).await?, 0);
    Ok(())
}

#[tokio::test]
async fn enabled_demo_seed_creates_exactly_five_matching_fake_landers()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(true, "http://127.0.0.1:8191")?;
    let pool = connect_database(&fixture.config).await?;

    let rows = fake_lander_rows(&pool).await?;
    assert_eq!(rows.len(), fake_landing_page_catalog().len());
    for catalog_lander in fake_landing_page_catalog() {
        let row = rows
            .iter()
            .find(|row| row.id == catalog_lander.id)
            .ok_or_else(|| std::io::Error::other("missing fake lander row"))?;
        assert_eq!(row.name, catalog_lander.name);
        assert_eq!(
            row.url,
            fake_landing_page_url("http://127.0.0.1:8191", *catalog_lander)
        );
        assert_eq!(row.cta_count, i64::from(catalog_lander.cta_count));
        assert_eq!(row.role, role_value(catalog_lander.role));
        assert_eq!(row.expected_conversion_event_type_ids_json, "[]");
    }
    Ok(())
}

#[tokio::test]
async fn demo_seed_is_idempotent_and_refreshes_only_stable_fake_landers()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(true, "http://127.0.0.1:8091")?;
    let pool = connect_database(&fixture.config).await?;
    insert_user_lander(&pool).await?;
    sqlx::query(
        "UPDATE landing_pages SET name = 'Stale Fake Name'
         WHERE id = 'fake-lander-standard-click-through'",
    )
    .execute(&pool)
    .await?;

    seed_fake_landing_page_catalog(&pool, &fixture.config).await?;

    assert_eq!(fake_lander_count(&pool).await?, 5);
    let refreshed_name: String = sqlx::query_scalar(
        "SELECT name FROM landing_pages WHERE id = 'fake-lander-standard-click-through'",
    )
    .fetch_one(&pool)
    .await?;
    let user_name: String =
        sqlx::query_scalar("SELECT name FROM landing_pages WHERE id = 'operator-lander'")
            .fetch_one(&pool)
            .await?;
    assert_eq!(refreshed_name, "Fake Standard Click-Through Lander");
    assert_eq!(user_name, "Operator Lander");
    Ok(())
}

#[tokio::test]
async fn invalid_fake_landing_page_base_url_fails_before_partial_seed()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(false, "https://public.example")?;
    let pool = connect_database(&fixture.config).await?;

    let result = seed_fake_landing_page_catalog(&pool, &fixture.config).await;

    assert!(result.is_err());
    assert_eq!(fake_lander_count(&pool).await?, 0);
    Ok(())
}

#[tokio::test]
async fn forced_seed_failure_rolls_back_fake_lander_changes()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = DatabaseFixture::new(false, "http://127.0.0.1:8091")?;
    let pool = connect_database(&fixture.config).await?;
    sqlx::query(
        "CREATE TRIGGER fail_fake_multi_cta_seed
         BEFORE INSERT ON landing_pages
         WHEN NEW.id = 'fake-lander-multi-cta'
         BEGIN
           SELECT RAISE(FAIL, 'forced fake lander seed failure');
         END",
    )
    .execute(&pool)
    .await?;

    let result = seed_fake_landing_page_catalog(&pool, &fixture.config).await;

    assert!(result.is_err());
    assert_eq!(fake_lander_count(&pool).await?, 0);
    Ok(())
}

#[derive(Debug)]
struct FakeLanderRow {
    id: String,
    name: String,
    url: String,
    cta_count: i64,
    role: String,
    expected_conversion_event_type_ids_json: String,
}

struct DatabaseFixture {
    _tempdir: tempfile::TempDir,
    config: ServerConfig,
}

impl DatabaseFixture {
    fn new(
        seed_fake_landing_pages: bool,
        fake_landing_page_base_url: &str,
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
            seed_fake_landing_pages,
            fake_landing_page_base_url,
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
    seed_fake_landing_pages: bool,
    fake_landing_page_base_url: &str,
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
        demo_seed_fake_affiliate_network: false,
        fake_affiliate_network_base_url: "http://127.0.0.1:8090".to_string(),
        demo_seed_fake_landing_pages: seed_fake_landing_pages,
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

async fn fake_lander_count(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM landing_pages WHERE id IN ({})",
        fake_lander_placeholders()
    ))
    .bind("fake-lander-standard-click-through")
    .bind("fake-lander-lead-capture")
    .bind("fake-lander-advertorial")
    .bind("fake-lander-after-optin")
    .bind("fake-lander-multi-cta")
    .fetch_one(pool)
    .await
}

async fn fake_lander_rows(pool: &SqlitePool) -> Result<Vec<FakeLanderRow>, sqlx::Error> {
    let rows = sqlx::query(&format!(
        "SELECT id, name, url, cta_count, role, expected_conversion_event_type_ids_json
         FROM landing_pages
         WHERE id IN ({})
         ORDER BY id",
        fake_lander_placeholders()
    ))
    .bind("fake-lander-standard-click-through")
    .bind("fake-lander-lead-capture")
    .bind("fake-lander-advertorial")
    .bind("fake-lander-after-optin")
    .bind("fake-lander-multi-cta")
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| {
            Ok(FakeLanderRow {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                url: row.try_get("url")?,
                cta_count: row.try_get("cta_count")?,
                role: row.try_get("role")?,
                expected_conversion_event_type_ids_json: row
                    .try_get("expected_conversion_event_type_ids_json")?,
            })
        })
        .collect()
}

async fn insert_user_lander(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO landing_pages
         (id, country, name, tags_json, url, url_tokens_json, cta_count, role,
          expected_conversion_event_type_ids_json, language, vertical, weight, notes,
          archived, created_at_millis, updated_at_millis)
         VALUES ('operator-lander', 'Global', 'Operator Lander', '[]',
          'https://operator.example/?next={click_url_1}', '[]', 1, 'standard', '[]',
          'en', 'operator-demo', 100, 'User-created lander', 0, 1, 1)",
    )
    .execute(pool)
    .await?;
    Ok(())
}

fn fake_lander_placeholders() -> &'static str {
    "?, ?, ?, ?, ?"
}

fn role_value(role: ad_buy_engine_domain::LandingPageRole) -> &'static str {
    match role {
        ad_buy_engine_domain::LandingPageRole::Standard => "standard",
        ad_buy_engine_domain::LandingPageRole::LeadCapture => "lead_capture",
        ad_buy_engine_domain::LandingPageRole::Advertorial => "advertorial",
        ad_buy_engine_domain::LandingPageRole::AfterOptin => "after_optin",
    }
}
