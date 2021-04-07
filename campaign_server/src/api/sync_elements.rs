use crate::db::{campaign, funnel, landing_page, offer, offer_source, traffic_source};
use crate::utils::database::{get_conn, PgPool};
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix_web::error::BlockingError;
use actix_web::web::{block, Data, Json};
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::backend_models::offer::OfferModel;
use ad_buy_engine::data::backend_models::offer_source::OfferSourceModel;
use ad_buy_engine::data::backend_models::DatabaseCommunication;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::crud::{
    CRUDElementRequest, CRUDElementResponse, PrimeElementBuild,
};
use ad_buy_engine::data::elements::funnel::Funnel;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::offer::Offer;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::sync::{SyncElementRequest, SyncElementResponse, SyncFlagData};
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use std::ops::Deref;

pub async fn sync(
    pool: Data<PgPool>,
    payload: Json<SyncElementRequest>,
) -> Result<Json<SyncElementResponse>, ApiError> {
    println!("{:?}", &payload);
    let mut response = SyncElementResponse {
        offer_sources: vec![],
        offers: vec![],
        landers: vec![],
        traffic_sources: vec![],
        funnels: vec![],
        campaigns: vec![],
    };

    let pool_clone = pool.clone();
    let elements_in_db = block(move || offer_source::all(&pool_clone)).await?;

    for element_in_request in &payload.offer_sources {
        if let Some(element) = elements_in_db
            .iter()
            .find(|s| s.id == element_in_request.id.to_string())
        {
            if element.last_updated != element_in_request.last_updated.timestamp() {
                response
                    .offer_sources
                    .push(SyncFlagData::Update(element.clone().into()))
            }
        } else {
            response
                .offer_sources
                .push(SyncFlagData::Remove(element_in_request.id));
        }
    }

    for element_in_database in elements_in_db {
        if let Some(synced) = payload
            .offer_sources
            .iter()
            .find(|s| s.id.to_string() == element_in_database.id)
        {
            ()
        } else {
            response
                .offer_sources
                .push(SyncFlagData::Insert(element_in_database.into()))
        }
    }

    let pool_clone = pool.clone();
    let elements_in_db = block(move || offer::all(&pool_clone)).await?;

    for element_in_request in &payload.offers {
        if let Some(element) = elements_in_db
            .iter()
            .find(|s| s.id == element_in_request.id.to_string())
        {
            if element.last_updated != element_in_request.last_updated.timestamp() {
                response
                    .offers
                    .push(SyncFlagData::Update(element.clone().into()))
            }
        } else {
            response
                .offers
                .push(SyncFlagData::Remove(element_in_request.id));
        }
    }

    for element_in_database in elements_in_db {
        if let Some(synced) = payload
            .offers
            .iter()
            .find(|s| s.id.to_string() == element_in_database.id)
        {
            ()
        } else {
            response
                .offers
                .push(SyncFlagData::Insert(element_in_database.into()))
        }
    }

    let pool_clone = pool.clone();
    let elements_in_db = block(move || traffic_source::all(&pool_clone)).await?;

    for element_in_request in &payload.traffic_sources {
        if let Some(element) = elements_in_db
            .iter()
            .find(|s| s.id == element_in_request.id.to_string())
        {
            if element.last_updated != element_in_request.last_updated.timestamp() {
                response
                    .traffic_sources
                    .push(SyncFlagData::Update(element.clone().into()))
            }
        } else {
            response
                .traffic_sources
                .push(SyncFlagData::Remove(element_in_request.id));
        }
    }

    for element_in_database in elements_in_db {
        if let Some(synced) = payload
            .traffic_sources
            .iter()
            .find(|s| s.id.to_string() == element_in_database.id)
        {
            ()
        } else {
            response
                .traffic_sources
                .push(SyncFlagData::Insert(element_in_database.into()))
        }
    }

    let pool_clone = pool.clone();
    let elements_in_db = block(move || landing_page::all(&pool_clone)).await?;

    for element_in_request in &payload.landers {
        if let Some(element) = elements_in_db
            .iter()
            .find(|s| s.id == element_in_request.id.to_string())
        {
            if element.last_updated != element_in_request.last_updated.timestamp() {
                response
                    .landers
                    .push(SyncFlagData::Update(element.clone().into()))
            }
        } else {
            response
                .landers
                .push(SyncFlagData::Remove(element_in_request.id));
        }
    }

    for element_in_database in elements_in_db {
        if let Some(synced) = payload
            .landers
            .iter()
            .find(|s| s.id.to_string() == element_in_database.id)
        {
            ()
        } else {
            response
                .landers
                .push(SyncFlagData::Insert(element_in_database.into()))
        }
    }

    let pool_clone = pool.clone();
    let elements_in_db = block(move || funnel::all(&pool_clone)).await?;

    for element_in_request in &payload.funnels {
        if let Some(element) = elements_in_db
            .iter()
            .find(|s| s.id == element_in_request.id.to_string())
        {
            if element.last_updated != element_in_request.last_updated.timestamp() {
                response
                    .funnels
                    .push(SyncFlagData::Update(element.clone().into()))
            }
        } else {
            response
                .funnels
                .push(SyncFlagData::Remove(element_in_request.id));
        }
    }

    for element_in_database in elements_in_db {
        if let Some(synced) = payload
            .funnels
            .iter()
            .find(|s| s.id.to_string() == element_in_database.id)
        {
            ()
        } else {
            response
                .funnels
                .push(SyncFlagData::Insert(element_in_database.into()))
        }
    }

    let pool_clone = pool.clone();
    let elements_in_db = block(move || campaign::all(&pool_clone)).await?;

    for element_in_request in &payload.campaigns {
        if let Some(element) = elements_in_db
            .iter()
            .find(|s| s.id == element_in_request.id.to_string())
        {
            if element.last_updated != element_in_request.last_updated.timestamp() {
                response
                    .campaigns
                    .push(SyncFlagData::Update(element.clone().into()))
            }
        } else {
            response
                .campaigns
                .push(SyncFlagData::Remove(element_in_request.id));
        }
    }

    for element_in_database in elements_in_db {
        if let Some(synced) = payload
            .campaigns
            .iter()
            .find(|s| s.id.to_string() == element_in_database.id)
        {
            ()
        } else {
            response
                .campaigns
                .push(SyncFlagData::Insert(element_in_database.into()))
        }
    }

    respond_json(response)
}
