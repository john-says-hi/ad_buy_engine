use crate::schema::users;
use crate::utils::authentication::hash;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::user::UserModel;
use ad_buy_engine::UserResponse;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: String,
    pub account_id: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

/// Find a user by the user's id or error out
pub fn find(pool: &PgPool, id: Uuid) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id as user_id, users};

    let not_found = format!("User {} not found", id);
    let conn = pool.get()?;
    let user = users
        .filter(user_id.eq(user_id))
        .first::<UserModel>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(user.into())
}

pub fn find_by_auth(
    pool: &PgPool,
    user_email: &str,
    user_password: &str,
) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{email, password, users};

    let conn = pool.get()?;
    let user = users
        .filter(email.eq(user_email.to_string()))
        .filter(password.eq(user_password.to_string()))
        .first::<UserModel>(&conn)
        .map_err(|_| ApiError::Unauthorized("Invalid login".into()))?;
    Ok(user.into())
}

pub fn create(
    pool: &PgPool,
    new_user: UserModel,
    account: AccountModel,
) -> Result<UserResponse, ApiError> {
    use crate::schema::accounts::dsl::accounts;
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;
    println!("new user id {:?}",&new_user.id);
    println!("new account id {:?}",&account.id);
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)?;

    diesel::insert_into(accounts)
        .values(&account)
        .execute(&conn)?;

    Ok(new_user.clone().into())
}

// pub fn update(pool: &PgPool, update_user: &UpdateUser) -> Result<UserResponse, ApiError> {
//     use crate::schema::users::dsl::{user_id, users};
//
//     let conn = pool.get()?;
//     diesel::update(users)
//         .filter(id.eq(update_user.id.clone()))
//         .set(update_user)
//         .execute(&conn)?;
//     find(&pool, Uuid::parse_str(&update_user.id)?)
// }
//
// pub fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), ApiError> {
//     use crate::schema::users::dsl::{user_id, users};
//
//     let conn = pool.get()?;
//     diesel::delete(users)
//         .filter(id.eq(user_id.to_string()))
//         .execute(&conn)?;
//     Ok(())
// }

impl From<NewUser> for UserModel {
    fn from(user: NewUser) -> Self {
        Self {
            id: user.id,
            account_id: user.account_id,
            email: user.email,
            password: hash(&user.password),
            last_updated: Utc::now().timestamp(),
        }
    }
}
