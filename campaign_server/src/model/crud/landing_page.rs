use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::landing_page::LandingPageModel;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::update;
use uuid::Uuid;

pub fn create_landing_page(
    pool: &PoolType,
    payload: LandingPageModel,
) -> Result<LandingPageModel, ApiError> {
    use crate::schema::landing_page_table::dsl::landing_page_table;
    Ok(insert_into(landing_page_table)
        .values(payload)
        .get_result::<LandingPageModel>(&pool.get()?)?)
}

pub fn update_landing_page(
    pool: &PoolType,
    payload: LandingPageModel,
) -> Result<LandingPageModel, ApiError> {
    use crate::schema::landing_page_table::dsl::{landing_page_id, landing_page_table};

    Ok(
        update(landing_page_table.filter(landing_page_id.eq(payload.landing_page_id.clone())))
            .set(payload)
            .get_result::<LandingPageModel>(&pool.get()?)?,
    )
}
