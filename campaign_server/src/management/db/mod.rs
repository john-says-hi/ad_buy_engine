use crate::utils::errors::ApiError;
use crate::utils::database::{PgPool, get_conn};
use ad_buy_engine::data::backend_models::account::AccountModel;
use std::ops::Deref;
use ad_buy_engine::data::backend_models::user::UserModel;
use ad_buy_engine::data::backend_models::EmailModel;

pub mod email;

pub fn delete_all_accounts(pool: &PgPool) -> Result<usize, ApiError> {
	Ok(AccountModel::delete_all(get_conn(pool)?.deref())?)
}
pub fn delete_all_users(pool: &PgPool) -> Result<usize, ApiError> {
	Ok(UserModel::delete_all(get_conn(pool)?.deref())?)
}
pub fn delete_all_emails(pool: &PgPool) -> Result<usize, ApiError> {
	Ok(EmailModel::delete_all(get_conn(pool)?.deref())?)
}
