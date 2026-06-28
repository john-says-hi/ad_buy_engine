use std::path::PathBuf;

use axum::http::StatusCode;
use axum::routing::{any, get, post, put};
use axum::{Json, Router};
use sqlx::SqlitePool;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::config::ServerConfig;
use crate::web::auth::{credentials, login, logout, session};
use crate::web::clicks::{campaign_click, lander_click};
use crate::web::crud::{
    archive_campaign, archive_funnel, archive_landing_page, archive_offer, archive_offer_source,
    archive_traffic_source, create_campaign, create_funnel, create_landing_page, create_offer,
    create_offer_source, create_traffic_source, get_campaign, get_funnel, get_landing_page,
    get_offer, get_offer_source, get_options, get_traffic_source, list_campaigns, list_funnels,
    list_landing_pages, list_offer_sources, list_offers, list_traffic_sources, update_campaign,
    update_funnel, update_landing_page, update_offer, update_offer_source, update_traffic_source,
};
use crate::web::health::health;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: SqlitePool,
    pub public_base_url: String,
    pub dashboard_dist: PathBuf,
    pub app_version: String,
}

pub async fn build_router(config: ServerConfig, pool: SqlitePool) -> anyhow::Result<Router> {
    let state = AppState {
        pool,
        public_base_url: config.public_base_url,
        dashboard_dist: config.dashboard_dist,
        app_version: config.app_version,
    };
    let session_layer = SessionManagerLayer::new(MemoryStore::default()).with_secure(false);
    let static_service = ServeDir::new(&state.dashboard_dist)
        .not_found_service(ServeFile::new(state.dashboard_dist.join("index.html")));

    Ok(Router::new()
        .route("/api/health", get(health))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/session", get(session))
        .route("/api/auth/credentials", put(credentials))
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
        .route("/api/options/{name}", get(get_options))
        .route("/api/{*path}", any(api_not_found))
        .route("/c/{campaign_id}", get(campaign_click))
        .route("/go/{visit_id}/{slot}", get(lander_click))
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
