use crate::app_state::AppState;
use crate::error::{AppError, AppResult};
use crate::http::request_context::authenticate_admin;
use crate::models::{CampaignInput, CampaignResponse};
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use chrono::Utc;
use url::Url;
use uuid::Uuid;

pub async fn list_campaigns(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<Vec<CampaignResponse>>> {
    authenticate_admin(&state, &headers).await?;

    let campaigns = sqlx::query_as::<_, CampaignResponse>(
        r#"
        SELECT id, name, slug, destination_url, is_active, created_at, updated_at
        FROM campaigns
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(campaigns))
}

pub async fn create_campaign(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<CampaignInput>,
) -> AppResult<Json<CampaignResponse>> {
    authenticate_admin(&state, &headers).await?;
    validate_campaign_input(&input)?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let is_active = input.is_active.unwrap_or(true);

    sqlx::query(
        r#"
        INSERT INTO campaigns (id, name, slug, destination_url, is_active, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
    )
    .bind(&id)
    .bind(input.name.trim())
    .bind(input.slug.trim())
    .bind(input.destination_url.trim())
    .bind(is_active)
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(map_unique_slug_error)?;

    get_campaign_by_id(&state, &id).await.map(Json)
}

pub async fn update_campaign(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(input): Json<CampaignInput>,
) -> AppResult<Json<CampaignResponse>> {
    authenticate_admin(&state, &headers).await?;
    validate_campaign_input(&input)?;

    let now = Utc::now().to_rfc3339();
    let is_active = input.is_active.unwrap_or(true);
    let result = sqlx::query(
        r#"
        UPDATE campaigns
        SET name = ?1,
            slug = ?2,
            destination_url = ?3,
            is_active = ?4,
            updated_at = ?5
        WHERE id = ?6
        "#,
    )
    .bind(input.name.trim())
    .bind(input.slug.trim())
    .bind(input.destination_url.trim())
    .bind(is_active)
    .bind(now)
    .bind(&id)
    .execute(&state.pool)
    .await
    .map_err(map_unique_slug_error)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("campaign not found".to_string()));
    }

    get_campaign_by_id(&state, &id).await.map(Json)
}

pub async fn delete_campaign(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> AppResult<Json<CampaignResponse>> {
    authenticate_admin(&state, &headers).await?;

    let now = Utc::now().to_rfc3339();
    let result = sqlx::query(
        r#"
        UPDATE campaigns
        SET is_active = 0,
            updated_at = ?1
        WHERE id = ?2
        "#,
    )
    .bind(now)
    .bind(&id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("campaign not found".to_string()));
    }

    get_campaign_by_id(&state, &id).await.map(Json)
}

async fn get_campaign_by_id(state: &AppState, id: &str) -> AppResult<CampaignResponse> {
    sqlx::query_as::<_, CampaignResponse>(
        r#"
        SELECT id, name, slug, destination_url, is_active, created_at, updated_at
        FROM campaigns
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("campaign not found".to_string()))
}

fn validate_campaign_input(input: &CampaignInput) -> AppResult<()> {
    if input.name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "campaign name is required".to_string(),
        ));
    }

    validate_slug(input.slug.trim())?;

    let url = Url::parse(input.destination_url.trim())
        .map_err(|_| AppError::BadRequest("destination URL is invalid".to_string()))?;

    match url.scheme() {
        "http" | "https" => Ok(()),
        _ => Err(AppError::BadRequest(
            "destination URL must use http or https".to_string(),
        )),
    }
}

fn validate_slug(slug: &str) -> AppResult<()> {
    if slug.len() < 3 {
        return Err(AppError::BadRequest(
            "campaign slug must be at least 3 characters".to_string(),
        ));
    }

    if !slug
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_')
    {
        return Err(AppError::BadRequest(
            "campaign slug can only contain letters, numbers, dashes, or underscores".to_string(),
        ));
    }

    Ok(())
}

fn map_unique_slug_error(error: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(database_error) = &error {
        if database_error.is_unique_violation() {
            return AppError::Conflict("campaign slug already exists".to_string());
        }
    }

    AppError::Database(error)
}
