pub mod modal;

use crate::appstate::app_state::AppState;
use crate::components::account_tab_section::custom_conversions::modal::ModalType;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::{hide_uk_modal, show_uk_modal};
use crate::utils::routes::AppRoute;
use crate::{notify_primary, notify_warning};
use ad_buy_engine::constant::apis::private::API_POST_ACCOUNT;
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::custom_events::CustomConversionEvent;
use ad_buy_engine::data::elements::crud::CRUDElementRequest;
use ad_buy_engine::AError;
use chrono::Utc;
use modal::CustomConversionModal;
use std::cell::RefCell;
use std::rc::Rc;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_material::list::GraphicType;
use yew_material::{MatListItem, MatMenu, MatSelect, MatTab, MatTabBar};
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_services::fetch::{FetchTask, Request, Response};
use yew_services::FetchService;

pub enum Msg {
    Post(CustomConversionEvent),
    Edit(usize),
    CreateToggle,
    Click,
    Ignore,
    FetchData(Account),
    FetchFailed,
    DeserializationFailed,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct CustomConversionSection {
    link: ComponentLink<Self>,
    props: Props,
    modal_type: ModalType,
    name: String,
    parameter: String,
    fetch_task: Option<FetchTask>,
    update_event_pos: usize,
}

impl Component for CustomConversionSection {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            modal_type: ModalType::Create,
            name: "".to_string(),
            parameter: "".to_string(),
            fetch_task: None,
            update_event_pos: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchFailed => {
                self.fetch_task = None;
                notify_warning("Fetch Failed")
            }
            Msg::DeserializationFailed => {
                self.fetch_task = None;
                notify_warning("Fetch Deserialization Failed")
            }
            Msg::FetchData(account) => {
                self.fetch_task = None;
                *self.props.state.borrow().account.borrow_mut() = account;
                self.props.state.borrow().store_account();
                hide_uk_modal("#custom-conversion-event");
            }
            Msg::CreateToggle => {
                self.modal_type = ModalType::Create;
                show_uk_modal("#custom-conversion-event")
            }
            Msg::Post(event) => {
                let state = self.props.state.borrow();
                let mut account = state.account.borrow_mut();

                if let ModalType::Create = self.modal_type {
                    account.custom_conversions.push(event);
                    let mut account_data = account.clone();
                    self.fetch_task = self.fetch(account_data);
                } else if let Some(item) = account.custom_conversions.get_mut(self.update_event_pos)
                {
                    item.name = event.name;
                    item.parameter = event.parameter;
                    let mut account_data = account.clone();
                    self.fetch_task = self.fetch(account_data);
                } else {
                    notify_warning("Internal Err: Could not find item in state");
                }
            }
            Msg::Edit(pos) => {
                if let Some(item) = self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .custom_conversions
                    .get(pos)
                {
                    self.update_event_pos = pos;
                    self.modal_type = ModalType::Update;
                    self.name = item.name.clone();
                    self.parameter = item.parameter.clone();
                    show_uk_modal("#custom-conversion-event")
                } else {
                    notify_warning("Internal Err: Could not find item in state")
                }
            }
            Msg::Click => {}
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut nodes = VList::new();

        if self
            .props
            .state
            .borrow()
            .account
            .borrow()
            .custom_conversions
            .is_empty()
        {
            nodes.push(html! {<h5>{"No Events Created"}</h5>})
        }

        for (idx, item) in self
            .props
            .state
            .borrow()
            .account
            .borrow()
            .custom_conversions
            .iter()
            .enumerate()
        {
            nodes.push(html! {
                    <tr>
                        <td>{item.name.clone()}</td>
                        <td>{item.parameter.clone()}</td>
                        <td><button class="uk-button uk-button-small uk-background-primary uk-light" onclick=self.link.callback(move |_| Msg::Edit(idx))>{"Edit"}</button></td>
                    </tr>
            })
        }

        html! {
        <div class="uk-section">

            <div class="uk-container">

                <h3>{"Custom Conversion Events"}</h3>
                <button uk-toggle="" href="#custom-conversion-event" onclick=self.link.callback(|_| Msg::Click) class="uk-button uk-button-default uk-background-primary uk-light"><span class="fas fa-plus uk-margin-small-right"></span>{"Create Event"}</button>

                <table class="uk-table uk-table-small uk-table-divider">
                    <thead>
                        <tr>
                            <th>{"Name"}</th>
                            <th>{"Parameter"}</th>
                            <th>{"Edit"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {nodes}
                    </tbody>
                </table>

            </div>
            {self.render_modal()}
        </div>
                }
    }
}

impl CustomConversionSection {
    pub fn render_modal(&self) -> VNode {
        let state = Rc::clone(&self.props.state);
        if let ModalType::Create = self.modal_type {
            html! {<CustomConversionModal state=state modal_type=self.modal_type onsubmit=self.link.callback(Msg::Post) />}
        } else {
            html! {<CustomConversionModal state=state name=&self.name parameter=&self.parameter modal_type=self.modal_type onsubmit=self.link.callback(Msg::Post) />}
        }
    }

    pub fn fetch(&self, mut account_data: Account) -> Option<FetchTask> {
        account_data.last_updated = Utc::now();

        let request = Request::post(API_POST_ACCOUNT)
            .header("Content-Type", "application/json")
            .body(Json(&account_data))
            .unwrap();

        let callback =
            self.link
                .callback(move |response: Response<Json<Result<Account, AError>>>| {
                    let (meta, Json(body)) = response.into_parts();

                    if meta.status.is_success() {
                        if let Ok(data) = body {
                            Msg::FetchData(data)
                        } else {
                            Msg::DeserializationFailed
                        }
                    } else {
                        Msg::FetchFailed
                    }
                });

        let fetch_task = FetchService::fetch(request, callback).expect("f43ss");
        Some(fetch_task)
    }
}
