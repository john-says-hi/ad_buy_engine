use std::fs;

use ad_buy_engine_domain::{
    CampaignDraft, DestinationType, EntityRow, FunnelDraft, FunnelPath, FunnelSequence,
    LandingPageDraft, OfferDraft, OfferSourceDraft, ReportDimensionKey, SequenceType,
    TrafficSourceDraft, UrlToken, WeightedReference,
};
use axum::body::Body;
use axum::http::header::USER_AGENT;
use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
use campaign_server::config::ServerConfig;
use campaign_server::services::click_processor::{process_campaign_click, process_lander_click};
use campaign_server::services::geoip::GeoIpService;
use campaign_server::storage::database::connect_database;
use campaign_server::storage::date_filter::VisitDateFilter;
use campaign_server::storage::entities::{
    create_campaign, create_funnel, create_landing_page, create_offer, create_offer_source,
    create_traffic_source, list_campaign_rows, list_funnel_rows, list_landing_page_rows,
    list_offer_rows, list_offer_source_rows, list_traffic_source_rows,
};
use campaign_server::storage::reports::{
    list_browser_rows, list_connection_rows, list_date_rows, list_day_parting_rows,
    list_device_rows, list_dimension_rows, list_os_rows,
};
use campaign_server::storage::settings::load_geolocation_settings;
use campaign_server::web::router::build_router;
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn creates_campaign_and_processes_lander_flow() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let database_path = tempdir.path().join("ad_buy_engine.sqlite3");
    let dashboard_dist = tempdir.path().join("dist");
    fs::create_dir(&dashboard_dist)?;
    fs::write(dashboard_dist.join("index.html"), "<main>dashboard</main>")?;
    let config = ServerConfig {
        database_url: format!("sqlite://{}", database_path.display()),
        public_base_url: "http://127.0.0.1:8088".to_string(),
        listen_address: "127.0.0.1:0".to_string(),
        dashboard_dist,
        app_version: "test".to_string(),
        maxmind_account_id: String::new(),
        maxmind_license_key: String::new(),
        geolite_city_database_path: tempdir
            .path()
            .join("GeoLite2-City.mmdb")
            .display()
            .to_string(),
        geolite_country_database_path: tempdir
            .path()
            .join("GeoLite2-Country.mmdb")
            .display()
            .to_string(),
        geolite_asn_database_path: tempdir
            .path()
            .join("GeoLite2-ASN.mmdb")
            .display()
            .to_string(),
    };
    let pool = connect_database(&config).await?;
    let geolocation_settings = load_geolocation_settings(&pool).await?;
    let geoip = GeoIpService::shared(&geolocation_settings.geoip_settings())?;

    let offer_source = create_offer_source(
        &pool,
        OfferSourceDraft {
            name: "Network".to_string(),
            tokens: default_tokens(),
            tracking_domain: "main".to_string(),
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
    let offer = create_offer(
        &pool,
        OfferDraft {
            offer_source_id: offer_source.id,
            country: "Global".to_string(),
            name: "Offer".to_string(),
            tags: Vec::new(),
            url: "https://offer.test/?cid={clickid}&src={src}".to_string(),
            url_tokens: default_tokens(),
            payout_model: "fixed".to_string(),
            payout_value: 1.0,
            currency: "USD".to_string(),
            language: "en".to_string(),
            vertical: "demo".to_string(),
            weight: 100,
            notes: String::new(),
        },
    )
    .await?;
    let lander = create_landing_page(
        &pool,
        LandingPageDraft {
            country: "Global".to_string(),
            name: "Lander".to_string(),
            tags: Vec::new(),
            url: "https://lander.test/?go={click_url_1}".to_string(),
            url_tokens: default_tokens(),
            cta_count: 1,
            language: "en".to_string(),
            vertical: "demo".to_string(),
            weight: 100,
            notes: String::new(),
        },
    )
    .await?;
    let traffic_source = create_traffic_source(
        &pool,
        TrafficSourceDraft {
            name: "Traffic".to_string(),
            external_id_parameter: "subid".to_string(),
            cost_parameter: "cost".to_string(),
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
    let funnel = create_funnel(
        &pool,
        FunnelDraft {
            country: "Global".to_string(),
            name: "Funnel".to_string(),
            redirect_handling: "default".to_string(),
            referrer_handling: "do_nothing".to_string(),
            conditional_sequences: Vec::new(),
            default_sequences: vec![FunnelSequence {
                id: "default".to_string(),
                name: "Default".to_string(),
                active: true,
                weight: 100,
                sequence_type: SequenceType::LandingPageAndOffers,
                conditions: Vec::new(),
                paths: vec![FunnelPath {
                    id: "path".to_string(),
                    weight: 100,
                    landing_page_id: Some(lander.id),
                    offers: vec![WeightedReference {
                        id: offer.id.clone(),
                        weight: 100,
                    }],
                    children: Vec::new(),
                }],
            }],
            notes: String::new(),
        },
    )
    .await?;
    let campaign = create_campaign(
        &pool,
        &config.public_base_url,
        CampaignDraft {
            traffic_source_id: traffic_source.id,
            destination_type: DestinationType::Funnel,
            funnel_id: Some(funnel.id),
            direct_sequence: None,
            cost_model: "CPC".to_string(),
            cost_value: 0.0,
            country: "Global".to_string(),
            name: "Campaign".to_string(),
            notes: String::new(),
        },
    )
    .await?;

    assert!(
        campaign
            .tracking_url
            .ends_with(&format!("/c/{}", campaign.id))
    );
    assert_eq!(
        campaign.traffic_source_query_template,
        "?subid={external_id}&cost={cost}"
    );

    let app = build_router(config.clone(), pool.clone()).await?;
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/c/{}", campaign.id))
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    let location = response
        .headers()
        .get("location")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    assert!(location.starts_with("https://lander.test/"));

    let app = build_router(config.clone(), pool.clone()).await?;
    let response = app
        .oneshot(Request::builder().uri("/offers").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
            (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
        ),
    );
    let outcome = process_campaign_click(
        &pool,
        &config.public_base_url,
        &campaign.id,
        &headers,
        Some("src=paid"),
        &geoip,
    )
    .await?;
    assert!(outcome.target.starts_with("https://lander.test/"));

    let all_time = VisitDateFilter::default();
    let rows = list_campaign_rows(&pool, all_time).await?;
    assert_row_counts(&rows, "Campaign", 2, 2);
    assert_row_counts(&list_offer_rows(&pool, all_time).await?, "Offer", 2, 2);
    assert_row_counts(
        &list_offer_source_rows(&pool, all_time).await?,
        "Network",
        2,
        2,
    );
    assert_row_counts(
        &list_landing_page_rows(&pool, all_time).await?,
        "Lander",
        2,
        2,
    );
    assert_row_counts(
        &list_traffic_source_rows(&pool, all_time).await?,
        "Traffic",
        2,
        2,
    );
    assert_row_counts(&list_funnel_rows(&pool, all_time).await?, "Funnel", 2, 2);

    let future = VisitDateFilter::new(Some(i64::MAX - 1), None);
    assert_row_counts(&list_offer_rows(&pool, future).await?, "Offer", 0, 0);
    assert_eq!(sum_visits(&list_browser_rows(&pool, future).await?), 0);

    let browser_rows = list_browser_rows(&pool, all_time).await?;
    assert_row_counts(&browser_rows, "Chrome", 1, 1);
    assert_eq!(sum_visits(&browser_rows), 2);
    assert_row_counts(
        &list_dimension_rows(&pool, all_time, ReportDimensionKey::BrowserVersions).await?,
        "125.0.0.0",
        1,
        1,
    );
    let device_rows = list_device_rows(&pool, all_time).await?;
    assert_row_counts(&device_rows, "Desktop", 1, 1);
    assert_eq!(sum_visits(&device_rows), 2);
    let os_rows = list_os_rows(&pool, all_time).await?;
    assert_row_counts(&os_rows, "Linux", 1, 1);
    assert_eq!(sum_visits(&os_rows), 2);
    assert_row_counts(
        &list_connection_rows(&pool, all_time).await?,
        "Unknown",
        2,
        2,
    );
    assert_eq!(sum_visits(&list_date_rows(&pool, all_time).await?), 2);
    assert_eq!(
        sum_visits(&list_day_parting_rows(&pool, all_time).await?),
        2
    );
    sqlx::query(
        "UPDATE visits SET country = 'US', region = 'California', city = 'San Francisco',
            asn = 'AS15169', asn_organization = 'Google LLC'",
    )
    .execute(&pool)
    .await?;
    assert_row_counts(
        &list_dimension_rows(&pool, all_time, ReportDimensionKey::Countries).await?,
        "US",
        2,
        2,
    );
    assert_row_counts(
        &list_dimension_rows(&pool, all_time, ReportDimensionKey::Regions).await?,
        "California",
        2,
        2,
    );
    assert_row_counts(
        &list_dimension_rows(&pool, all_time, ReportDimensionKey::Cities).await?,
        "San Francisco",
        2,
        2,
    );
    assert_row_counts(
        &list_dimension_rows(&pool, all_time, ReportDimensionKey::AsnOrganizations).await?,
        "Google LLC",
        2,
        2,
    );

    let visit_id = outcome
        .target
        .split("/go/")
        .nth(1)
        .and_then(|tail| tail.split('/').next())
        .ok_or_else(|| std::io::Error::other("visit id missing from lander URL"))?;
    let continuation = process_lander_click(&pool, visit_id, 1).await?;
    assert!(continuation.target.starts_with("https://offer.test/"));
    assert!(continuation.target.contains("src=paid"));
    Ok(())
}

fn default_tokens() -> Vec<UrlToken> {
    vec![UrlToken {
        name: "clickid".to_string(),
        token: "{clickid}".to_string(),
    }]
}

fn assert_row_counts(rows: &[EntityRow], name: &str, visits: i64, unique_visits: i64) {
    let counts = rows
        .iter()
        .find(|row| row.name == name)
        .map(|row| (row.visits, row.unique_visits));
    assert_eq!(counts, Some((visits, unique_visits)));
}

fn sum_visits(rows: &[EntityRow]) -> i64 {
    rows.iter().map(|row| row.visits).sum()
}
