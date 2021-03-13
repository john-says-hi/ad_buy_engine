use crate::dns::dns_cname::request_subdomain;
use crate::model::user::*;
use crate::schema::account_table::dsl::{account_id, account_table};
use crate::schema::invitation_table::dsl::invitation_table;
use crate::schema::user_table::dsl::{user_id, user_table};
use crate::utils::authentication::hash;
use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use crate::utils::helpers::{respond_json, respond_ok};
use actix_web::client::Client;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::user::User;
use ad_buy_engine::{CreateUserRequest, UserResponse};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use crate::model;

pub async fn get_user(
    uid: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = block(move || find(&pool, uid.clone())).await?;
    respond_json(user)
}

pub async fn create_user(
    client: Data<Client>,
    pool: Data<PoolType>,
    params: Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let pool_a = pool.clone();
    let pool_b = pool.clone();
    let params_a = params.clone();
    let params_b = params.clone();

    let inv =
        block(move || crate::model::invitation::find_by_email(&pool_a, params_a.email)).await?;

    if inv.email_confirmed {
        let new_user = User {
            user_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            email: inv.email.clone(),
            password: hash(&params_b.password),
        };

        let new_account = Account::from(new_user.clone());
        
            println!("Sub Domain created, {}", request_subdomain(client, new_account
                .domains_configuration
                .subdomain
                .clone()
                .to_string()).await?);

        let user = block(move || create(&pool, new_user.into(), new_account.into())).await?;
        block(move || model::invitation::remove(&pool_b, &inv.invitation_id)).await?;

        respond_json(user.into())
    } else {
        Err(ApiError::BadRequest(
            "Invitation Not Verified, Check Your Email".into(),
        ))
    }
}
//
// /// Update a user
// pub async fn update_user(
//     user_id: Path<Uuid>,
//     pool: Data<PoolType>,
//     params: Json<UpdateUserRequest>,
// ) -> Result<Json<UserResponse>, ApiError> {
//     validate(&params)?;
//
//     let update_user = UpdateUser {
//         id: user_id.to_string(),
//         email: params.email.to_string(),
//         password: "".to_string(),
//         updated_by: user_id.to_string(),
//     };
//     let user = block(move || update(&pool, &update_user)).await?;
//     respond_json(user.into())
// }
