use crate::agents::tick_tock::{TickTock, TickTockRequest};
use crate::appstate::app_state::AppState;
use crate::components::page_utilities::crud_element::notes::NotesComponent;
use crate::components::page_utilities::crud_element::whitelist_postback_ips::WhitelistPostbackIPsComponent;
use crate::components::primitives::text_area::TextArea;
use crate::components::primitives::TextInput;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::{hide_uk_modal, toggle_uk_dropdown};
use crate::utils::routes::AppRoute;
use crate::{notify_danger, notify_primary, notify_warning, RootComponent};
use ad_buy_engine::constant::apis::private::API_CRUD_ELEMENT;
use ad_buy_engine::constant::browser_storage_keys::OFFER_SOURCES;
use ad_buy_engine::data::account::domains_configuration::CustomDomainName;
use ad_buy_engine::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use ad_buy_engine::data::custom_events::CustomConversionEvent;
use ad_buy_engine::data::elements::crud::{
    CRUDElementRequest, CRUDElementResponse, CreatableElement, PrimeElementBuild,
};
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::lists::referrer_handling::{ReferrerHandling, ReplaceReferrerList};
use ad_buy_engine::data::lists::Currency;
use ad_buy_engine::data::work_space::Clearance;
use ad_buy_engine::AError;
use chrono::Utc;
use std::cell::RefCell;
use std::net::IpAddr;
use std::rc::Rc;
use strum::IntoEnumIterator;
use url::Url;
use uuid::Uuid;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VNode;

use yew_services::fetch::{FetchTask, Request, Response};
use yew_services::storage::Area;
use yew_services::{FetchService, StorageService};

pub enum Msg {
    Submit,
    Ignore,
    Tick,
    UpdateName(InputData),
    UpdateReferrerList(InputData),
    UpdatePercent(InputData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    pub onsubmit: Callback<ReplaceReferrerList>,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub percent: u8,
    #[prop_or_default]
    pub referrer_list: String,
}

pub struct ReferrerHandlingModal {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub name: String,
    pub percent: u8,
    pub referrer_list: String,
}

impl Component for ReferrerHandlingModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: props.name.clone(),
            percent: 100,
            referrer_list: props.referrer_list.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(i) => self.name = i.value,
            Msg::UpdateReferrerList(i) => self.referrer_list = i.value,
            Msg::Submit => {
                if let Some(pos) = self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .referrer_handling_list
                    .iter()
                    .position(|s| s.name_of_list == self.name)
                {
                    notify_danger("List name already set! Please change the list's name.")
                } else {
                    self.props.onsubmit.emit(ReplaceReferrerList {
                        percent_of_originals_to_replace: self.percent,
                        name_of_list: self.name.clone(),
                        referrer_list_items: self.referrer_list.clone(),
                    })
                }
            }
            Msg::Ignore => {}
            Msg::Tick => {}
            Msg::UpdatePercent(i) => {
                if let Ok(percent) = i.value.parse::<u8>() {
                    self.percent = percent;
                } else {
                    notify_warning("Please enter a number between 1-100")
                }
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.name = props.name.clone();
        self.referrer_list = props.referrer_list.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <div id="referrer-handling" class="uk-flex-top" uk-modal="bg-close:false;">
           <div class="uk-modal-dialog uk-margin-auto-vertical">
              <button class="uk-modal-close-default" type="button" uk-close=""></button>
              <div class="uk-modal-header">
                 <h2 class="uk-modal-title uk-text-center">{"Referrer List"}</h2>
              </div>
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">
                        <TextInput label="Name of List:" value=&self.name placeholder="Name" oninput=self.link.callback(Msg::UpdateName) />
                        <TextInput label="(Optional) Percent of Referrers to Replace" value=&self.percent.to_string() oninput=self.link.callback(Msg::UpdatePercent) />
                        <TextArea rows="24" label="Referrers to obfuscate with; (i.e. \"https://friends.com/profile/\")" value=&self.referrer_list oninput=self.link.callback(Msg::UpdateReferrerList) />

                   </div>

                 <div class="uk-modal-footer uk-text-right">
                    <button class="uk-button uk-button-default uk-modal-close" type="button">{"Cancel"}</button>
                    <button onclick=self.link.callback(|_|Msg::Submit) class="uk-button uk-button-primary" type="button">{"Save"}</button>
                 </div>
              </div>
           </div>
        </div>


                                            }
    }
}
