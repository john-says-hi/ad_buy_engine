use crate::notify_danger;
use crate::utils::javascript::js_bindings::redirect_login;
use ad_buy_engine::constant::apis::private::API_URL_LOGOUT;
use ad_buy_engine::AError;
use yew::format::Nothing;
use yew::prelude::*;
use yew_services::fetch::{FetchTask, Request, Response};
use yew_services::FetchService;

pub enum Msg {
    Click,
    LogoutSuccess,
    LogoutFailed,
}

pub struct Logout {
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

impl Component for Logout {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, task: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LogoutFailed => {
                self.task = None;
                notify_danger("Logout Failed");
                redirect_login();
            }
            Msg::LogoutSuccess => {
                self.task = None;
                redirect_login();
            }
            Msg::Click => {
                let request = Request::delete(API_URL_LOGOUT)
                    .body(Nothing)
                    .expect("Fasdw@");
                let callback = self.link.callback(|res: Response<Result<String, AError>>| {
                    if res.status().is_success() {
                        Msg::LogoutSuccess
                    } else {
                        Msg::LogoutFailed
                    }
                });
                let task = FetchService::fetch(request, callback).expect("#$5v43");
                self.task = Some(task);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cb = self.link.callback(|_| Msg::Click);
        html! {
            <li class="uk-navbar-item">
                    <div uk-tooltip="title: Logout">
                        <span class="uk-icon uk-margin" uk-icon="icon: sign-out" onclick=cb></span>
                    </div>
            </li>
        }
    }
}
