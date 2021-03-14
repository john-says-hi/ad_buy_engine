use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::traffic_source::TrafficSourceModel;
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::update;
use uuid::Uuid;

pub fn create_traffic_source(
    pool: &PgPool,
    payload: TrafficSourceModel,
) -> Result<TrafficSourceModel, ApiError> {
    use crate::schema::traffic_source_table::dsl::traffic_source_table;
    Ok(insert_into(traffic_source_table)
        .values(payload)
        .get_result::<TrafficSourceModel>(&pool.get()?)?)
}

pub fn update_traffic_source(
    pool: &PgPool,
    payload: TrafficSourceModel,
) -> Result<TrafficSourceModel, ApiError> {
    use crate::schema::traffic_source_table::dsl::{traffic_source_id, traffic_source_table};

    Ok(
        update(
            traffic_source_table.filter(traffic_source_id.eq(payload.traffic_source_id.clone())),
        )
        .set(payload)
        .get_result::<TrafficSourceModel>(&pool.get()?)?,
    )
}
