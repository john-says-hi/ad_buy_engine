use crate::model::crud::campaign::{create_campaign, update_campaign};
use crate::model::crud::funnel::{create_funnel, update_funnel};
use crate::model::crud::landing_page::{create_landing_page, update_landing_page};
use crate::model::crud::offer::{create_offer, update_offer};
use crate::model::crud::offer_source::{create_offer_source, update_offer_source};
use crate::model::crud::traffic_source::{create_traffic_source, update_traffic_source};
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use crate::utils::state::AppState;
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

pub async fn process_crud(
    pool: Data<PgPool>,
    payload: Json<CRUDElementRequest>,
    state: AppState,
) -> Result<Json<CRUDElementResponse>, ApiError> {
    let state = state.into_inner();

    match payload.into_inner() {
        CRUDElementRequest::Create(element) => {
            println!("3");
            match element {
                PrimeElementBuild::OfferSource(data) => {
                    println!("3");

                    let element: OfferSource =
                        block(move || create_offer_source(&pool, data.into()))
                            .await?
                            .into();
                    println!("3");

                    respond_json(CRUDElementResponse {
                        list_of_return_elements: vec![element.into()],
                    })
                }
                PrimeElementBuild::Offer(data) => {
                    let element: Offer = block(move || create_offer(&pool, data.into()))
                        .await?
                        .into();
                    respond_json(CRUDElementResponse {
                        list_of_return_elements: vec![element.into()],
                    })
                }
                PrimeElementBuild::LandingPage(data) => {
                    let element: LandingPage =
                        block(move || create_landing_page(&pool, data.into()))
                            .await?
                            .into();
                    respond_json(CRUDElementResponse {
                        list_of_return_elements: vec![element.into()],
                    })
                }
                PrimeElementBuild::Funnel(data) => {
                    let element: Funnel = block(move || create_funnel(&pool, data.into()))
                        .await?
                        .into();
                    respond_json(CRUDElementResponse {
                        list_of_return_elements: vec![element.into()],
                    })
                }
                PrimeElementBuild::TrafficSource(data) => {
                    let element: TrafficSource =
                        block(move || create_traffic_source(&pool, data.into()))
                            .await?
                            .into();
                    respond_json(CRUDElementResponse {
                        list_of_return_elements: vec![element.into()],
                    })
                }
                PrimeElementBuild::Campaign(data) => {
                    let element: Campaign = block(move || create_campaign(&pool, data.into()))
                        .await?
                        .into();
                    let new = element.clone();
                    let id = element.campaign_id.clone();
                    {
                        let mut campaigns = &mut *state.lock().expect("f43ewse");
                        campaigns.insert(id, new);
                    }

                    respond_json(CRUDElementResponse {
                        list_of_return_elements: vec![element.into()],
                    })
                }
            }
        }

        CRUDElementRequest::Update(element_list) => {
            let mut response = CRUDElementResponse {
                list_of_return_elements: vec![],
            };
            let mut list = element_list.iter();
            while let Some(element) = list.next().cloned() {
                let pool = pool.clone();
                match element {
                    PrimeElementBuild::OfferSource(data) => {
                        println!("2");
                        let element: OfferSource =
                            block(move || update_offer_source(&pool, data.into()))
                                .await?
                                .into();
                        response.list_of_return_elements.push(element.into())
                    }
                    PrimeElementBuild::Offer(data) => {
                        let element: Offer = block(move || update_offer(&pool, data.into()))
                            .await?
                            .into();
                        response.list_of_return_elements.push(element.into())
                    }
                    PrimeElementBuild::LandingPage(data) => {
                        let element: LandingPage =
                            block(move || update_landing_page(&pool, data.into()))
                                .await?
                                .into();
                        response.list_of_return_elements.push(element.into())
                    }
                    PrimeElementBuild::Funnel(data) => {
                        let element: Funnel = block(move || update_funnel(&pool, data.into()))
                            .await?
                            .into();
                        response.list_of_return_elements.push(element.into())
                    }
                    PrimeElementBuild::TrafficSource(data) => {
                        let element: TrafficSource =
                            block(move || update_traffic_source(&pool, data.into()))
                                .await?
                                .into();
                        response.list_of_return_elements.push(element.into())
                    }
                    PrimeElementBuild::Campaign(data) => {
                        let element: Campaign = block(move || update_campaign(&pool, data.into()))
                            .await?
                            .into();

                        let new = element.clone();
                        let id = element.campaign_id.clone();
                        {
                            let mut campaigns = &mut *state.lock().expect("f43ewse");
                            campaigns.insert(id, new);
                        }

                        response.list_of_return_elements.push(element.into())
                    }
                }
            }
            respond_json(response)
        }
    }
}
