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
mod modal;
use crate::components::account_tab_section::referrer_handling::modal::ReferrerHandlingModal;
use ad_buy_engine::data::lists::referrer_handling::{ReferrerHandling, ReplaceReferrerList};
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
    Post(ReplaceReferrerList),
    Edit(usize),
    CreateToggle,
    // Click,
    Ignore,
    FetchData(Account),
    FetchFailed,
    DeserializationFailed,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct ReferrerHandlingAccountSection {
    link: ComponentLink<Self>,
    props: Props,
    fetch_task: Option<FetchTask>,
    pub name: String,
    pub percent: u8,
    pub referrer_list: String,
    pub edit: bool,
    pub selected_list_pos: usize,
}

impl Component for ReferrerHandlingAccountSection {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            fetch_task: None,
            name: "".to_string(),
            percent: 0,
            referrer_list: "".to_string(),
            edit: false,
            selected_list_pos: 0,
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
                hide_uk_modal("#referrer-handling");
            }

            Msg::CreateToggle => {
                self.edit = false;
                show_uk_modal("#referrer-handling")
            }
            Msg::Post(event) => {
                let state = self.props.state.borrow();
                let mut account = state.account.borrow_mut();

                if self.edit {
                    account
                        .referrer_handling_list
                        .remove(self.selected_list_pos);
                    account.referrer_handling_list.push(event);
                } else {
                    account.referrer_handling_list.push(event);
                }

                let mut account_data = account.clone();

                self.referrer_list.clear();
                self.name.clear();

                self.fetch_task = self.fetch(account_data);
            }

            Msg::Edit(pos) => {
                self.edit = true;
                if let Some(item) = self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .referrer_handling_list
                    .get(pos)
                {
                    self.selected_list_pos = pos;
                    self.name = item.name_of_list.clone();
                    self.referrer_list = item.referrer_list_items.clone();
                    self.percent = item.percent_of_originals_to_replace;
                    show_uk_modal("#referrer-handling")
                } else {
                    notify_warning("Internal Err: Could not find item in state")
                }
            }
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
            .referrer_handling_list
            .is_empty()
        {
            nodes.push(html! {<h5>{"No Lists Created"}</h5>})
        }

        for (idx, item) in self
            .props
            .state
            .borrow()
            .account
            .borrow()
            .referrer_handling_list
            .iter()
            .enumerate()
        {
            nodes.push(html! {
                    <tr>
                        <td>{item.name_of_list.clone()}</td>
                        <td>{item.percent_of_originals_to_replace.to_string()}</td>
                        <td><button class="uk-button uk-button-small uk-background-primary uk-light" onclick=self.link.callback(move |_| Msg::Edit(idx))>{"Edit"}</button></td>
                    </tr>
            })
        }

        html! {
        <div class="uk-section">

            <div class="uk-container">

                <h3>{"Referrer Replace Lists"}</h3>
                <button uk-toggle="" href="#referrer-handling" onclick=self.link.callback(|_| Msg::CreateToggle) class="uk-button uk-button-default uk-background-primary uk-light"><span class="fas fa-plus uk-margin-small-right"></span>{"Create List"}</button>

                <table class="uk-table uk-table-small uk-table-divider">
                    <thead>
                        <tr>
                            <th>{"Name"}</th>
                            <th>{"Percent to Replace"}</th>
                            <th>{"Edit"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {nodes}
                    </tbody>
                </table>

            </div>
            <ReferrerHandlingModal state=Rc::clone(&self.props.state) name=&self.name percent=self.percent referrer_list=&self.referrer_list  onsubmit=self.link.callback(Msg::Post) />
        </div>
                }
    }
}

impl ReferrerHandlingAccountSection {
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
