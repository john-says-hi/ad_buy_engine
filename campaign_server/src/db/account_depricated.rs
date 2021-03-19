use crate::utils::database::{PgPool, get_conn};
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::invitation::Invitation;
use diesel::prelude::*;
use diesel::update;
use uuid::Uuid;
use std::ops::Deref;

pub fn query_account(pool: &PgPool, _account_id: Uuid) -> Result<AccountModel, ApiError> {
    use crate::schema::accounts::dsl::{id as account_id, accounts};
    Ok(accounts
        .filter(account_id.eq(_account_id.to_string()))
        .first::<AccountModel>(get_conn(pool)?.deref())
        .map_err(|_| ApiError::NotFound("No Account Found".to_string()))?)
}

pub fn return_all_accounts(pool: &PgPool) -> Result<Vec<AccountModel>, ApiError> {
    use crate::schema::accounts::dsl::{id as account_id, accounts};
    Ok(accounts
        .load::<AccountModel>(&pool.get()?)
        .map_err(|_| ApiError::NotFound("No Account Found".to_string()))?)
}

pub fn update_account_database(
    pool: &PgPool,
    _account_id: Uuid,
    payload: AccountModel,
) -> Result<AccountModel, ApiError> {
    use crate::schema::accounts::dsl::{id as account_id, accounts};
    Ok(
        update(accounts.filter(account_id.eq(_account_id.to_string())))
            .set(payload)
            .get_result::<AccountModel>(&pool.get()?)?,
    )
}

