use crate::utils::database::PgPool;
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
    pool: &PgPool,
    payload: LandingPageModel,
) -> Result<LandingPageModel, ApiError> {
    use crate::schema::landing_pages::dsl::landing_pages;
    Ok(insert_into(landing_pages)
        .values(payload)
        .get_result::<LandingPageModel>(&pool.get()?)?)
}

pub fn update_landing_page(
    pool: &PgPool,
    payload: LandingPageModel,
) -> Result<LandingPageModel, ApiError> {
    use crate::schema::landing_pages::dsl::{id as landing_page_id, landing_pages};

    Ok(
        update(landing_pages.filter(landing_page_id.eq(payload.id.clone())))
            .set(payload)
            .get_result::<LandingPageModel>(&pool.get()?)?,
    )
}
