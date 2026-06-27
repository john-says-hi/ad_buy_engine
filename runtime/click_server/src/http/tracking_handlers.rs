use crate::app_state::AppState;
use crate::error::{AppError, AppResult};
use axum::body::Body;
use axum::extract::{Path, RawQuery, State};
use axum::http::header::{LOCATION, REFERER, USER_AGENT};
use axum::http::{HeaderMap, Response, StatusCode};
use chrono::Utc;
use uuid::Uuid;

const REDIRECT_STATUS_CODE: u16 = 302;

pub async fn track_click(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
) -> AppResult<Response<Body>> {
    let campaign = sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT id, destination_url
        FROM campaigns
        WHERE slug = ?1
          AND is_active = 1
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("campaign not found".to_string()))?;

    let created_at = Utc::now().to_rfc3339();
    sqlx::query(
        r#"
        INSERT INTO click_events (
            id,
            campaign_id,
            slug,
            destination_url,
            request_query,
            referrer,
            ip_address,
            user_agent,
            redirect_status_code,
            created_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&campaign.0)
    .bind(&slug)
    .bind(&campaign.1)
    .bind(query)
    .bind(header_to_string(&headers, REFERER.as_str()))
    .bind(client_ip(&headers))
    .bind(header_to_string(&headers, USER_AGENT.as_str()))
    .bind(i64::from(REDIRECT_STATUS_CODE))
    .bind(created_at)
    .execute(&state.pool)
    .await?;

    Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, campaign.1)
        .body(Body::empty())
        .map_err(|error| AppError::Internal(anyhow::anyhow!("invalid redirect response: {error}")))
}

fn header_to_string(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(ToString::to_string)
}

fn client_ip(headers: &HeaderMap) -> Option<String> {
    header_to_string(headers, "x-forwarded-for")
        .and_then(|value| value.split(',').next().map(|part| part.trim().to_string()))
        .filter(|value| !value.is_empty())
        .or_else(|| header_to_string(headers, "x-real-ip"))
}
