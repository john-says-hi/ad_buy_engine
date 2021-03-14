#![feature(in_band_lifetimes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;
pub mod dns;
pub use ad_buy_engine::schema;

use crate::server::server;

pub mod helper_functions;
pub mod campaign_agent;
pub mod email_service;
pub mod api;
pub mod db;
mod private_routes;
mod public_routes;
mod server;
pub mod management;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
