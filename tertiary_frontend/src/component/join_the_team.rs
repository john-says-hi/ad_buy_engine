use crate::alert;
use crate::prelude::*;
use crate::routes::AppRoute;
use anyhow::Error;

use crate::javascript::js_bindings::login_redirect;
use ad_buy_engine::constant::API_URL_REGISTER_NEW_USER_TO_TEAM;
use ad_buy_engine::RegisterAnother;
use mailchecker::{blacklist, is_valid};
use yew::services::fetch::{Credentials, FetchOptions, Request, Response};
use yew::services::{DialogService, FetchService};
use yew::{
    format::Json, html, prelude::*, services::fetch, virtual_dom::VNode, Component, ComponentLink,
    Html, InputData, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*, Switch};

pub struct JoinTheTeamComponent {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<fetch::FetchTask>,
    inputs_disabled: bool,
    register_button_disabled: bool,
    email: String,
    username: String,
    password: String,
}

impl JoinTheTeamComponent {
    fn update_button_state(&mut self) {
        self.register_button_disabled =
            self.email.is_empty() || self.username.is_empty() || self.password.is_empty();
    }
}

pub enum Message {
    LoginFailed,
    LoginSuccess,
    RegisterSuccess,
    RegisterFailed,
    Ignore,
    RegisterRequest,
    UpdateEmail(String),
    UpdateUsername(String),
    UpdatePassword(String),
}

impl Component for JoinTheTeamComponent {
    type Message = Message;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Message::Ignore);
        let router = RouteAgent::bridge(callback);

        let email = String::new();

        JoinTheTeamComponent {
            link,
            router,
            fetch_task: None,
            inputs_disabled: false,
            register_button_disabled: false,
            email,
            username: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::RegisterRequest => {
                if !is_valid(self.email.as_str()) {
                    alert("INVALID EMAIL")
                } else if self.username.len() > 32 || self.username.len() < 4 {
                    alert("USERNAME MUST BE BETWEEN 4 AND 32 CHARS")
                } else if self.password.len() > 48 || self.password.len() < 12 {
                    alert("PASSWORD MUST BE OVER 12 CHARS AND UNDER 48")
                } else {
                    // VALID, SUBMIT
                    let data = RegisterAnother {
                        email: self.email.clone(),
                        username: self.username.clone(),
                        password: self.password.clone(),
                    };

                    let request = Request::post(API_URL_REGISTER_NEW_USER_TO_TEAM)
                        .header("Content-Type", "application/json")
                        .body(Json(&data))
                        .unwrap();

                    let callback =
                        self.link
                            .callback(|res: Response<Result<String, anyhow::Error>>| {
                                if res.status().is_success() {
                                    Message::RegisterSuccess
                                } else {
                                    Message::RegisterFailed
                                }
                            });
                    //
                    let task = fetch::FetchService::fetch(request, callback).unwrap();
                    //
                    self.fetch_task = Some(task);
                }
            }

            Message::UpdateEmail(email) => {
                self.email = email;
                self.update_button_state();
            }

            Message::UpdateUsername(username) => {
                self.username = username;
                self.update_button_state();
            }

            Message::UpdatePassword(new_password) => {
                self.password = new_password;
                self.update_button_state();
            }

            Message::RegisterSuccess => {
                self.fetch_task = None;
                alert("Welcome to the team! You may now login...");
                login_redirect()
            }

            Message::LoginSuccess => {
                self.fetch_task = None;
            }

            Message::LoginFailed => {
                self.fetch_task = None;

                login_redirect()
            }

            Message::RegisterFailed => {
                alert("Something went wrong. Try Again.");
                self.fetch_task = None;
            }

            Message::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        //        self.email = props.email;
        true
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
                                            <h2>{"You're Almost There!"}</h2>
                                            <h4>{"Finish Your Registration Below..."}</h4>

                                                           <form>
                                                                <fieldset class="uk-fieldset",>

                                                                    <input class="uk-input uk-margin",
                                                                        placeholder={"Email"},
                                                                        disabled=self.inputs_disabled,
                                                                        value=&self.email,
                                                                        oninput=self.link.callback(|e:InputData| Message::UpdateEmail(e.value)),
                                                                        />

                                                                    <input class="uk-input uk-margin",
                                                                        placeholder={"Username"},
                                                                        disabled=self.inputs_disabled,
                                                                        value=&self.username,
                                                                        oninput=self.link.callback(|e:InputData| Message::UpdateUsername(e.value)),
                                                                        />

                                                                    <input class="uk-input uk-margin-bottom",
                                                                        type="password",
                                                                        placeholder={"Password"},
                                                                        disabled=self.inputs_disabled,
                                                                        value=&self.password,
                                                                        oninput=self.link.callback(|e:InputData| Message::UpdatePassword(e.value)),
                                                                        />

                                                                </fieldset>
                                                            </form>

                                                                    <button class="uk-button uk-button-primary",
                                                                        type="submit",
                                                                        disabled=self.register_button_disabled,
                                                                        onclick=self.link.callback(move |_| Message::RegisterRequest),
                                                                        >{"Register!"}</button>
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
