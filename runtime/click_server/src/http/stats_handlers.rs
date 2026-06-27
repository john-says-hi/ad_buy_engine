use crate::app_state::AppState;
use crate::error::{AppError, AppResult};
use crate::http::request_context::authenticate_admin;
use crate::models::{CampaignStatsResponse, StatsSummaryResponse};
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;

pub async fn get_stats_summary(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<StatsSummaryResponse>> {
    authenticate_admin(&state, &headers).await?;

    let total_clicks = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM click_events")
        .fetch_one(&state.pool)
        .await?
        .0;

    let active_campaigns =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM campaigns WHERE is_active = 1")
            .fetch_one(&state.pool)
            .await?
            .0;

    Ok(Json(StatsSummaryResponse {
        total_clicks,
        active_campaigns,
    }))
}

pub async fn get_campaign_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> AppResult<Json<CampaignStatsResponse>> {
    authenticate_admin(&state, &headers).await?;

    let stats = sqlx::query_as::<_, CampaignStatsResponse>(
        r#"
        SELECT
            campaigns.id AS campaign_id,
            campaigns.name AS campaign_name,
            campaigns.slug AS slug,
            COUNT(click_events.id) AS total_clicks
        FROM campaigns
        LEFT JOIN click_events ON click_events.campaign_id = campaigns.id
        WHERE campaigns.id = ?1
        GROUP BY campaigns.id, campaigns.name, campaigns.slug
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("campaign not found".to_string()))?;

    Ok(Json(stats))
}
