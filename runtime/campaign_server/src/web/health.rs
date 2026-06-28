use ad_buy_engine_domain::HealthResponse;
use axum::Json;
use axum::extract::State;
use sqlx::Row;

use crate::error::ServerResult;
use crate::web::router::AppState;

pub async fn health(State(state): State<AppState>) -> ServerResult<Json<HealthResponse>> {
    let row = sqlx::query("SELECT schema_version, app_version FROM app_settings WHERE id = 1")
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(HealthResponse {
        ok: true,
        app_version: row.try_get("app_version")?,
        schema_version: row.try_get("schema_version")?,
    }))
}
