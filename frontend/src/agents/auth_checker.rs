use crate::utils::javascript::js_bindings::redirect_login;
use ad_buy_engine::AError;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use yew::agent::*;
use yew::format::Nothing;
use yew::prelude::*;
use yew_services::fetch::{FetchTask, Response};
use yew_services::{FetchService, IntervalService, Task};

pub enum Message {
    Ping,
    Ignore,
    AuthFailed,
}

#[derive(Deserialize, Serialize)]
/// Available timer requests
pub enum Request {
    Initialize,
}

#[derive(Deserialize, Serialize)]
pub struct AuthResponse;

pub struct AuthChecker {
    agent_link: AgentLink<AuthChecker>,
    callback: Callback<()>,
    fetch_task: Option<FetchTask>,
    timer_task: Option<Box<dyn Task>>,
}

impl Agent for AuthChecker {
    type Input = Request;
    type Message = Message;
    type Output = AuthResponse;
    type Reach = Context<Self>;

    /// Creates a new AuthChecker
    fn create(link: AgentLink<Self>) -> Self {
        Self {
            callback: link.callback(|_| Message::Ping),
            agent_link: link,
            fetch_task: None,
            timer_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Message::Ignore => self.fetch_task = None,

            Message::AuthFailed => {
                self.fetch_task = None;
                redirect_login()
            }

            Message::Ping => {
                let request = yew_services::fetch::Request::get("").body(Nothing).unwrap();

                let callback = self
                    .agent_link
                    .callback(|res: Response<Result<String, AError>>| {
                        if res.status().is_success() {
                            return Message::Ignore;
                        } else {
                            Message::AuthFailed
                        }
                    });

                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        match msg {
            Request::Initialize => {
                let handle = IntervalService::spawn(Duration::from_secs(30), self.callback.clone());
                self.timer_task = Some(Box::new(handle));
            }
        }
    }
}
