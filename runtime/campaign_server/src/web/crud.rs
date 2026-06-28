use ad_buy_engine_domain::{
    Campaign, CampaignDraft, EntityRow, Funnel, FunnelDraft, LandingPage, LandingPageDraft,
    ListResponse, Offer, OfferDraft, OfferSource, OfferSourceDraft, OptionsResponse, TrafficSource,
    TrafficSourceDraft,
};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use tower_sessions::Session;

use crate::error::ServerResult;
use crate::services::authentication::require_session;
use crate::storage::entities;
use crate::web::router::AppState;

pub async fn list_offer_sources(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: entities::list_offer_source_rows(&state.pool).await?,
    }))
}

pub async fn create_offer_source(
    State(state): State<AppState>,
    session: Session,
    Json(draft): Json<OfferSourceDraft>,
) -> ServerResult<Json<OfferSource>> {
    require_session(&state.pool, &session, false).await?;
    entities::create_offer_source(&state.pool, draft)
        .await
        .map(Json)
}

pub async fn get_offer_source(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<Json<OfferSource>> {
    require_session(&state.pool, &session, false).await?;
    entities::get_offer_source(&state.pool, &id).await.map(Json)
}

pub async fn update_offer_source(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
    Json(draft): Json<OfferSourceDraft>,
) -> ServerResult<Json<OfferSource>> {
    require_session(&state.pool, &session, false).await?;
    entities::update_offer_source(&state.pool, &id, draft)
        .await
        .map(Json)
}

pub async fn archive_offer_source(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<StatusCode> {
    require_session(&state.pool, &session, false).await?;
    entities::archive_offer_source(&state.pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_offers(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: entities::list_offer_rows(&state.pool).await?,
    }))
}

pub async fn create_offer(
    State(state): State<AppState>,
    session: Session,
    Json(draft): Json<OfferDraft>,
) -> ServerResult<Json<Offer>> {
    require_session(&state.pool, &session, false).await?;
    entities::create_offer(&state.pool, draft).await.map(Json)
}

pub async fn get_offer(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<Json<Offer>> {
    require_session(&state.pool, &session, false).await?;
    entities::get_offer(&state.pool, &id).await.map(Json)
}

pub async fn update_offer(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
    Json(draft): Json<OfferDraft>,
) -> ServerResult<Json<Offer>> {
    require_session(&state.pool, &session, false).await?;
    entities::update_offer(&state.pool, &id, draft)
        .await
        .map(Json)
}

pub async fn archive_offer(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<StatusCode> {
    require_session(&state.pool, &session, false).await?;
    entities::archive_offer(&state.pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_landing_pages(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: entities::list_landing_page_rows(&state.pool).await?,
    }))
}

pub async fn create_landing_page(
    State(state): State<AppState>,
    session: Session,
    Json(draft): Json<LandingPageDraft>,
) -> ServerResult<Json<LandingPage>> {
    require_session(&state.pool, &session, false).await?;
    entities::create_landing_page(&state.pool, draft)
        .await
        .map(Json)
}

pub async fn get_landing_page(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<Json<LandingPage>> {
    require_session(&state.pool, &session, false).await?;
    entities::get_landing_page(&state.pool, &id).await.map(Json)
}

pub async fn update_landing_page(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
    Json(draft): Json<LandingPageDraft>,
) -> ServerResult<Json<LandingPage>> {
    require_session(&state.pool, &session, false).await?;
    entities::update_landing_page(&state.pool, &id, draft)
        .await
        .map(Json)
}

pub async fn archive_landing_page(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<StatusCode> {
    require_session(&state.pool, &session, false).await?;
    entities::archive_landing_page(&state.pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_traffic_sources(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: entities::list_traffic_source_rows(&state.pool).await?,
    }))
}

pub async fn create_traffic_source(
    State(state): State<AppState>,
    session: Session,
    Json(draft): Json<TrafficSourceDraft>,
) -> ServerResult<Json<TrafficSource>> {
    require_session(&state.pool, &session, false).await?;
    entities::create_traffic_source(&state.pool, draft)
        .await
        .map(Json)
}

pub async fn get_traffic_source(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<Json<TrafficSource>> {
    require_session(&state.pool, &session, false).await?;
    entities::get_traffic_source(&state.pool, &id)
        .await
        .map(Json)
}

pub async fn update_traffic_source(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
    Json(draft): Json<TrafficSourceDraft>,
) -> ServerResult<Json<TrafficSource>> {
    require_session(&state.pool, &session, false).await?;
    entities::update_traffic_source(&state.pool, &id, draft)
        .await
        .map(Json)
}

pub async fn archive_traffic_source(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<StatusCode> {
    require_session(&state.pool, &session, false).await?;
    entities::archive_traffic_source(&state.pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_funnels(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: entities::list_funnel_rows(&state.pool).await?,
    }))
}

pub async fn create_funnel(
    State(state): State<AppState>,
    session: Session,
    Json(draft): Json<FunnelDraft>,
) -> ServerResult<Json<Funnel>> {
    require_session(&state.pool, &session, false).await?;
    entities::create_funnel(&state.pool, draft).await.map(Json)
}

pub async fn get_funnel(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<Json<Funnel>> {
    require_session(&state.pool, &session, false).await?;
    entities::get_funnel(&state.pool, &id).await.map(Json)
}

pub async fn update_funnel(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
    Json(draft): Json<FunnelDraft>,
) -> ServerResult<Json<Funnel>> {
    require_session(&state.pool, &session, false).await?;
    entities::update_funnel(&state.pool, &id, draft)
        .await
        .map(Json)
}

pub async fn archive_funnel(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<StatusCode> {
    require_session(&state.pool, &session, false).await?;
    entities::archive_funnel(&state.pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_campaigns(
    State(state): State<AppState>,
    session: Session,
) -> ServerResult<Json<ListResponse<EntityRow>>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(ListResponse {
        items: entities::list_campaign_rows(&state.pool).await?,
    }))
}

pub async fn create_campaign(
    State(state): State<AppState>,
    session: Session,
    Json(draft): Json<CampaignDraft>,
) -> ServerResult<Json<Campaign>> {
    require_session(&state.pool, &session, false).await?;
    entities::create_campaign(&state.pool, &state.public_base_url, draft)
        .await
        .map(Json)
}

pub async fn get_campaign(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<Json<Campaign>> {
    require_session(&state.pool, &session, false).await?;
    entities::get_campaign(&state.pool, &id).await.map(Json)
}

pub async fn update_campaign(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
    Json(draft): Json<CampaignDraft>,
) -> ServerResult<Json<Campaign>> {
    require_session(&state.pool, &session, false).await?;
    entities::update_campaign(&state.pool, &state.public_base_url, &id, draft)
        .await
        .map(Json)
}

pub async fn archive_campaign(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> ServerResult<StatusCode> {
    require_session(&state.pool, &session, false).await?;
    entities::archive_campaign(&state.pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_options(
    State(state): State<AppState>,
    session: Session,
    Path(name): Path<String>,
) -> ServerResult<Json<OptionsResponse>> {
    require_session(&state.pool, &session, false).await?;
    Ok(Json(OptionsResponse {
        items: entities::option_items(&state.pool, &name).await?,
    }))
}
