use ad_buy_engine_domain::{
    DomainSettingsResponse, DomainSettingsUpdate, FieldError, GeolocationDownloadResponse,
    GeolocationSettingsResponse, GeolocationSettingsUpdate,
};
use axum::Json;
use axum::extract::State;
use tower_sessions::Session;

use crate::error::{ServerError, ServerResult};
use crate::services::authentication::require_session;
use crate::services::geoip::GeoIpService;
use crate::services::geolite_download;
use crate::storage::settings::{
    load_domain_settings, load_geolocation_settings, update_domain_settings,
    update_geolocation_settings,
};
use crate::web::router::AppState;

pub async fn get_domain_settings(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<DomainSettingsResponse>> {
    require_session(&state.pool, &session, false).await?;
    let settings = load_domain_settings(&state.pool).await?;
    Ok(Json(settings.to_response(&state.base_url_overrides)))
}

pub async fn put_domain_settings(
    State(state): State<AppState>,
    session: Session,
    Json(update): Json<DomainSettingsUpdate>,
) -> ServerResult<Json<DomainSettingsResponse>> {
    require_session(&state.pool, &session, false).await?;
    let settings = update_domain_settings(&state.pool, update).await?;
    Ok(Json(settings.to_response(&state.base_url_overrides)))
}

pub async fn get_geolocation_settings(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<GeolocationSettingsResponse>> {
    require_session(&state.pool, &session, false).await?;
    let settings = load_geolocation_settings(&state.pool).await?;
    Ok(Json(settings.to_response(
        geoip_status(&state, DatabaseStatusKind::City),
        geoip_status(&state, DatabaseStatusKind::Country),
        geoip_status(&state, DatabaseStatusKind::Asn),
    )))
}

pub async fn put_geolocation_settings(
    State(state): State<AppState>,
    session: Session,
    Json(update): Json<GeolocationSettingsUpdate>,
) -> ServerResult<Json<GeolocationSettingsResponse>> {
    require_session(&state.pool, &session, false).await?;
    let settings = update_geolocation_settings(&state.pool, update).await?;
    GeoIpService::reload(&state.geoip, &settings.geoip_settings())?;
    Ok(Json(settings.to_response(
        geoip_status(&state, DatabaseStatusKind::City),
        geoip_status(&state, DatabaseStatusKind::Country),
        geoip_status(&state, DatabaseStatusKind::Asn),
    )))
}

pub async fn download_geolite_databases(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<GeolocationDownloadResponse>> {
    require_session(&state.pool, &session, false).await?;
    let settings = load_geolocation_settings(&state.pool).await?;
    validate_download_credentials(&settings.account_id, &settings.license_key)?;
    let response = geolite_download::download_geolite_databases(&settings).await?;
    GeoIpService::reload(&state.geoip, &settings.geoip_settings())?;
    Ok(Json(response))
}

#[derive(Clone, Copy)]
enum DatabaseStatusKind {
    City,
    Country,
    Asn,
}

fn geoip_status(
    state: &AppState,
    status_kind: DatabaseStatusKind,
) -> ad_buy_engine_domain::GeolocationDatabaseStatus {
    let Ok(geoip) = state.geoip.read() else {
        return ad_buy_engine_domain::GeolocationDatabaseStatus {
            configured_path: String::new(),
            exists: false,
            database_type: None,
            build_epoch: None,
            last_loaded_at_millis: None,
            error: Some("GeoIP service lock is unavailable".to_string()),
        };
    };
    match status_kind {
        DatabaseStatusKind::City => geoip.city_status(),
        DatabaseStatusKind::Country => geoip.country_status(),
        DatabaseStatusKind::Asn => geoip.asn_status(),
    }
}

fn validate_download_credentials(account_id: &str, license_key: &str) -> ServerResult<()> {
    let mut details = Vec::new();
    if account_id.trim().is_empty() {
        details.push(FieldError {
            field: "account_id".to_string(),
            message: "MaxMind account ID is required".to_string(),
        });
    }
    if license_key.trim().is_empty() {
        details.push(FieldError {
            field: "license_key".to_string(),
            message: "MaxMind license key is required".to_string(),
        });
    }
    if details.is_empty() {
        Ok(())
    } else {
        Err(ServerError::validation(
            "MaxMind credentials are required before downloading GeoLite databases",
            details,
        ))
    }
}
