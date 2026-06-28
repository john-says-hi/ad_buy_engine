use crate::campaign_agent::CampaignAgent;
use crate::api::campaign_state::process_click;
use crate::helper_functions::{rate_limit, ssl_config};
use crate::private_routes::private_routes;
use crate::public_routes::public_routes;
use crate::utils::authentication::get_identity_service;
use crate::utils::cache::add_cache;
use crate::utils::config::CONFIG;
use crate::utils::database::{establish_connection};
use crate::utils::middleware::click_processor::ClickProcessor;
use crate::utils::state::init_state;
use actix_cors::Cors;
use actix_files::Files;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_service::Service;
use actix_web::client::Client;
use actix_web::http::header;
use actix_web::http::header::HOST;
use actix_web::web::{get, resource, scope, Data, JsonConfig};
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use ad_buy_engine::constant::local_system_location::DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC;
use ad_buy_engine::constant::server_info::CAMPAIGN_SERVER_IP_PORT_TERSE;
use ad_buy_engine::data::backend_models::campaign::CampaignModel;
use ad_buy_engine::data::elements::campaign::Campaign;
use chrono::Duration as ChronoDuration;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use futures::executor;
use futures::FutureExt;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use r2d2_diesel::ConnectionManager;
use std::sync::mpsc;
use std::time::Duration;
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use diesel_migrations::run_pending_migrations;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = establish_connection();
diesel_migrations::
    run_pending_migrations(&pool.clone().get().expect("hyuu"))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    let mut filtered_restored: Vec<Campaign> = {
        use crate::schema::campaigns::dsl::campaigns;
        campaigns
            .load::<CampaignModel>(&pool.clone().get().expect("4rgfsadf"))
            .unwrap()
            .iter()
            .cloned()
            .map(|s| s.into())
            .collect::<Vec<Campaign>>()
    };

    filtered_restored.iter().filter(|s| {
        s.last_clicked.timestamp() < Utc::now().timestamp() + ChronoDuration::days(3).num_seconds()
    });

    let app_state = init_state(filtered_restored);
    let store = MemoryStore::new();
    let campaign_agent = CampaignAgent::start("campaign_server:1488");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(RedirectSchemeBuilder::new().enable(true).build())
            .configure(add_cache)
            .wrap(
                Cors::new()
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .data(JsonConfig::default().limit(4_096_000))
            .wrap(get_identity_service())
            .data(Client::new())
            .data(pool.clone())
            .app_data(app_state.clone())
            // .service(
            //     scope("/click")
            //         .wrap(rate_limit(25, 60, store.clone()))
            //         .service(resource("/{campaign_id}").route(get().to(process_click))),
            // )
            // .wrap(rate_limit(100, 60, store.clone()))
            .wrap_fn(|req, srv| {
                println!("\n");
                srv.call(req).map(|res| res)
            })
            .configure(public_routes)
            .configure(private_routes)
            .service(
                scope("").default_service(
                    Files::new("", DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC)
                        .index_file("index.html")
                        .use_last_modified(true),
                ),
            )
            .data(campaign_agent.clone())
    })
        .bind("campaign_server:80")?
    .bind_openssl("campaign_server:443", ssl_config())?
    .workers(4)
    .run();

    server.await
}
