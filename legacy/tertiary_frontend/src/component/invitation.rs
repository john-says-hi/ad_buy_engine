use crate::prelude::*;
use crate::routes::AppRoute;
use anyhow::Error;

use crate::alert;
use ad_buy_engine::constant::apis::public::API_URL_CREATE_INVITATION;
use ad_buy_engine::InvitationRequest;
use mailchecker::{blacklist, is_valid};
use serde::Serialize;
use yew::format::{Json, Nothing};
use yew::services::fetch::{Request, Response};
use yew::services::{DialogService, FetchService};
use yew::{
    html, prelude::*, services::fetch, virtual_dom::VNode, Component, ComponentLink, Html,
    InputData, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*, Switch};

pub struct InvitationComponent {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<fetch::FetchTask>,
    inputs_disabled: bool,
    invitation_button_disabled: bool,
    email: String,
}

impl InvitationComponent {
    fn update_button_state(&mut self) {
        self.invitation_button_disabled = self.email.is_empty();
    }
}

pub enum Message {
    Ignore,
    SubmitSuccess,
    Failed,
    InvitationRequest,
    UpdateEmail(String),
}

impl Component for InvitationComponent {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Message::Ignore);
        let router = RouteAgent::bridge(callback);

        InvitationComponent {
            link,
            router,
            fetch_task: None,
            inputs_disabled: false,
            invitation_button_disabled: false,
            email: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::InvitationRequest => {
                let data = InvitationRequest {
                    email: self.email.clone(),
                };

                let request = Request::post(API_URL_CREATE_INVITATION)
                    .header("Content-Type", "application/json")
                    .body(Json(&data))
                    .unwrap();

                let callback = self
                    .link
                    .callback(|response: Response<Json<Result<(), Error>>>| {
                        let (x, Json(body)) = response.into_parts();
                        if x.status.is_success() {
                            return Message::SubmitSuccess;
                        }
                        Message::Failed
                        //
                    });

                let task = FetchService::fetch(request, callback).unwrap();

                self.fetch_task = Some(task);
            }

            Message::UpdateEmail(email) => {
                self.email = email;
                self.update_button_state();
            }

            Message::SubmitSuccess => {
                let slim_email = shorten_email(self.email.clone());
                self.fetch_task = None;
                self.router
                    .send(ChangeRoute(AppRoute::CheckYourEmail(slim_email).into()))
            }
            Message::Failed => {
                DialogService::alert("Failed Submission, Try Again.");
                self.fetch_task = None;
            }
            Message::Ignore => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
        <div>
                    <div class="  uk-preserve-color ">
                        <div class="uk-container">
                            <div class="uk-child-width-expand uk-text-center" uk-grid="">
                                <div class=""></div>
                                    <div class="uk-width-1-2 uk-section">
                                        <div class=" uk-card uk-card-default uk-card-body   uk-border-rounded",>
                                            <h2>{"Request Invitation"}</h2>

                                                           <form>
                                                                <fieldset class="uk-fieldset",>

                                                                    <input class="uk-input uk-margin",
                                                                        placeholder={"Email"},
                                                                        disabled=self.inputs_disabled,
                                                                        value=&self.email,
                                                                        oninput=self.link.callback(|e:InputData| Message::UpdateEmail(e.value)),
                                                                        />

                                                                </fieldset>
                                                            </form>

                                                                    <button class="uk-button uk-button-primary",
                                                                        type="submit",
                                                                        disabled=self.invitation_button_disabled,
                                                                        onclick=self.link.callback(move |_| Message::InvitationRequest),
                                                                        >{"Get Invitation!"}</button>
                                        </div>
                                    </div>
                                <div class="">

                                </div>
                            </div>
                        </div>
                    </div>
        </div>
                    }
    }
}

fn shorten_email(eml: String) -> String {
    let mut c = 0;
    let mut char_vec = vec![];
    for charr in eml.chars().into_iter() {
        if charr == '@' {
            c = 1;
        }
        if c == 1 {
            char_vec.push(charr)
        }
    }
    char_vec.remove(0);
    let mut s_eml = String::new();
    for letter in char_vec.iter() {
        s_eml.push(*letter);
        let x = *letter;
        let y = letter;
        let z = *letter;
    }
    s_eml
}
