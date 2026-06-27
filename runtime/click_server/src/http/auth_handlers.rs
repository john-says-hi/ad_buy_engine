use crate::app_state::AppState;
use crate::error::{AppError, AppResult};
use crate::http::request_context::{
    authenticate_admin, build_expired_session_cookie, build_session_cookie, read_cookie_value,
    SESSION_COOKIE_NAME,
};
use crate::http::setup_handlers::setup_is_complete;
use crate::models::{AdminUserRecord, LoginRequest, SessionResponse};
use crate::security::{hash_token, new_session_token, verify_password};
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::HeaderMap;
use axum::Json;
use chrono::{Duration, Utc};
use uuid::Uuid;

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<(HeaderMap, Json<SessionResponse>)> {
    let admin = sqlx::query_as::<_, AdminUserRecord>(
        r#"
        SELECT id, username, password_hash
        FROM admin_users
        WHERE username = ?1
        "#,
    )
    .bind(request.username.trim())
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    if !verify_password(&request.password, &admin.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = new_session_token();
    let token_hash = hash_token(&token);
    let now = Utc::now();
    let expires_at = now + Duration::seconds(state.config.session_ttl_seconds);

    sqlx::query(
        r#"
        INSERT INTO sessions (id, admin_user_id, token_hash, expires_at, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&admin.id)
    .bind(token_hash)
    .bind(expires_at.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(&state.pool)
    .await?;

    let mut headers = HeaderMap::new();
    let cookie = build_session_cookie(
        &token,
        state.config.session_ttl_seconds,
        state.config.cookie_secure,
    );
    headers.insert(
        SET_COOKIE,
        cookie
            .parse()
            .map_err(|error| anyhow::anyhow!("invalid session cookie: {error}"))?,
    );

    Ok((
        headers,
        Json(SessionResponse {
            authenticated: true,
            username: Some(admin.username),
            setup_complete: true,
        }),
    ))
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<(HeaderMap, Json<SessionResponse>)> {
    if let Some(token) = read_cookie_value(&headers, SESSION_COOKIE_NAME) {
        let token_hash = hash_token(&token);
        sqlx::query("DELETE FROM sessions WHERE token_hash = ?1")
            .bind(token_hash)
            .execute(&state.pool)
            .await?;
    }

    let mut response_headers = HeaderMap::new();
    let cookie = build_expired_session_cookie(state.config.cookie_secure);
    response_headers.insert(
        SET_COOKIE,
        cookie
            .parse()
            .map_err(|error| anyhow::anyhow!("invalid expired session cookie: {error}"))?,
    );

    Ok((
        response_headers,
        Json(SessionResponse {
            authenticated: false,
            username: None,
            setup_complete: setup_is_complete(&state).await?,
        }),
    ))
}

pub async fn get_session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<SessionResponse>> {
    let setup_complete = setup_is_complete(&state).await?;
    let admin = authenticate_admin(&state, &headers).await.ok();

    Ok(Json(SessionResponse {
        authenticated: admin.is_some(),
        username: admin.map(|admin| admin.username),
        setup_complete,
    }))
}
