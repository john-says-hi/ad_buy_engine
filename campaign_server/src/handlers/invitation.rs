use crate::model::invitation;
use crate::model::invitation::remove;
use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use crate::utils::helpers::{respond_json, respond_ok};
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use ad_buy_engine::data::backend_models::invitation::Invitation;
use ad_buy_engine::InvitationRequest;
use chrono::Local;
use diesel::prelude::*;
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

pub async fn create(
    pool: Data<PoolType>,
    params: Json<InvitationRequest>,
) -> Result<Json<String>, ApiError> {
    let _pool = pool.clone();
    let _params = params.0.email.clone();

    block(move || crate::model::invitation::dedup(&_pool, params.email.clone()))
        .await
        .map_err(|e| {
            println!("Error: {}", "543g34");
            e
        })?;

    let new = Invitation {
        invitation_id: Uuid::new_v4().to_string(),
        email: _params,
        email_confirmed: false,
        expires_at: Local::now().naive_local() + chrono::Duration::hours(24),
    };

    block(move || crate::model::invitation::new(&pool, &new)).await?;

    respond_json("check your email".to_string())
}

pub async fn update(_id: Path<Uuid>, pool: Data<PoolType>) -> Result<HttpResponse, ApiError> {
    let pool_a = pool.clone();
    let pool_b = pool.clone();

    let mut item = block(move || invitation::find(&pool, *_id)).await?;
    println!("1");

    if item.expires_at > chrono::Local::now().naive_local() {
        println!("1");

        item.email_confirmed = true;
        assert_eq!(true, item.email_confirmed);

        block(move || invitation::update(&pool_a, &item)).await?;
        Ok(HttpResponse::Found()
            .header(actix_web::http::header::LOCATION, "/tertiary/#register")
            .finish())
    } else {
        block(move || remove(&pool_b, &item.invitation_id)).await?;
        Err(ApiError::BadRequest("Old Invitation".into()))
    }
}

pub async fn delete(_id: Path<Uuid>, pool: Data<PoolType>) -> Result<HttpResponse, ApiError> {
    block(move || crate::model::invitation::remove(&pool, &_id.to_string())).await?;
    respond_ok()
}
