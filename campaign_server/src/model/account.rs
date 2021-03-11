use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::invitation::Invitation;
use diesel::prelude::*;
use diesel::update;
use uuid::Uuid;

pub fn query_account(pool: &PoolType, _account_id: Uuid) -> Result<AccountModel, ApiError> {
    use crate::schema::account_table::dsl::{account_id, account_table};
    Ok(account_table
        .filter(account_id.eq(_account_id.to_string()))
        .first::<AccountModel>(&pool.get()?)
        .map_err(|_| ApiError::NotFound("No Account Found".to_string()))?)
}

pub fn return_all_accounts(pool: &PoolType) -> Result<Vec<AccountModel>, ApiError> {
    use crate::schema::account_table::dsl::{account_id, account_table};
    Ok(account_table
        .load::<AccountModel>(&pool.get()?)
        .map_err(|_| ApiError::NotFound("No Account Found".to_string()))?)
}

pub fn update_account_database(
    pool: &PoolType,
    _account_id: Uuid,
    payload: AccountModel,
) -> Result<AccountModel, ApiError> {
    use crate::schema::account_table::dsl::{account_id, account_table};
    Ok(
        update(account_table.filter(account_id.eq(_account_id.to_string())))
            .set(payload)
            .get_result::<AccountModel>(&pool.get()?)?,
    )
}
