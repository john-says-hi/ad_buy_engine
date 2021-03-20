#![recursion_limit = "2560"]
#![allow(clippy::clippy::into_iter_on_ref)]
#![allow(clippy::ptr_arg)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unreachable_patterns)]
#[macro_use]
extern crate educe;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate dotenv_codegen;
extern crate wee_alloc;

use wasm_bindgen::prelude::*;
#[macro_use]
pub mod macros;
pub mod agents;
pub mod appstate;
pub mod components;
pub mod error;
pub mod prelude;
pub mod utils;

pub use ad_buy_engine;

use yew_router::prelude::*;

use crate::agents::fetch_agent::{FetchRequest, FetchResponse};
use crate::agents::tick_tock::{TickTock, TickTockRequest};
use crate::agents::{AuthChecker, FetchAgent};
use crate::appstate::app_state::{AppState, STATE};
use crate::components::account_component::AccountComponent;
use crate::components::app_bar::AppBar;
use crate::components::main_component::MainComponent;
use crate::prelude::*;
use crate::utils::javascript::js_bindings;
use crate::utils::routes::AppRoute;
use crate::utils::uikit::NotificationStatus;
use ad_buy_engine::constant::apis::private::API_GET_ACCOUNT;
use ad_buy_engine::constant::browser_storage_keys::{SYNC_HISTORY_KEY, USER_ACCOUNT_STATE_KEY};
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::sync::SyncHistoryLedger;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::AError;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_services::fetch::{FetchTask, Request, Response};
use yew_services::storage::Area;
use yew_services::{FetchService, StorageService};

pub struct RootComponent {
    child_component: Option<AppRoute>,
    router_agent: Box<dyn Bridge<RouteAgent<()>>>,
    fetch_agent: Box<dyn Bridge<FetchAgent>>,
    tt: Box<dyn Bridge<TickTock>>,
    state: Rc<RefCell<AppState>>,
    visits: Arc<RwLock<Vec<Visit>>>,
    link: ComponentLink<Self>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    Fetch(FetchResponse),
    Route(Route<()>),
    Ignore,
}

impl Component for RootComponent {
    type Message = Message;
    type Properties = ();

    fn create(mut props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut tt = TickTock::bridge(link.callback(|_| Message::Ignore));
        let mut router_agent = RouteAgent::bridge(link.callback(Message::Route));
        let mut fetch_agent = FetchAgent::bridge(link.callback(|res| Message::Fetch(res)));
        fetch_agent.send(FetchRequest::GetAccount);

        let mut state = Rc::new(RefCell::new(AppState::init()));

        let nav = state.borrow().return_app_route();

        Self {
            child_component: Some(nav),
            router_agent,
            fetch_agent,
            state: Rc::clone(&state),
            visits: Arc::new(RwLock::new(vec![])),
            link,
            tt,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        notify_primary("Props Change Detected from root");
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Fetch(response) => match response {
                FetchResponse::ReturnAccount(account) => {
                    *self.state.borrow().account.borrow_mut() = account;
                    self.state.borrow_mut().store_account();
                    self.state
                        .borrow()
                        .sync_ledger
                        .borrow_mut()
                        .update_account_update_date();
                    self.state.borrow().store_sync_ledger();
                    
                    let sync_elem_req = self.state.borrow().request_sync_elements();
                    self.fetch_agent.send(FetchRequest::SyncElements(sync_elem_req));
                }
                
                FetchResponse::ReturnSyncElemResponse(res)=>{
                    self.state.borrow().sync_update(res);
                }
            },

            Message::Route(route) => {
                self.child_component = AppRoute::switch(route);
                self.state.borrow().selected_elements.borrow_mut().clear();
                self.tt.send(TickTockRequest::Tick);
            }
            Message::Ignore => {}
        }
        true
    }

    fn view(&self) -> Html {
        if let Some(cc) = &self.child_component {
            match cc {
                AppRoute::Dashboard => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Campaign => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Offers => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Landers => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Sequences => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Funnels => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Traffic => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::OfferSources => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Connection => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::ISPCarrier => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::MobileCarrier => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Proxy => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Devices => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Brand => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Model => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::OS => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::OSVersion => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Browser => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::BrowserVersion => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::DateDay => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::DateMonth => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::DayOfWeek => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::HourOfDay => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Conversions => {
                    VNode::from(html! {<MainComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::Account => {
                    VNode::from(html! {<AccountComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::CustomConversions => {
                    VNode::from(html! {<AccountComponent state=Rc::clone(&self.state)  />})
                }
                AppRoute::ReferrerHandling => {
                    VNode::from(html! {<AccountComponent state=Rc::clone(&self.state)  />})
                }
                _ => {
                    html! { "404 Page Not Found" }
                }
            }
        } else {
            html! {
            {"404 No CC Available"}
            }
        }
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<RootComponent>();
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn alert(msg: &str, status: NotificationStatus) {
    js_bindings::uikit_notify(
        JsValue::from_str(msg),
        JsValue::from_str(status.to_string().as_str()),
    );
}

pub fn alert_success(msg: &str) {
    alert(msg, NotificationStatus::Success)
}
pub fn notify_primary(msg: &str) {
    alert(msg, NotificationStatus::Primary)
}
pub fn notify_warning(msg: &str) {
    alert(msg, NotificationStatus::Warning)
}
pub fn notify_danger(msg: &str) {
    alert(msg, NotificationStatus::Danger)
}
pub fn notify_debug(msg: String) {
    alert(msg.as_str(), NotificationStatus::Danger)
}
