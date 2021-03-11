use crate::data::user::User;
#[cfg(feature = "backend")]
use crate::schema::*;
use crate::UserResponse;
use chrono::{Local, NaiveDateTime, Utc};
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "user_table",
    primary_key("user_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserModel {
    pub user_id: String,
    pub account_id: String,
    pub email: String,
    pub password: String,
    pub last_updated: i64,
}

impl From<User> for UserModel {
    fn from(u: User) -> Self {
        Self {
            user_id: u.user_id.to_string(),
            account_id: u.account_id.to_string(),
            email: u.email,
            password: u.password,
            last_updated: Utc::now().timestamp(),
        }
    }
}

impl From<UserModel> for User {
    fn from(u: UserModel) -> Self {
        Self {
            user_id: Uuid::parse_str(&u.user_id).unwrap(),
            account_id: Uuid::parse_str(&u.account_id).unwrap(),
            email: u.email,
            password: u.password,
        }
    }
}

impl From<UserModel> for UserResponse {
    fn from(m: UserModel) -> Self {
        Self {
            id: Uuid::parse_str(&m.user_id).expect("G%$sef"),
            account_id: Uuid::parse_str(&m.account_id).expect("R3gsaef"),
            email: m.email,
        }
    }
}
