use ad_buy_engine_domain::{CredentialUpdateRequest, LoginRequest, SessionResponse};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use tower_sessions::Session;

use crate::error::ServerResult;
use crate::services::authentication;
use crate::web::router::AppState;

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    Json(request): Json<LoginRequest>,
) -> ServerResult<Json<SessionResponse>> {
    authentication::login(&state.pool, &session, request)
        .await
        .map(Json)
}

pub async fn logout(session: Session) -> ServerResult<StatusCode> {
    authentication::logout(&session).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn session(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<SessionResponse>> {
    authentication::session_response(&state.pool, &session)
        .await
        .map(Json)
}

pub async fn credentials(
    State(state): State<AppState>,
    session: Session,
    Json(request): Json<CredentialUpdateRequest>,
) -> ServerResult<Json<SessionResponse>> {
    authentication::update_credentials(&state.pool, &session, request)
        .await
        .map(Json)
}
