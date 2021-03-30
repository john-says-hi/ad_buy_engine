use crate::utils::database::{get_conn, PgPool};
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::funnel::FunnelModel;
use ad_buy_engine::data::backend_models::user::UserModel;
use ad_buy_engine::data::backend_models::DatabaseCommunication;
use ad_buy_engine::data::backend_models::EmailModel;
use std::ops::Deref;

pub fn delete_all_funnels(pool: &PgPool) -> Result<usize, ApiError> {
    Ok(FunnelModel::delete_all(get_conn(pool)?.deref())?)
}

pub fn delete_all_accounts(pool: &PgPool) -> Result<usize, ApiError> {
    Ok(AccountModel::delete_all(get_conn(pool)?.deref())?)
}
pub fn delete_all_users(pool: &PgPool) -> Result<usize, ApiError> {
    Ok(UserModel::delete_all(get_conn(pool)?.deref())?)
}
pub fn delete_all_emails(pool: &PgPool) -> Result<usize, ApiError> {
    Ok(EmailModel::delete_all(get_conn(pool)?.deref())?)
}
