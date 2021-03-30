use crate::management::db;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_ok;
use actix_web::web::{block, Data};
use actix_web::HttpResponse;
use ad_buy_engine::data::backend_models::EmailModel;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub async fn reset_users_accounts_emls(pool: Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let local_pool = pool.clone();
    block(move || db::debug::delete_all_accounts(&local_pool)).await?;
    let local_pool = pool.clone();
    block(move || db::debug::delete_all_emails(&local_pool)).await?;
    let local_pool = pool.clone();
    block(move || db::debug::delete_all_users(&local_pool)).await?;
    respond_ok()
}

pub async fn delete_all_funnels(pool: Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let local_pool = pool.clone();
    block(move || db::debug::delete_all_funnels(&local_pool)).await?;
    respond_ok()
}
