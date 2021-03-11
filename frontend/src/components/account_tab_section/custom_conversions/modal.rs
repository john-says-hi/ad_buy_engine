use crate::agents::tick_tock::{TickTock, TickTockRequest};
use crate::appstate::app_state::AppState;
use crate::components::page_utilities::crud_element::notes::NotesComponent;
use crate::components::page_utilities::crud_element::whitelist_postback_ips::WhitelistPostbackIPsComponent;
use crate::components::primitives::text_area::TextArea;
use crate::components::primitives::TextInput;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::{hide_uk_modal, toggle_uk_dropdown};
use crate::utils::routes::AppRoute;
use crate::{notify_danger, notify_primary, RootComponent};
use ad_buy_engine::constant::apis::private::API_CRUD_ELEMENT;
use ad_buy_engine::constant::browser_storage_keys::OFFER_SOURCES;
use ad_buy_engine::data::account::domains_configuration::CustomDomainName;
use ad_buy_engine::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use ad_buy_engine::data::custom_events::CustomConversionEvent;
use ad_buy_engine::data::elements::crud::{
    CRUDElementRequest, CRUDElementResponse, CreatableElement, PrimeElementBuild,
};
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
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
use yew_material::{MatSwitch, MatTextArea, MatTextField};
use yew_services::fetch::{FetchTask, Request, Response};
use yew_services::storage::Area;
use yew_services::{FetchService, StorageService};

pub enum Msg {
    Submit,
    Ignore,
    Tick,
    UpdateName(InputData),
    UpdateParameter(InputData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    pub onsubmit: Callback<CustomConversionEvent>,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub parameter: String,
    pub modal_type: ModalType,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum ModalType {
    Create,
    Update,
}

pub struct CustomConversionModal {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub name: String,
    pub parameter: String,
}

impl Component for CustomConversionModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: props.name.clone(),
            parameter: props.parameter.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(i) => self.name = i.value,
            Msg::UpdateParameter(i) => self.parameter = i.value,
            Msg::Submit => {
                if let Some(pos) = self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .custom_conversions
                    .iter()
                    .position(|s| s.parameter == self.parameter)
                {
                    notify_danger("Parameter Already Set! Please change paramater.")
                } else {
                    self.props.onsubmit.emit(CustomConversionEvent {
                        include_in_conversions_column: false,
                        include_in_revenue_column: false,
                        send_postback_to_traffic_source: false,
                        include_in_cost_column: false,
                        name: self.name.clone(),
                        parameter: self.parameter.clone(),
                    })
                }
            }
            Msg::Ignore => {}
            Msg::Tick => {}
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.name = props.name.clone();
        self.parameter = props.parameter.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let title = if let ModalType::Update = self.props.modal_type {
            "Edit Custom Event"
        } else {
            "New Custom Event"
        };
        html! {


        <div id="custom-conversion-event" class="uk-flex-top" uk-modal="bg-close:false;">
           <div class="uk-modal-dialog uk-margin-auto-vertical">
              <button class="uk-modal-close-default" type="button" uk-close=""></button>
              <div class="uk-modal-header">
                 <h2 class="uk-modal-title uk-text-center">{title}</h2>
              </div>
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">

                        <TextInput label="Event Name:" value=&self.name placeholder="Name" oninput=self.link.callback(Msg::UpdateName) />
                        <TextArea rows="4" label="URL Parameter; (i.e. \"email_submit\")" value=&self.parameter oninput=self.link.callback(Msg::UpdateParameter) />

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
