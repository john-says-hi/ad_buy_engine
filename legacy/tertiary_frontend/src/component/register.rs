use crate::alert;
use crate::prelude::*;
use crate::routes::AppRoute;
use anyhow::Error;

use crate::javascript::js_bindings::login_redirect;
use ad_buy_engine::constant::apis::public::API_URL_CREATE_REGISTER;
use ad_buy_engine::CreateUserRequest;
use mailchecker::{blacklist, is_valid};
use yew::services::fetch::{Request, Response};
use yew::services::{DialogService, FetchService};
use yew::{
    format::Json, html, prelude::*, services::fetch, virtual_dom::VNode, Component, ComponentLink,
    Html, InputData, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*, Switch};

pub struct RegisterComponent {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<fetch::FetchTask>,
    inputs_disabled: bool,
    register_button_disabled: bool,
    email: String,
    username: String,
    company_name: String,
    password: String,
}

impl RegisterComponent {
    fn update_button_state(&mut self) {
        self.register_button_disabled = self.email.is_empty()
            || self.username.is_empty()
            || self.company_name.is_empty()
            || self.password.is_empty();
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
    UpdateTeamName(String),
    UpdatePassword(String),
}

impl Component for RegisterComponent {
    type Message = Message;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Message::Ignore);
        let router = RouteAgent::bridge(callback);

        let email = String::new();

        // let email = if CookieService::new().get(EMAIL_COOKIE_NAME).is_ok() {
        //     CookieService::new().get(EMAIL_COOKIE_NAME).ok().unwrap()
        // } else {
        //     String::new()
        // };
        // if !email.is_empty() {
        //     CookieService::new().remove(EMAIL_COOKIE_NAME)
        // }

        RegisterComponent {
            link,
            router,
            fetch_task: None,
            inputs_disabled: false,
            register_button_disabled: false,
            email,
            username: String::from("hiusernameok"),
            company_name: String::new(),
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
                } else if self.company_name.len() > 32 || self.company_name.len() < 4 {
                    alert("TEAM NAME MUST BE BETWEEN 4 AND 32 CHARS")
                } else if self.password.len() > 48 || self.password.len() < 12 {
                    alert("PASSWORD MUST BE OVER 12 CHARS AND UNDER 48")
                } else {
                    // VALID, SUBMIT
                    let data = CreateUserRequest {
                        company_name: self.company_name.clone(),
                        email: self.email.clone(),
                        password: self.password.clone(),
                    };

                    let request = Request::post(API_URL_CREATE_REGISTER)
                        .header("Content-Type", "application/json")
                        .body(Json(&data))
                        .unwrap();

                    let callback = self
                        .link
                        .callback(|res: Response<Result<_, anyhow::Error>>| {
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

            Message::UpdateTeamName(team) => {
                self.company_name = team;
                self.update_button_state();
            }

            Message::UpdatePassword(new_password) => {
                self.password = new_password;
                self.update_button_state();
            }

            Message::RegisterSuccess => {
                self.fetch_task = None;
                alert("Welcome to Ad Buy Engine!");
                login_redirect()
            }

            Message::LoginSuccess => {
                self.fetch_task = None;
            }

            Message::LoginFailed => {
                self.fetch_task = None;
                alert("Automatic Login Failed. Reason Unknown, Routing to Login Page");
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
                                                                        placeholder={"Team Name"},
                                                                        disabled=self.inputs_disabled,
                                                                        value=&self.company_name,
                                                                        oninput=self.link.callback(|e:InputData| Message::UpdateTeamName(e.value)),
                                                                        />

                                                                    <input class="uk-input uk-margin",
                                                                        placeholder={"Email"},
                                                                        disabled=self.inputs_disabled,
                                                                        value=&self.email,
                                                                        oninput=self.link.callback(|e:InputData| Message::UpdateEmail(e.value)),
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
