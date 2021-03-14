use crate::appstate::app_state::AppState;
use crate::{alert, notify_primary};
use ad_buy_engine::constant::apis::private::API_GET_ACCOUNT;
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::sync::sync_update::{ SyncVisitsResponse};
use ad_buy_engine::AError;
use std::cell::RefCell;
use std::rc::Rc;
use yew::agent::*;
use yew::format::{Json, Nothing};
use yew_services::fetch::{FetchTask, Request, Response};
use yew_services::FetchService;

mod clone;
mod create;
mod deactivate;
mod reactivate;
mod sync;
mod update;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FetchResponse {
    ReturnAccount(Account),
    // ReturnUserAccountData(),
    // ReturnElements(SyncElementsResponse),
    // ReturnVisits(SyncVisitsResponse),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FetchRequest {
    GetAccount,
    // SyncUserAccountData,
    // SyncElements,
    // SyncVisits,
}

pub struct FetchAgent {
    pub link: AgentLink<Self>,
    pub fetch_task: Option<FetchTask>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Msg {
    Ignore,
    Failed,
    // SyncElements(SyncElementsResponse),
    // SyncVisits(SyncVisitsResponse),
    // SyncUserAccountData(Account),
    FetchAccount((Account, HandlerId)),
}

impl Agent for FetchAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = FetchRequest;
    type Output = FetchResponse;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) {
        match _msg {
            Msg::FetchAccount((data, hid)) => {
                self.fetch_task = None;
                self.link.respond(hid, FetchResponse::ReturnAccount(data));
            }

            Msg::Ignore => {}

            Msg::Failed => {
                self.fetch_task = None;
                notify_primary("Fetch Failed")
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, hid: HandlerId) {
        match msg {
            FetchRequest::GetAccount => {
                let request = Request::get(API_GET_ACCOUNT)
                    .body(Nothing)
                    .expect("F34segr");

                let callback =
                    self.link
                        .callback(move |response: Response<Json<Result<Account, AError>>>| {
                            let (meta, Json(body)) = response.into_parts();

                            if meta.status.is_success() {
                                if let Ok(data) = body {
                                    Msg::FetchAccount((data, hid))
                                } else {
                                    Msg::Failed
                                }
                            } else {
                                Msg::Failed
                            }
                        });

                let fetch_task = FetchService::fetch(request, callback).expect("f43ss");
                self.fetch_task = Some(fetch_task)
            }
        }
    }
}
