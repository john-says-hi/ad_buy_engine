use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::campaign::CampaignModel;
use ad_buy_engine::data::elements::campaign::Campaign;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::update;
use uuid::Uuid;

pub fn create_campaign(pool: &PoolType, payload: CampaignModel) -> Result<CampaignModel, ApiError> {
    use crate::schema::campaign_table::dsl::campaign_table;
    Ok(insert_into(campaign_table)
        .values(payload)
        .get_result::<CampaignModel>(&pool.get()?)?)
}

pub fn update_campaign(pool: &PoolType, payload: CampaignModel) -> Result<CampaignModel, ApiError> {
    use crate::schema::campaign_table::dsl::{campaign_id, campaign_table};

    Ok(
        update(campaign_table.filter(campaign_id.eq(payload.campaign_id.clone())))
            .set(payload)
            .get_result::<CampaignModel>(&pool.get()?)?,
    )
}
