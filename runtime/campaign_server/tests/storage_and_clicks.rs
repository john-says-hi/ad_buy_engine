use std::fs;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{
    CampaignDraft, ConversionTrackingResponse, DestinationType, DomainSettingsUpdate, EntityRow,
    FunnelDraft, FunnelPath, FunnelSequence, LandingPageDraft, LandingPageRole, OfferDraft,
    OfferSourceDraft, ReportDimensionKey, SequenceType, TrafficSourceDraft, UrlToken,
    WeightedReference,
};
use axum::body::{Body, to_bytes};
use axum::http::header::USER_AGENT;
use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
use campaign_server::config::{BaseUrlOverrides, ServerConfig, UpdateConfig};
use campaign_server::services::click_processor::{process_campaign_click, process_lander_click};
use campaign_server::services::geoip::GeoIpService;
use campaign_server::storage::database::connect_database;
use campaign_server::storage::date_filter::VisitDateFilter;
use campaign_server::storage::entities::{
    create_campaign, create_funnel, create_landing_page, create_offer, create_offer_source,
    create_traffic_source, list_campaign_rows, list_funnel_rows, list_landing_page_rows,
    list_offer_rows, list_offer_source_rows, list_traffic_source_rows, update_campaign,
};
use campaign_server::storage::reports::{
    list_browser_rows, list_connection_rows, list_date_rows, list_day_parting_rows,
    list_device_rows, list_dimension_rows, list_os_rows,
};
use campaign_server::storage::settings::{
    effective_tracking_base_url, load_domain_settings, load_geolocation_settings,
    update_domain_settings,
};
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
    let config = test_config(
        &database_path,
        dashboard_dist,
        tempdir.path(),
        "https://track.test",
        "https://admin.test",
        "http://127.0.0.1:8088",
    );
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
            role: LandingPageRole::Standard,
            expected_conversion_event_type_ids: Vec::new(),
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
        &config.tracking_base_url,
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

    assert_eq!(
        campaign.tracking_url,
        format!("https://track.test/c/{}", campaign.id)
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
    let outcome =
        process_campaign_click(&pool, &campaign.id, &headers, Some("src=paid"), &geoip).await?;
    assert!(outcome.target.starts_with("https://lander.test/"));
    assert!(outcome.target.contains("go=https://track.test/go/"));

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

#[tokio::test]
async fn processes_lead_capture_lander_to_advertorial_to_offer()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let database_path = tempdir.path().join("multi-step.sqlite3");
    let dashboard_dist = tempdir.path().join("dist");
    fs::create_dir(&dashboard_dist)?;
    fs::write(dashboard_dist.join("index.html"), "<main>dashboard</main>")?;
    let config = test_config(
        &database_path,
        dashboard_dist,
        tempdir.path(),
        "https://track.test",
        "https://admin.test",
        "http://127.0.0.1:8088",
    );
    let pool = connect_database(&config).await?;
    let geoip = GeoIpService::shared(&load_geolocation_settings(&pool).await?.geoip_settings())?;

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
            name: "Sales Page".to_string(),
            tags: Vec::new(),
            url: "https://sales.test/?cid={clickid}&src={src}".to_string(),
            url_tokens: default_tokens(),
            payout_model: "fixed".to_string(),
            payout_value: 25.0,
            currency: "USD".to_string(),
            language: "en".to_string(),
            vertical: "demo".to_string(),
            weight: 100,
            notes: String::new(),
        },
    )
    .await?;
    let lead_lander = create_landing_page(
        &pool,
        LandingPageDraft {
            country: "Global".to_string(),
            name: "Lead Capture".to_string(),
            tags: Vec::new(),
            url: "https://lead.test/?next={click_url_1}".to_string(),
            url_tokens: default_tokens(),
            cta_count: 1,
            role: LandingPageRole::LeadCapture,
            expected_conversion_event_type_ids: vec!["lead".to_string()],
            language: "en".to_string(),
            vertical: "demo".to_string(),
            weight: 100,
            notes: String::new(),
        },
    )
    .await?;
    let advertorial = create_landing_page(
        &pool,
        LandingPageDraft {
            country: "Global".to_string(),
            name: "Advertorial".to_string(),
            tags: Vec::new(),
            url: "https://advertorial.test/?next={click_url_1}".to_string(),
            url_tokens: default_tokens(),
            cta_count: 1,
            role: LandingPageRole::Advertorial,
            expected_conversion_event_type_ids: Vec::new(),
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
            name: "Lead Funnel".to_string(),
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
                paths: vec![FunnelPath {
                    id: "lead".to_string(),
                    weight: 100,
                    landing_page_id: Some(lead_lander.id),
                    offers: Vec::new(),
                    children: vec![FunnelPath {
                        id: "advertorial".to_string(),
                        weight: 100,
                        landing_page_id: Some(advertorial.id),
                        offers: Vec::new(),
                        children: vec![FunnelPath {
                            id: "sale".to_string(),
                            weight: 100,
                            landing_page_id: None,
                            offers: vec![WeightedReference {
                                id: offer.id.clone(),
                                weight: 100,
                            }],
                            children: Vec::new(),
                        }],
                    }],
                }],
            }],
            notes: String::new(),
        },
    )
    .await?;
    let campaign = create_campaign(
        &pool,
        &config.tracking_base_url,
        CampaignDraft {
            traffic_source_id: traffic_source.id,
            destination_type: DestinationType::Funnel,
            funnel_id: Some(funnel.id),
            direct_sequence: None,
            cost_model: "CPC".to_string(),
            cost_value: 0.0,
            country: "Global".to_string(),
            name: "Lead Campaign".to_string(),
            notes: String::new(),
        },
    )
    .await?;

    let headers = HeaderMap::new();
    let outcome =
        process_campaign_click(&pool, &campaign.id, &headers, Some("src=paid"), &geoip).await?;
    assert!(outcome.target.starts_with("https://lead.test/"));
    let first_slot = route_slot(&outcome.target)?;
    let visit_id = route_visit_id(&outcome.target)?;

    let advertorial_outcome = process_lander_click(&pool, &visit_id, first_slot).await?;
    assert!(
        advertorial_outcome
            .target
            .starts_with("https://advertorial.test/")
    );
    let second_slot = route_slot(&advertorial_outcome.target)?;

    let offer_outcome = process_lander_click(&pool, &visit_id, second_slot).await?;
    assert!(offer_outcome.target.starts_with("https://sales.test/"));
    assert!(offer_outcome.target.contains("src=paid"));
    Ok(())
}

#[tokio::test]
async fn postback_tracks_custom_conversion_and_dedupes_pii()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let database_path = tempdir.path().join("postback.sqlite3");
    let dashboard_dist = tempdir.path().join("dist");
    fs::create_dir(&dashboard_dist)?;
    fs::write(dashboard_dist.join("index.html"), "<main>dashboard</main>")?;
    let config = test_config(
        &database_path,
        dashboard_dist,
        tempdir.path(),
        "https://track.test",
        "https://admin.test",
        "http://127.0.0.1:8088",
    );
    let pool = connect_database(&config).await?;
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
            url: "https://offer.test/?cid={clickid}".to_string(),
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
    let traffic_source = create_traffic_source(
        &pool,
        TrafficSourceDraft {
            name: "Traffic".to_string(),
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
    let campaign = create_campaign(
        &pool,
        &config.tracking_base_url,
        CampaignDraft {
            traffic_source_id: traffic_source.id,
            destination_type: DestinationType::DirectSequence,
            funnel_id: None,
            direct_sequence: Some(FunnelSequence::default_offer(offer.id)),
            cost_model: "CPC".to_string(),
            cost_value: 0.0,
            country: "Global".to_string(),
            name: "Campaign".to_string(),
            notes: String::new(),
        },
    )
    .await?;
    let geoip = GeoIpService::shared(&load_geolocation_settings(&pool).await?.geoip_settings())?;
    let visit =
        process_campaign_click(&pool, &campaign.id, &HeaderMap::new(), None, &geoip).await?;
    let visit_id = visit
        .target
        .split("cid=")
        .nth(1)
        .ok_or_else(|| std::io::Error::other("visit id missing"))?;

    let app = build_router(config.clone(), pool.clone()).await?;
    let uri = format!("/postback?cid={visit_id}&type=Lead&email=test@example.com&eventid=lead-1");
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 64 * 1024).await?;
    let tracked: ConversionTrackingResponse = serde_json::from_slice(&body)?;
    assert!(!tracked.duplicate);
    assert_eq!(tracked.event_type_id, "lead");

    let app = build_router(config.clone(), pool.clone()).await?;
    let duplicate_uri =
        format!("/postback?cid={visit_id}&type=Lead&email=test@example.com&eventid=lead-1");
    let duplicate_response = app
        .oneshot(Request::builder().uri(duplicate_uri).body(Body::empty())?)
        .await?;
    let duplicate_body = to_bytes(duplicate_response.into_body(), 64 * 1024).await?;
    let duplicate: ConversionTrackingResponse = serde_json::from_slice(&duplicate_body)?;
    assert!(duplicate.duplicate);

    let raw_payloads: Vec<String> = sqlx::query_scalar(
        "SELECT raw_payload_json FROM conversion_events ORDER BY created_at_millis",
    )
    .fetch_all(&pool)
    .await?;
    assert_eq!(raw_payloads.len(), 2);
    assert!(
        raw_payloads
            .iter()
            .all(|payload| !payload.contains("test@example.com"))
    );
    Ok(())
}

#[tokio::test]
async fn fresh_database_seeds_domain_base_urls() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let database_path = tempdir.path().join("fresh.sqlite3");
    let config = test_config(
        &database_path,
        tempdir.path().join("dist"),
        tempdir.path(),
        "https://track.example.com",
        "https://admin.example.com",
        "http://127.0.0.1:8088",
    );

    let pool = connect_database(&config).await?;
    let settings = load_domain_settings(&pool).await?;
    let response = settings.to_response(&config.base_url_overrides());

    assert_eq!(response.primary_tracking_domain, "track.example.com");
    assert_eq!(response.tracking_base_url, "https://track.example.com");
    assert_eq!(response.admin_dashboard_domain, "admin.example.com");
    assert_eq!(
        response.admin_dashboard_base_url,
        "https://admin.example.com"
    );
    Ok(())
}

#[tokio::test]
async fn old_database_gets_domain_columns() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let database_path = tempdir.path().join("old.sqlite3");
    let database_url = format!("sqlite://{}", database_path.display());
    fs::File::create(&database_path)?;
    let old_pool = sqlx::SqlitePool::connect(&database_url).await?;
    sqlx::query(
        "CREATE TABLE app_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            public_base_url TEXT NOT NULL,
            session_key_generated_at_millis INTEGER NOT NULL,
            schema_version INTEGER NOT NULL,
            app_version TEXT NOT NULL,
            created_at_millis INTEGER NOT NULL,
            updated_at_millis INTEGER NOT NULL
        )",
    )
    .execute(&old_pool)
    .await?;
    sqlx::query(
        "INSERT INTO app_settings
         (id, public_base_url, session_key_generated_at_millis, schema_version, app_version,
          created_at_millis, updated_at_millis)
         VALUES (1, 'https://legacy.example.com', 1, 1, 'old', 1, 1)",
    )
    .execute(&old_pool)
    .await?;
    old_pool.close().await;
    let config = test_config(
        &database_path,
        tempdir.path().join("dist"),
        tempdir.path(),
        "https://track-env.example.com",
        "https://admin-env.example.com",
        "http://127.0.0.1:8088",
    );

    let pool = connect_database(&config).await?;
    let settings = load_domain_settings(&pool).await?;

    assert_eq!(settings.primary_tracking_domain, "legacy.example.com");
    assert_eq!(settings.tracking_base_url, "https://legacy.example.com");
    assert_eq!(settings.admin_dashboard_domain, "legacy.example.com");
    assert_eq!(
        settings.admin_dashboard_base_url,
        "https://legacy.example.com"
    );
    Ok(())
}

#[tokio::test]
async fn tracking_env_override_wins_without_deleting_stored_settings()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let database_path = tempdir.path().join("override.sqlite3");
    let config = test_config(
        &database_path,
        tempdir.path().join("dist"),
        tempdir.path(),
        "https://stored-track.example.com",
        "https://stored-admin.example.com",
        "http://127.0.0.1:8088",
    );
    let pool = connect_database(&config).await?;
    let overrides = BaseUrlOverrides {
        tracking_base_url: Some("https://env-track.example.com".to_string()),
        admin_dashboard_base_url: Some("https://env-admin.example.com".to_string()),
        public_base_url_fallback: "https://legacy-public.example.com".to_string(),
    };

    let settings = load_domain_settings(&pool).await?;
    let response = settings.to_response(&overrides);

    assert_eq!(response.tracking_base_url, "https://env-track.example.com");
    assert_eq!(
        response.admin_dashboard_base_url,
        "https://env-admin.example.com"
    );
    let stored = load_domain_settings(&pool).await?;
    assert_eq!(stored.tracking_base_url, "https://stored-track.example.com");
    assert_eq!(
        stored.admin_dashboard_base_url,
        "https://stored-admin.example.com"
    );
    Ok(())
}

#[tokio::test]
async fn legacy_public_base_url_fallback_seeds_tracking_roles()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let database_path = tempdir.path().join("legacy-public.sqlite3");
    let config = test_config(
        &database_path,
        tempdir.path().join("dist"),
        tempdir.path(),
        "https://legacy-public.example.com",
        "https://legacy-public.example.com",
        "https://legacy-public.example.com",
    );

    let pool = connect_database(&config).await?;
    let settings = load_domain_settings(&pool).await?;

    assert_eq!(
        settings.effective_tracking_base_url(&config.base_url_overrides()),
        "https://legacy-public.example.com"
    );
    Ok(())
}

#[tokio::test]
async fn campaign_update_regenerates_url_after_domain_change()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let database_path = tempdir.path().join("campaign-domain.sqlite3");
    let config = test_config(
        &database_path,
        tempdir.path().join("dist"),
        tempdir.path(),
        "https://old-track.example.com",
        "https://old-admin.example.com",
        "http://127.0.0.1:8088",
    );
    let pool = connect_database(&config).await?;
    let traffic_source = create_traffic_source(
        &pool,
        TrafficSourceDraft {
            name: "Traffic".to_string(),
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
    let draft = CampaignDraft {
        traffic_source_id: traffic_source.id,
        destination_type: DestinationType::DirectSequence,
        funnel_id: None,
        direct_sequence: Some(FunnelSequence::default_offer("offer-1")),
        cost_model: "CPC".to_string(),
        cost_value: 0.0,
        country: "Global".to_string(),
        name: "Campaign".to_string(),
        notes: String::new(),
    };
    let campaign = create_campaign(&pool, "https://old-track.example.com", draft.clone()).await?;

    update_domain_settings(
        &pool,
        DomainSettingsUpdate::from_primary_domain("new-track.example.com".to_string()),
    )
    .await?;
    let tracking_base_url =
        effective_tracking_base_url(&pool, &config.base_url_overrides()).await?;
    let updated = update_campaign(&pool, &tracking_base_url, &campaign.id, draft).await?;

    assert_eq!(
        updated.tracking_url,
        format!("https://new-track.example.com/c/{}", campaign.id)
    );
    Ok(())
}

fn default_tokens() -> Vec<UrlToken> {
    vec![UrlToken {
        name: "clickid".to_string(),
        token: "{clickid}".to_string(),
    }]
}

fn route_visit_id(target: &str) -> Result<String, std::io::Error> {
    target
        .split("/go/")
        .nth(1)
        .and_then(|tail| tail.split('/').next())
        .map(ToOwned::to_owned)
        .ok_or_else(|| std::io::Error::other("visit id missing from click URL"))
}

fn route_slot(target: &str) -> Result<u8, std::io::Error> {
    target
        .split("/go/")
        .nth(1)
        .and_then(|tail| tail.split('/').nth(1))
        .and_then(|slot| {
            slot.split(|character| ['?', '&'].contains(&character))
                .next()
        })
        .and_then(|slot| slot.parse().ok())
        .ok_or_else(|| std::io::Error::other("slot missing from click URL"))
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

fn test_config(
    database_path: &Path,
    dashboard_dist: PathBuf,
    data_dir: &Path,
    tracking_base_url: &str,
    admin_dashboard_base_url: &str,
    public_base_url: &str,
) -> ServerConfig {
    ServerConfig {
        database_url: format!("sqlite://{}", database_path.display()),
        tracking_base_url: tracking_base_url.to_string(),
        tracking_base_url_override: None,
        admin_dashboard_base_url: admin_dashboard_base_url.to_string(),
        admin_dashboard_base_url_override: None,
        public_base_url: public_base_url.to_string(),
        listen_address: "127.0.0.1:0".to_string(),
        dashboard_dist,
        app_version: "test".to_string(),
        maxmind_account_id: String::new(),
        maxmind_license_key: String::new(),
        geolite_city_database_path: data_dir.join("GeoLite2-City.mmdb").display().to_string(),
        geolite_country_database_path: data_dir.join("GeoLite2-Country.mmdb").display().to_string(),
        geolite_asn_database_path: data_dir.join("GeoLite2-ASN.mmdb").display().to_string(),
        updates: UpdateConfig {
            enabled: false,
            control_dir: data_dir.join("update_control"),
            repo: "john-says-hi/ad_buy_engine".to_string(),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            active_slot: None,
        },
    }
}
