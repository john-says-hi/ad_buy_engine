use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub ok: bool,
}

#[derive(Debug, Serialize)]
pub struct VersionResponse {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct SetupStatusResponse {
    pub setup_complete: bool,
}

#[derive(Debug, Deserialize)]
pub struct CompleteSetupRequest {
    pub setup_secret: String,
    pub username: String,
    pub password: String,
    pub tracking_domain: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub authenticated: bool,
    pub username: Option<String>,
    pub setup_complete: bool,
}

#[derive(Debug, Deserialize)]
pub struct CampaignInput {
    pub name: String,
    pub slug: String,
    pub destination_url: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct CampaignResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub destination_url: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow)]
pub struct AdminUserRecord {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize)]
pub struct StatsSummaryResponse {
    pub total_clicks: i64,
    pub active_campaigns: i64,
}

#[derive(Debug, FromRow, Serialize)]
pub struct CampaignStatsResponse {
    pub campaign_id: String,
    pub campaign_name: String,
    pub slug: String,
    pub total_clicks: i64,
}

#[derive(Debug, Serialize)]
pub struct MaintenanceStatusResponse {
    pub service: String,
    pub database: String,
    pub update_channel: String,
}
