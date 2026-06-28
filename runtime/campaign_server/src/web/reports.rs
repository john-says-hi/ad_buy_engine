use ad_buy_engine_domain::{EntityRow, ListResponse};
use axum::Json;
use axum::extract::{Query, State};
use tower_sessions::Session;

use crate::error::ServerResult;
use crate::services::authentication::require_session;
use crate::storage::reports;
use crate::web::date_filter::DateFilterQuery;
use crate::web::router::AppState;

pub async fn list_browsers(
    State(state): State<AppState>,
    Query(date_filter): Query<DateFilterQuery>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: reports::list_browser_rows(&state.pool, date_filter.into()).await?,
    }))
}

pub async fn list_devices(
    State(state): State<AppState>,
    Query(date_filter): Query<DateFilterQuery>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: reports::list_device_rows(&state.pool, date_filter.into()).await?,
    }))
}

pub async fn list_operating_systems(
    State(state): State<AppState>,
    Query(date_filter): Query<DateFilterQuery>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: reports::list_os_rows(&state.pool, date_filter.into()).await?,
    }))
}

pub async fn list_connections(
    State(state): State<AppState>,
    Query(date_filter): Query<DateFilterQuery>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: reports::list_connection_rows(&state.pool, date_filter.into()).await?,
    }))
}

pub async fn list_dates(
    State(state): State<AppState>,
    Query(date_filter): Query<DateFilterQuery>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: reports::list_date_rows(&state.pool, date_filter.into()).await?,
    }))
}

pub async fn list_day_parts(
    State(state): State<AppState>,
    Query(date_filter): Query<DateFilterQuery>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: reports::list_day_parting_rows(&state.pool, date_filter.into()).await?,
    }))
}
