use crate::model::account::{query_account, return_all_accounts, update_account_database};
use crate::utils::authentication::{decode_jwt, PrivateClaim};
use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix_identity::Identity;
use actix_web::web::{block, Data, Json};
use ad_buy_engine::data::account::Account;

pub async fn get_account_model(
    pool: Data<PoolType>,
    id: Identity,
) -> Result<Json<Account>, ApiError> {
    let restored_identity: PrivateClaim =
        decode_jwt(&id.identity().expect("g3qw")).map_err(|e| e)?;
    respond_json(
        block(move || query_account(&pool, restored_identity.account_id))
            .await?
            .into(),
    )
}

pub async fn update_account(
    pool: Data<PoolType>,
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
    pool: Data<PoolType>,
    id: Identity,
) -> Result<Json<Vec<Account>>, ApiError> {
    let restored_identity: PrivateClaim =
        decode_jwt(&id.identity().expect("g3qw")).map_err(|e| e)?;

    let mut list = block(move || return_all_accounts(&pool)).await?;
    let resp = list
        .iter()
        .map(|s| s.clone().into())
        .collect::<Vec<Account>>();

    respond_json(resp)
}
