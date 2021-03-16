use crate::utils::database::PgPool;
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
    pool: &PgPool,
    payload: OfferSourceModel,
) -> Result<OfferSourceModel, ApiError> {
    use crate::schema::offer_source_table::dsl::offer_source_table;

    Ok(insert_into(offer_source_table)
        .values(payload)
        .get_result::<OfferSourceModel>(&pool.get()?)?)
}

pub fn update_offer_source(
    pool: &PgPool,
    payload: OfferSourceModel,
) -> Result<OfferSourceModel, ApiError> {
    use crate::schema::offer_source_table::dsl::{id as offer_source_id, offer_source_table};
    println!("{}", &payload.id.as_str());

    Ok(
        update(offer_source_table.filter(offer_source_id.eq(payload.id.clone())))
            .set(payload)
            .get_result::<OfferSourceModel>(&pool.get()?)?,
    )
}
