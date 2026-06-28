use std::path::PathBuf;

use axum::http::StatusCode;
use axum::routing::{any, get, post, put};
use axum::{Json, Router};
use sqlx::SqlitePool;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::config::{BaseUrlOverrides, ServerConfig};
use crate::services::geoip::{GeoIpService, SharedGeoIpService};
use crate::storage::settings::load_geolocation_settings;
use crate::web::auth::{credentials, login, logout, session};
use crate::web::clicks::{campaign_click, lander_click};
use crate::web::conversions::{conversion_gif, postback};
use crate::web::crud::{
    archive_campaign, archive_conversion_event_type, archive_funnel, archive_landing_page,
    archive_offer, archive_offer_source, archive_traffic_source, create_campaign,
    create_conversion_event_type, create_funnel, create_landing_page, create_offer,
    create_offer_source, create_traffic_source, get_campaign, get_conversion_event_type,
    get_funnel, get_landing_page, get_offer, get_offer_source, get_options, get_traffic_source,
    list_campaigns, list_conversion_event_types, list_funnels, list_landing_pages,
    list_offer_sources, list_offers, list_traffic_sources, update_campaign,
    update_conversion_event_type, update_funnel, update_landing_page, update_offer,
    update_offer_source, update_traffic_source,
};
use crate::web::health::health;
use crate::web::reports::{
    list_browsers, list_connections, list_dates, list_day_parts, list_devices, list_dimension,
    list_operating_systems, list_report_dimensions,
};
use crate::web::settings::{
    download_geolite_databases, get_domain_settings, get_geolocation_settings, put_domain_settings,
    put_geolocation_settings,
};
use crate::web::updates::{check_updates, rollback_update, start_update, update_status};

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: SqlitePool,
    pub base_url_overrides: BaseUrlOverrides,
    pub dashboard_dist: PathBuf,
    pub app_version: String,
    pub update_config: crate::config::UpdateConfig,
    pub geoip: SharedGeoIpService,
}

pub async fn build_router(config: ServerConfig, pool: SqlitePool) -> anyhow::Result<Router> {
    let geolocation_settings = load_geolocation_settings(&pool).await?;
    let geoip = GeoIpService::shared(&geolocation_settings.geoip_settings())?;
    let state = AppState {
        pool,
        base_url_overrides: config.base_url_overrides(),
        dashboard_dist: config.dashboard_dist,
        app_version: config.app_version,
        update_config: config.updates,
        geoip,
    };
    let session_layer = SessionManagerLayer::new(MemoryStore::default()).with_secure(false);
    let static_service = ServeDir::new(&state.dashboard_dist)
        .fallback(ServeFile::new(state.dashboard_dist.join("index.html")));

    Ok(Router::new()
        .route("/api/health", get(health))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/session", get(session))
        .route("/api/auth/credentials", put(credentials))
        .route("/api/updates/status", get(update_status))
        .route("/api/updates/check", post(check_updates))
        .route("/api/updates/start", post(start_update))
        .route("/api/updates/rollback", post(rollback_update))
        .route(
            "/api/settings/geolocation",
            get(get_geolocation_settings).put(put_geolocation_settings),
        )
        .route(
            "/api/settings/domain",
            get(get_domain_settings).put(put_domain_settings),
        )
        .route(
            "/api/settings/geolocation/download",
            post(download_geolite_databases),
        )
        .route(
            "/api/offer-sources",
            get(list_offer_sources).post(create_offer_source),
        )
        .route(
            "/api/offer-sources/{id}",
            get(get_offer_source)
                .put(update_offer_source)
                .delete(archive_offer_source),
        )
        .route("/api/offers", get(list_offers).post(create_offer))
        .route(
            "/api/offers/{id}",
            get(get_offer).put(update_offer).delete(archive_offer),
        )
        .route(
            "/api/landers",
            get(list_landing_pages).post(create_landing_page),
        )
        .route(
            "/api/landers/{id}",
            get(get_landing_page)
                .put(update_landing_page)
                .delete(archive_landing_page),
        )
        .route(
            "/api/conversions",
            get(list_conversion_event_types).post(create_conversion_event_type),
        )
        .route(
            "/api/conversions/{id}",
            get(get_conversion_event_type)
                .put(update_conversion_event_type)
                .delete(archive_conversion_event_type),
        )
        .route(
            "/api/traffic-sources",
            get(list_traffic_sources).post(create_traffic_source),
        )
        .route(
            "/api/traffic-sources/{id}",
            get(get_traffic_source)
                .put(update_traffic_source)
                .delete(archive_traffic_source),
        )
        .route("/api/funnels", get(list_funnels).post(create_funnel))
        .route(
            "/api/funnels/{id}",
            get(get_funnel).put(update_funnel).delete(archive_funnel),
        )
        .route("/api/campaigns", get(list_campaigns).post(create_campaign))
        .route(
            "/api/campaigns/{id}",
            get(get_campaign)
                .put(update_campaign)
                .delete(archive_campaign),
        )
        .route("/api/reports/browsers", get(list_browsers))
        .route("/api/reports/connection", get(list_connections))
        .route("/api/reports/date", get(list_dates))
        .route("/api/reports/day-parting", get(list_day_parts))
        .route("/api/reports/device", get(list_devices))
        .route("/api/reports/os", get(list_operating_systems))
        .route("/api/reports/dimensions", get(list_report_dimensions))
        .route("/api/reports/dimensions/{key}", get(list_dimension))
        .route("/api/options/{name}", get(get_options))
        .route("/api/{*path}", any(api_not_found))
        .route("/c/{campaign_id}", get(campaign_click))
        .route("/go/{visit_id}/{slot}", get(lander_click))
        .route("/postback", get(postback).post(postback))
        .route("/conversion.gif", get(conversion_gif))
        .fallback_service(static_service)
        .layer(TraceLayer::new_for_http())
        .layer(session_layer)
        .with_state(state))
}

async fn api_not_found() -> (StatusCode, Json<ad_buy_engine_domain::ApiErrorBody>) {
    (
        StatusCode::NOT_FOUND,
        Json(ad_buy_engine_domain::ApiErrorBody {
            code: ad_buy_engine_domain::ApiErrorCode::NotFound,
            message: "API route not found".to_string(),
            details: Vec::new(),
        }),
    )
}
