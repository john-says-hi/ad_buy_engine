use crate::db::account_depricating::{query_account, return_all_accounts, update_account_database};
use crate::utils::authentication::{decode_jwt, PrivateClaim};
use crate::utils::database::{PgPool, get_conn};
use crate::utils::errors::ApiError;
use crate::utils::helpers::{respond_json, respond_ok};
use actix_identity::Identity;
use actix_web::web::{block, Data, Json};
use ad_buy_engine::data::account::Account;
use actix_web::HttpResponse;
use diesel::RunQueryDsl;
use std::ops::Deref;

pub async fn get_account_model(
    pool: Data<PgPool>,
    id: Identity,
) -> Result<Json<Account>, ApiError> {
    let restored_identity: PrivateClaim =
        decode_jwt(&id.identity().expect("g3qw")).map_err(|e| e)?;
    println!("acc id :  {}",&restored_identity.account_id);
    println!("user id :  {}",&restored_identity.user_id);
    respond_json(
        block(move || query_account(&pool, restored_identity.account_id))
            .await?
            .into(),
    )
}

pub async fn update_account(
    pool: Data<PgPool>,
    id: Identity,
    payload: Json<Account>,
) -> Result<Json<Account>, ApiError> {
    let restored_identity: PrivateClaim =
        decode_jwt(&id.identity().expect("g3qw")).map_err(|e| e)?;
    respond_json(
        block(move || {
            update_account_database(
                &pool,
                restored_identity.account_id,
                payload.into_inner().into(),
            )
        })
        .await?
        .into(),
    )
}

pub async fn get_all_accounts(
    pool: Data<PgPool>,
) -> Result<Json<Vec<Account>>, ApiError> {

    let mut list = block(move || return_all_accounts(&pool)).await?;
    let resp = list
        .iter()
        .map(|s| {
            println!("{:?}", &s);
            s.clone().into()
        })
        .collect::<Vec<Account>>();

    respond_json(resp)
}


