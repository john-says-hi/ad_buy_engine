use crate::db::user_depricated::find_by_auth;
use crate::utils::authentication::{create_jwt, hash, PrivateClaim};
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::{respond_json, respond_ok};
use actix_identity::Identity;
use actix_web::web::{block, Data, HttpResponse, Json};
use ad_buy_engine::{LoginRequest, UserResponse};
use serde::Serialize;
use validator::Validate;

pub async fn login(
    id: Identity,
    pool: Data<PgPool>,
    params: Json<LoginRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let hashed = hash(&params.password);
    let user = block(move || find_by_auth(&pool, &params.email, &hashed)).await?;

    let private_claim = PrivateClaim::new(user.id, user.account_id, user.email.clone());
    let jwt = create_jwt(private_claim)?;

    id.remember(jwt);
    respond_json(user.into())
}

pub async fn logout(id: Identity) -> Result<HttpResponse, ApiError> {
    id.forget();
    respond_ok()
}
