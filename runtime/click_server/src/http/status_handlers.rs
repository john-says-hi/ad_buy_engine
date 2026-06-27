use crate::app_state::AppState;
use crate::models::{HealthResponse, MaintenanceStatusResponse, VersionResponse};
use axum::extract::State;
use axum::Json;

pub async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse { ok: true })
}

pub async fn get_version(State(state): State<AppState>) -> Json<VersionResponse> {
    Json(VersionResponse {
        name: "click_server".to_string(),
        version: state.config.version.clone(),
    })
}

pub async fn get_maintenance_status() -> Json<MaintenanceStatusResponse> {
    Json(MaintenanceStatusResponse {
        service: "running".to_string(),
        database: "sqlite".to_string(),
        update_channel: "github_releases".to_string(),
    })
}
