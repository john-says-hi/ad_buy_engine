use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::offer::OfferModel;
use ad_buy_engine::data::elements::offer::Offer;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::update;
use uuid::Uuid;

pub fn create_offer(pool: &PoolType, payload: OfferModel) -> Result<OfferModel, ApiError> {
    use crate::schema::offer_table::dsl::offer_table;
    Ok(insert_into(offer_table)
        .values(payload)
        .get_result::<OfferModel>(&pool.get()?)?)
}

pub fn update_offer(pool: &PoolType, payload: OfferModel) -> Result<OfferModel, ApiError> {
    use crate::schema::offer_table::dsl::{offer_id, offer_table};

    Ok(
        update(offer_table.filter(offer_id.eq(payload.offer_id.clone())))
            .set(payload)
            .get_result::<OfferModel>(&pool.get()?)?,
    )
}
