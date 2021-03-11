use crate::handlers::account::{get_account_model, get_all_accounts, update_account};
use crate::handlers::campaign_state::process_click;
use crate::handlers::crud::process_crud;
use crate::handlers::health::get_team_id;
use crate::handlers::{
    auth::{login, logout},
    crud,
    health::get_health,
    invitation,
    user::{create_user, get_user},
};
use crate::utils::middleware::auth::Auth as AuthMiddleware;
use crate::utils::middleware::click_processor::ClickProcessor;
use actix_files::Files;
use actix_web::web::{get, post, resource};
use actix_web::{web, HttpResponse};
use ad_buy_engine::constant::apis::private::{
    API_CRUD_ELEMENT, API_GET_ACCOUNT, API_POST_ACCOUNT, API_URL_LOGOUT,
};
use ad_buy_engine::constant::apis::public::{
    API_URL_CONFIRM_EMAIL_INVITATION, API_URL_CREATE_INVITATION, API_URL_CREATE_REGISTER,
    API_URL_LOGIN,
};
use ad_buy_engine::constant::local_system_location::{
    DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC, DIRECTORY_LOCATION_MAIN_PUBLIC_TERTIARY_STATIC,
    DIRECTORY_LOCATION_MAIN_SECURE_STATIC,
};
use ad_buy_engine::string_manipulation::backend::api_path_builder::{
    parse_api_v2_url, parse_v1_api, trim_api_v1,
};

pub fn private_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .wrap(AuthMiddleware)
            .service(web::scope("/auth").route(
                parse_v1_api("auth", API_URL_LOGOUT, false).as_str(),
                web::delete().to(logout),
            ))
            .service(resource(trim_api_v1(API_CRUD_ELEMENT)).route(post().to(process_crud)))
            .service(resource(trim_api_v1(API_GET_ACCOUNT)).route(get().to(get_account_model)))
            .service(resource(trim_api_v1(API_POST_ACCOUNT)).route(post().to(update_account))),
    )
    .service(
        web::scope("/secure").wrap(AuthMiddleware).service(
            Files::new("", DIRECTORY_LOCATION_MAIN_SECURE_STATIC)
                .index_file("index.html")
                .use_last_modified(true),
        ),
    );
}
