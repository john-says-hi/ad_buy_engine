use crate::utils::database::{PgPool, get_conn};
use ad_buy_engine::data::backend_models::EmailModel;
use crate::utils::errors::ApiError;
use std::ops::Deref;

pub fn get_all_emails(pool: &PgPool) -> Result<Vec<EmailModel>, ApiError> {
	Ok(EmailModel::all(get_conn(pool)?.deref())?)
}