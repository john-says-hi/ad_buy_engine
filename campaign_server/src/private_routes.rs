use crate::api::account::{get_account_model, get_all_accounts, update_account};
use crate::api::campaign_state::process_click;
use crate::api::crud_element::process_crud;
use crate::api::health::get_team_id;
use crate::api::{
    auth::{login, logout},
    crud_element,
    health::get_health,
    invitation,
    user::{create_user, get_user},
};
use crate::api::sync_elements;
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
            .service(resource("/auth/logout").route(web::delete().to(logout)))
        .service(resource("/get_account").route(get().to(get_account_model)))
        .service(resource("/crud_element").route(post().to(process_crud)))
            .service(resource("/account").route(post().to(update_account)))
            .service(resource("/sync_elements").route(post().to(sync_elements::sync)))
    )
    .service(
        web::scope("/secure").wrap(AuthMiddleware).service(
            Files::new("", DIRECTORY_LOCATION_MAIN_SECURE_STATIC)
                .index_file("index.html")
                .use_last_modified(true),
        ),
    );
}
