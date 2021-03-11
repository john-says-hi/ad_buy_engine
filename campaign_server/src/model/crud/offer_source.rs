use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::offer_source::OfferSourceModel;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::result::Error;
use diesel::update;
use uuid::Uuid;

pub fn create_offer_source(
    pool: &PoolType,
    payload: OfferSourceModel,
) -> Result<OfferSourceModel, ApiError> {
    use crate::schema::offer_source_table::dsl::offer_source_table;
    // let res = insert_into(offer_source_table)
    //     .values(payload)
    //     .get_result::<OfferSourceModel>(&pool.get()?);
    //
    // match res {
    //     QueryResult::Ok(a) => Ok(a),
    //     QueryResult::Err(e) => {
    //         match e {
    //             Error::AlreadyInTransaction => {
    //                 println!("111")
    //             }
    //             Error::DatabaseError(a, b) => {
    //                 println!("222")
    //             }
    //             Error::DeserializationError(e) => {
    //                 println!("333")
    //             }
    //             Error::InvalidCString(e) => {
    //                 println!("444")
    //             }
    //             Error::NotFound => {
    //                 println!("555")
    //             }
    //             Error::QueryBuilderError(e) => {
    //                 println!("666")
    //             }
    //             Error::RollbackTransaction => {
    //                 println!("777")
    //             }
    //             Error::SerializationError(e) => {
    //                 println!("888")
    //             }
    //             _ => {
    //                 println!("999")
    //             }
    //         }
    //         Err(ApiError::NotFound("".to_string()))
    //     }
    // }

    Ok(insert_into(offer_source_table)
        .values(payload)
        .get_result::<OfferSourceModel>(&pool.get()?)?)
}

pub fn update_offer_source(
    pool: &PoolType,
    payload: OfferSourceModel,
) -> Result<OfferSourceModel, ApiError> {
    use crate::schema::offer_source_table::dsl::{offer_source_id, offer_source_table};
    println!("{}", &payload.offer_source_id.as_str());

    Ok(
        update(offer_source_table.filter(offer_source_id.eq(payload.offer_source_id.clone())))
            .set(payload)
            .get_result::<OfferSourceModel>(&pool.get()?)?,
    )
}
