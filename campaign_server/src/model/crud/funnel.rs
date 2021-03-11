use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::funnel::FunnelModel;
use ad_buy_engine::data::elements::funnel::Funnel;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::update;
use uuid::Uuid;

pub fn create_funnel(pool: &PoolType, payload: FunnelModel) -> Result<FunnelModel, ApiError> {
    use crate::schema::funnel_table::dsl::funnel_table;
    Ok(insert_into(funnel_table)
        .values(payload)
        .get_result::<FunnelModel>(&pool.get()?)?)
}

pub fn update_funnel(pool: &PoolType, payload: FunnelModel) -> Result<FunnelModel, ApiError> {
    use crate::schema::funnel_table::dsl::{funnel_id, funnel_table};

    Ok(
        update(funnel_table.filter(funnel_id.eq(payload.funnel_id.clone())))
            .set(payload)
            .get_result::<FunnelModel>(&pool.get()?)?,
    )
}
