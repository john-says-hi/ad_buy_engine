use crate::alert;
use crate::javascript::js_bindings::send_to_secure;
use crate::routes::AppRoute;
use yew::format::Json;
use yew::services::fetch::{Credentials, FetchOptions, FetchTask, Redirect, Request, Response};
use yew::services::{DialogService, FetchService};
use yew::{
    html, prelude::*, virtual_dom::VNode, Component, ComponentLink, Html, InputData, Properties,
    ShouldRender,
};
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::service::RouteService;
use ad_buy_engine::constant::apis::public::API_URL_LOGIN;
use ad_buy_engine::LoginRequest;

pub struct LoginComponent {
    link: ComponentLink<Self>,
    counter: usize,
    router: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
    inputs_disabled: bool,
    register_button_disabled: bool,
    email: String,
    password: String,
}

impl LoginComponent {
    fn update_button_state(&mut self) {
        self.register_button_disabled = self.email.is_empty() || self.password.is_empty();
    }
}

pub enum Message {
    RegisterRoute,
    UpdateIdent(String),
    UpdatePassword(String),
    LoginRequest,
    Ignore,
    Failed,
    Success,
}

impl Component for LoginComponent {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Message::Ignore);
        let router = RouteAgent::bridge(callback);

        LoginComponent {
            link,
            counter: 0,
            router,
            fetch_task: None,
            inputs_disabled: false,
            register_button_disabled: false,
            email: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::RegisterRoute => {
                self.router.send(ChangeRoute(AppRoute::Invitation.into()));
            }

            Message::UpdateIdent(i) => {
                self.email = i;
                self.update_button_state();
            }
            Message::UpdatePassword(p) => {
                self.password = p;
                self.update_button_state();
            }
            Message::Success => {
                self.fetch_task = None;
                send_to_secure()
            }
            Message::Failed => {
                self.fetch_task = None;
                alert("Login Failed, Please Check Your Credentials and Try Again. Username and Email are Permitted");
            }
            Message::LoginRequest => {
                let data = LoginRequest {
                    email: self.email.to_string(),
                    password: self.password.clone(),
                };

                let request = Request::post(API_URL_LOGIN)
                    .header("Content-Type", "application/json")
                    .body(Json(&data))
                    .unwrap();

                let callback =
                    self.link
                        .callback(|res: Response<Json<Result<String, anyhow::Error>>>| {
                            if res.status().is_success() {
                                return Message::Success;
                            } else {
                                Message::Failed
                            }
                        });

                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
            }
            Message::Ignore => {
                self.fetch_task = None;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> VNode {
        let home_btn = VNode::from(html! {<a href="/">{"Home"}</a>});
        let reg_cb = self.link.callback(move |_| Message::RegisterRoute);
        let register_btn = VNode::from(html! {<a onclick=reg_cb>{"Request Invite"}</a>});

        html! {
            <div>
                        <div class="  uk-preserve-color ">
                            <div class="uk-container">
                                <div class="uk-child-width-expand uk-text-center" uk-grid="">
                                    <div class=""></div>
                                        <div class="uk-width-1-2 uk-section">
                                            <div class=" uk-card uk-card-default uk-card-body   uk-border-rounded",>
                                            <h2>{"Login"}</h2>
                                    <h4>{"Welcome Back"}</h4>
                                                   <form>
                                                        <fieldset class="uk-fieldset",>

                                                            <input class="uk-input uk-margin",
                                                                placeholder={"Email"},
                                                                disabled=self.inputs_disabled,
                                                                value=&self.email,
                                                                oninput=self.link.callback(|e:InputData| Message::UpdateIdent(e.value)),
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
                                                                onclick=self.link.callback(move |_| Message::LoginRequest),
                                                                >{"Go to Dashboard!"}</button>

                                                                <div>{home_btn}{" | "}{register_btn}</div>

                                            </div>
                                        </div>
                                    <div class=""></div>
                                </div>
                            </div>
                        </div>
            </div>
        }
    }
}
