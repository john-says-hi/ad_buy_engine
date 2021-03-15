#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate educe;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate strum_macros;
#[cfg(feature = "backend")]
#[macro_use]
extern crate diesel;

#[macro_use]
pub mod macros;
pub mod constant;
pub mod data;

pub mod string_manipulation;

#[cfg(feature = "backend")]
pub mod schema;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Country {
    Global,
    ISOCountry(data::lists::country::Country),
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Country::Global => "Global".to_string(),
            Country::ISOCountry(iso_country) => iso_country.to_string(),
        }
    }
}

pub type AError = anyhow::Error;
pub type ISOLanguage = LanguageCode;

pub use crate::data::iso_language::{LanguageCode, ParseError as ISOLangParseError};
use boyer_moore_magiclen::BMByte;
use either::Either;
pub use ipnet;
use uuid::Uuid;
use weighted_rs::{SmoothWeight, Weight};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InvitationRequest {
    pub email: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub company_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Register {
    pub email: String,
    pub username: String,
    pub team_name: String,
    pub password: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterAnother {
    pub email: String,
    pub username: String,
    pub password: String,
}
