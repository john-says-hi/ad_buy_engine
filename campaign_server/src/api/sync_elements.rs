
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix_web::error::BlockingError;
use actix_web::web::{block, Data, Json};
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::crud::{
    CRUDElementRequest, CRUDElementResponse, PrimeElementBuild,
};
use ad_buy_engine::data::elements::funnel::Funnel;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::offer::Offer;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use ad_buy_engine::data::elements::sync::{SyncElementRequest, SyncElementResponse};

pub async fn sync(
    pool: Data<PgPool>,
    payload: Json<SyncElementRequest>,
) -> Result<Json<SyncElementResponse>, ApiError> {
    let mut response = SyncElementResponse{
        offer_sources: vec![],
        offers: vec![],
        landers: vec![],
        traffic_sources: vec![],
        funnels: vec![],
        campaigns: vec![]
    };
    
    
    
    respond_json(response)
}
