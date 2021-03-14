use crate::utils::database::PgPool;
use actix_web::web::{Data, block};
use actix_web::HttpResponse;
use crate::utils::errors::ApiError;
use diesel::RunQueryDsl;
use super::db;
use crate::utils::helpers::respond_ok;

pub mod email;

pub async fn reset_users_accounts_emls(
	pool: Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
	let local_pool=pool.clone();
	block(move || db::delete_all_accounts(&local_pool) ).await?;
	let local_pool=pool.clone();
	block(move || db::delete_all_emails(&local_pool) ).await?;
	let local_pool=pool.clone();
	block(move || db::delete_all_users(&local_pool) ).await?;
	respond_ok()
}
