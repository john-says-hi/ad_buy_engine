use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::error::BlockingError;
use actix_web::web::{block, Data, Json, Path, Query};
use actix_web::HttpResponse;
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::backend_models::campaign::CampaignModel;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::crud::{
    CRUDElementRequest, CRUDElementResponse, PrimeElementBuild,
};
use ad_buy_engine::data::elements::funnel::Funnel;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::offer::Offer;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use chrono::Utc;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;

pub async fn process_click(
    pool: Data<PoolType>,
    app_state: Data<Mutex<HashMap<Uuid, Campaign>>>,
    redis: Data<Addr<RedisActor>>,
    campaign_id: Path<Uuid>,
    traffic_source_parameters: Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    if let Some(found) = find_campaign(campaign_id.into_inner(), app_state, &pool) {
        // build click map
        // build visit
        // visitor identity
        //  save to redis

        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApiError::NotFound(
            "no campaign found in appstate".to_string(),
        ))
    }
}

fn find_campaign(
    id: Uuid,
    state: Data<Mutex<HashMap<Uuid, Campaign>>>,
    pool: &PoolType,
) -> Option<Campaign> {
    let state = state.into_inner();

    let restored = {
        let campaigns = &*state.lock().expect("asdf");
        campaigns.get(&id).cloned()
    };

    if let Some(_) = &restored {
        return restored;
    } else {
        let mut restored: Campaign = {
            use crate::schema::campaign_table::dsl::{campaign_id, campaign_table};
            use diesel::prelude::*;
            campaign_table
                .filter(campaign_id.eq(id.to_string()))
                .first::<CampaignModel>(&pool.get().expect("FREds"))
                .expect("G%sdf")
                .into()
        };
        restored.last_clicked = Utc::now();
        {
            let mut lock = state.lock().unwrap();
            lock.insert(restored.campaign_id.clone(), restored.clone());
        }

        Some(restored)
    }
}
