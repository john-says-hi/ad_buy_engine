use ad_buy_engine_domain::DashboardSummaryResponse;
use axum::Json;
use axum::extract::{Query, State};
use tower_sessions::Session;

use crate::error::ServerResult;
use crate::services::authentication::require_session;
use crate::storage::dashboard::dashboard_summary;
use crate::web::date_filter::DateFilterQuery;
use crate::web::router::AppState;

pub async fn summary(
    State(state): State<AppState>,
    Query(date_filter): Query<DateFilterQuery>,
    session: Session,
) -> ServerResult<Json<DashboardSummaryResponse>> {
    require_session(&state.pool, &session, false).await?;
    dashboard_summary(&state.pool, date_filter.into())
        .await
        .map(Json)
}
