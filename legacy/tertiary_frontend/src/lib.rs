#![recursion_limit = "2560"]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate serde_json;

pub mod component;
// pub mod cookie_service;
pub mod javascript;
pub mod prelude;
pub mod routes;
pub mod text;

use crate::{
    component::{
        check_your_email::CheckYourEmail, footer::PublicFooter, header::PublicHeader,
        home::PublicHomePage, invitation::InvitationComponent, join_the_team::JoinTheTeamComponent,
        login::LoginComponent, register::RegisterComponent,
    },
    prelude::*,
};

use crate::routes::AppRoute;

use yew::{agent::Bridged, format::Json, html, prelude::*, services::fetch::FetchTask};

use yew::virtual_dom::VNode;
use yew_router::{
    agent::{RouteAgent, RouteRequest::ChangeRoute},
    route::Route,
    switch::{AllowMissing, Switch},
};
pub mod uikit;
use crate::uikit::NotificationStatus;
use uikit::UIkitService;
use wasm_bindgen::prelude::*;

pub fn alert(msg: &str) {
    UIkitService::new().notify(msg, NotificationStatus::Danger);
}

pub const TEAM_CS: &'static str = "QErvq35";
pub const TEAM_CN: &'static str = "%$Gw4erg";
pub const AUTH_C: &'static str = "Asdfasdf";

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<RootComponent>();
}

pub struct RootComponent {
    child_component: Option<AppRoute>,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent<()>>>,
}

pub enum Message {
    Route(Route<()>),
    Ignore,
}

impl Component for RootComponent {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        // Create needed services
        let mut fetch_task = None;
        let mut router_agent = RouteAgent::bridge(link.callback(Message::Route));
        //        router_agent.send(ChangeRoute(AppRoute::Home.into()));

        // check for existing path  to convert into route
        let route = yew_router::service::RouteService::<()>::new().get_route();
        //        alert(route.as_str());

        let child_component = match route.as_str() {
            "/tertiary/#register" => Some(AppRoute::Register),
            "/tertiary/#login" => Some(AppRoute::Login),
            "/tertiary/#invitation" => Some(AppRoute::Invitation),
            //            "/#join_the_team" => Some(AppRoute::RegisterAnotherUser),
            //            "/#login" => Some(AppRoute::Login),
            //            "/#invitation" => Some(AppRoute::Invitation),
            _ => Some(AppRoute::Login),
        };

        Self {
            child_component,
            fetch_task,
            router_agent,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Ignore => {}
            Message::Route(route) => self.child_component = AppRoute::switch(route),
        }
        true
    }

    fn view(&self) -> VNode {
        if let Some(cc) = &self.child_component {
            match cc {
                // AppRoute::Home => VNode::from(html! {
                // <>
                //     <PublicHeader :/>
                //         <div>
                //             <PublicHomePage :/>
                //         </div>
                //     <PublicFooter :/>
                // </>
                // }),
                AppRoute::Login => VNode::from(html! {
                <div>
                        <div>
                            <LoginComponent :/>
                        </div>
                </div>
                }),

                // USER INIT
                AppRoute::CheckYourEmail(es) => {
                    html! {
                                        <>
                                            <div class="  uk-preserve-color ">
                                                <div class="uk-container">
                                                    <div class="uk-child-width-expand uk-text-center" uk-grid="">
                                                        <div class=""></div>
                                                            <div class="uk-width-1-2 uk-section">
                                                                <div class=" uk-card uk-card-default uk-card-body   uk-border-rounded",>
                                                                <h2>{"Check Your Email and Get Ready To Lock 'n Load, it's campaign time..."}</h2>
                                                                <h4>{"Click To Go To "} <a href=format!("https://{}",es) > {es}</a></h4>
                                                                </div>
                                                            </div>
                                                        <div class=""></div>
                                                    </div>
                                                </div>
                                            </div>
                    //                            <CheckYourEmail :/>
                                        </>
                                        }
                }

                AppRoute::Register => {
                    html! {
                    <div>
                        <RegisterComponent :/>
                    </div>
                    }
                }

                AppRoute::RegisterAnotherUser => {
                    html! {
                        <div>
                            <JoinTheTeamComponent :/>
                        </div>
                    }
                }

                AppRoute::Invitation => {
                    html! {
                        <div>
                            <InvitationComponent :/>
                        </div>
                    }
                }
            }
        } else {
            VNode::from(html! {
            <div>
                    <div>
                        <LoginComponent :/>
                    </div>
            </div>
            })
            // html! { "No child master_campaign_element available" }
        }
    }
}
