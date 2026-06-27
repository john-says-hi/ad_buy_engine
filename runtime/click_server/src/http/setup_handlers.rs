use crate::app_state::AppState;
use crate::error::{AppError, AppResult};
use crate::http::request_context::build_session_cookie;
use crate::models::{CompleteSetupRequest, SessionResponse, SetupStatusResponse};
use crate::security::{hash_password, hash_token, new_session_token, secrets_match};
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::HeaderMap;
use axum::Json;
use chrono::{Duration, Utc};
use url::Url;
use uuid::Uuid;

pub async fn get_setup_status(
    State(state): State<AppState>,
) -> AppResult<Json<SetupStatusResponse>> {
    Ok(Json(SetupStatusResponse {
        setup_complete: setup_is_complete(&state).await?,
    }))
}

pub async fn complete_setup(
    State(state): State<AppState>,
    Json(request): Json<CompleteSetupRequest>,
) -> AppResult<(HeaderMap, Json<SessionResponse>)> {
    if setup_is_complete(&state).await? {
        return Err(AppError::Conflict("setup is already complete".to_string()));
    }

    if !secrets_match(&request.setup_secret, &state.config.setup_secret) {
        return Err(AppError::Forbidden("setup secret is invalid".to_string()));
    }

    validate_setup_request(&request)?;

    let now = Utc::now().to_rfc3339();
    let admin_id = Uuid::new_v4().to_string();
    let password_hash = hash_password(&request.password)?;
    let token = new_session_token();
    let token_hash = hash_token(&token);
    let expires_at = Utc::now() + Duration::seconds(state.config.session_ttl_seconds);

    let mut transaction = state.pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO admin_users (id, username, password_hash, created_at)
        VALUES (?1, ?2, ?3, ?4)
        "#,
    )
    .bind(&admin_id)
    .bind(request.username.trim())
    .bind(password_hash)
    .bind(&now)
    .execute(&mut *transaction)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO sessions (id, admin_user_id, token_hash, expires_at, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&admin_id)
    .bind(token_hash)
    .bind(expires_at.to_rfc3339())
    .bind(&now)
    .execute(&mut *transaction)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO settings (key, value, updated_at)
        VALUES ('setup_complete', 'true', ?1)
        ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at
        "#,
    )
    .bind(&now)
    .execute(&mut *transaction)
    .await?;

    if let Some(tracking_domain) = cleaned_optional_text(request.tracking_domain.as_deref()) {
        sqlx::query(
            r#"
            INSERT INTO domain_settings (id, tracking_domain, https_enabled, updated_at)
            VALUES (1, ?1, 0, ?2)
            ON CONFLICT(id) DO UPDATE
            SET tracking_domain = excluded.tracking_domain,
                https_enabled = excluded.https_enabled,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(tracking_domain)
        .bind(&now)
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;

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
            username: Some(request.username.trim().to_string()),
            setup_complete: true,
        }),
    ))
}

pub async fn setup_is_complete(state: &AppState) -> AppResult<bool> {
    let value =
        sqlx::query_as::<_, (String,)>("SELECT value FROM settings WHERE key = 'setup_complete'")
            .fetch_optional(&state.pool)
            .await?;

    Ok(value
        .map(|(stored_value,)| stored_value == "true")
        .unwrap_or(false))
}

fn validate_setup_request(request: &CompleteSetupRequest) -> AppResult<()> {
    if request.username.trim().len() < 3 {
        return Err(AppError::BadRequest(
            "username must be at least 3 characters".to_string(),
        ));
    }

    if request.password.len() < 12 {
        return Err(AppError::BadRequest(
            "password must be at least 12 characters".to_string(),
        ));
    }

    if let Some(domain) = cleaned_optional_text(request.tracking_domain.as_deref()) {
        validate_domain_like_value(domain)?;
    }

    Ok(())
}

fn cleaned_optional_text(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn validate_domain_like_value(value: &str) -> AppResult<()> {
    if value.contains('/') {
        Url::parse(value)
            .map_err(|_| AppError::BadRequest("tracking domain URL is invalid".to_string()))?;
    }

    Ok(())
}
