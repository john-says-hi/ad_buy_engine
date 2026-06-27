use crate::app_state::AppState;
use crate::http::auth_handlers::{get_session, login, logout};
use crate::http::campaign_handlers::{
    create_campaign, delete_campaign, list_campaigns, update_campaign,
};
use crate::http::setup_handlers::{complete_setup, get_setup_status};
use crate::http::stats_handlers::{get_campaign_stats, get_stats_summary};
use crate::http::status_handlers::{get_health, get_maintenance_status, get_version};
use crate::http::tracking_handlers::track_click;
use axum::routing::{get, post, put};
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

pub fn build_router(state: AppState) -> Router {
    let admin_dist_dir = state.config.admin_dist_dir.clone();
    let index_file = admin_dist_dir.join("index.html");
    let admin_web = ServeDir::new(admin_dist_dir).fallback(ServeFile::new(index_file));

    Router::new()
        .route("/health", get(get_health))
        .route("/api/version", get(get_version))
        .route("/api/maintenance/status", get(get_maintenance_status))
        .route("/api/setup/status", get(get_setup_status))
        .route("/api/setup/complete", post(complete_setup))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/session", get(get_session))
        .route("/api/campaigns", get(list_campaigns).post(create_campaign))
        .route(
            "/api/campaigns/{id}",
            put(update_campaign).delete(delete_campaign),
        )
        .route("/api/stats/summary", get(get_stats_summary))
        .route("/api/stats/campaign/{id}", get(get_campaign_stats))
        .route("/c/{slug}", get(track_click))
        .fallback_service(admin_web)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
