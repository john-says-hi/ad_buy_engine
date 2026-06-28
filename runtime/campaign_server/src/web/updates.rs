use ad_buy_engine_domain::{
    UpdateCheckRequest, UpdateRequestKind, UpdateRollbackRequest, UpdateStartRequest,
    UpdateStatusResponse,
};
use axum::Json;
use axum::extract::State;
use tower_sessions::Session;

use crate::error::{ServerError, ServerResult};
use crate::services::{authentication, updates};
use crate::web::router::AppState;

const CHECK_CONFIRMATION: &str = "CHECK";
const INSTALL_CONFIRMATION: &str = "INSTALL";
const ROLLBACK_CONFIRMATION: &str = "ROLLBACK";

pub async fn update_status(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<UpdateStatusResponse>> {
    authentication::require_session(&state.pool, &session, false).await?;
    status_from_state(&state).map(Json)
}

pub async fn check_updates(
    State(state): State<AppState>,
    session: Session,
    Json(request): Json<UpdateCheckRequest>,
) -> ServerResult<Json<UpdateStatusResponse>> {
    let username = authentication::require_session(&state.pool, &session, false).await?;
    require_confirmation(&request.confirmation, CHECK_CONFIRMATION)?;
    updates::queue_request(
        &state.update_config,
        &state.app_version,
        UpdateRequestKind::Check,
        username,
        None,
    )
    .map(Json)
}

pub async fn start_update(
    State(state): State<AppState>,
    session: Session,
    Json(request): Json<UpdateStartRequest>,
) -> ServerResult<Json<UpdateStatusResponse>> {
    let username = authentication::require_session(&state.pool, &session, false).await?;
    authentication::verify_operator_password(&state.pool, &request.current_password).await?;
    require_confirmation(&request.confirmation, INSTALL_CONFIRMATION)?;
    updates::queue_request(
        &state.update_config,
        &state.app_version,
        UpdateRequestKind::Install,
        username,
        request.requested_version,
    )
    .map(Json)
}

pub async fn rollback_update(
    State(state): State<AppState>,
    session: Session,
    Json(request): Json<UpdateRollbackRequest>,
) -> ServerResult<Json<UpdateStatusResponse>> {
    let username = authentication::require_session(&state.pool, &session, false).await?;
    authentication::verify_operator_password(&state.pool, &request.current_password).await?;
    require_confirmation(&request.confirmation, ROLLBACK_CONFIRMATION)?;
    updates::queue_request(
        &state.update_config,
        &state.app_version,
        UpdateRequestKind::Rollback,
        username,
        None,
    )
    .map(Json)
}

fn status_from_state(state: &AppState) -> ServerResult<UpdateStatusResponse> {
    updates::status(&state.update_config, &state.app_version)
}

fn require_confirmation(actual: &str, expected: &str) -> ServerResult<()> {
    if actual.trim() == expected {
        Ok(())
    } else {
        Err(ServerError::bad_request(format!(
            "Type {expected} to confirm this update action"
        )))
    }
}
