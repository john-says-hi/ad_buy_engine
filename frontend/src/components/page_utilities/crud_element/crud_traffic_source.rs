use super::dropdowns::{
    currency_dropdown::CurrencyDropdown, tracking_domain_dropdown::TrackingDomainDropdown,
    tracking_method_dropdown::TrackingMethodDropdown,
};
use crate::agents::tick_tock::{TickTock, TickTockRequest};
use crate::appstate::app_state::AppState;
use crate::components::account_tab_section::custom_conversions::modal::ModalType;
use crate::components::page_utilities::crud_element::dropdowns::referrer_handling_dropdown::ReferrerHandlingDropdown;
use crate::components::page_utilities::crud_element::notes::NotesComponent;
use crate::components::page_utilities::crud_element::select_custom_event::SelectCustomConversionEvents;
use crate::components::page_utilities::crud_element::whitelist_postback_ips::WhitelistPostbackIPsComponent;
use crate::components::primitives::text_area::TextArea;
use crate::components::primitives::TextInput;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::{hide_uk_modal, toggle_uk_dropdown};
use crate::utils::routes::AppRoute;
use crate::{notify_primary, RootComponent, notify_danger};
use ad_buy_engine::constant::apis::private::API_CRUD_ELEMENT;
use ad_buy_engine::constant::browser_storage_keys::OFFER_SOURCES;
use ad_buy_engine::data::account::domains_configuration::CustomDomainName;
use ad_buy_engine::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs, ConversionCapConfig, PayoutType};
use ad_buy_engine::data::custom_events::{CustomConversionEvent, CustomConversionEventToken};
use ad_buy_engine::data::elements::crud::{
    CRUDElementRequest, CRUDElementResponse, CreatableElement, PrimeElementBuild,
};
use ad_buy_engine::data::elements::offer_source::{ OfferSource};
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::{ExternalIDParameter, CustomParameter, CostParameter};
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
use ad_buy_engine::data::lists::{Currency, Language, Vertical, DataURLToken};
use ad_buy_engine::data::work_space::Clearance;
use ad_buy_engine::{AError, Country};
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
use ad_buy_engine::data::elements::offer::Offer;
use rust_decimal::Decimal;
use crate::components::page_utilities::crud_element::complex_sub_component::offer_url_token_selector::OfferURLTokenSelector;
use crate::components::page_utilities::crud_element::dropdowns::offer_source_dropdown::OfferSourceDropdown;
use crate::components::page_utilities::crud_element::dropdowns::country_dropdown::CountryDropdown;
use crate::components::page_utilities::crud_element::complex_sub_component::tags_selector::TagsSelector;
use crate::components::page_utilities::crud_element::complex_sub_component::offer_url_generator::OfferUrlGenerator;
use crate::components::page_utilities::crud_element::complex_sub_component::payout_type_handler::PayoutTypeHandler;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use crate::components::page_utilities::crud_element::complex_sub_component::lander_url_generator::LanderUrlGenerator;
use crate::components::page_utilities::crud_element::complex_sub_component::lander_url_token_selector::LanderURLTokenSelector;
use crate::components::page_utilities::crud_element::complex_sub_component::landing_page_click_url_generator::LandingPageClickURLGenerator;
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use crate::components::page_utilities::crud_element::complex_sub_component::traffic_source_postback_url_generator::TrafficSourcePostbackUrlGenerator;
use crate::components::page_utilities::crud_element::complex_sub_component::traffic_source_postback_url_token_selector::TrafficSourcePostbackURLTokenSelector;
use crate::components::page_utilities::crud_element::complex_sub_component::traffic_source_url_parameter_configuration::TrafficSourceUrlParameterConfig;
use crate::components::page_utilities::crud_element::toggle_switch::ToggleSwitch;

pub enum Msg {
    Tick,
    Click,
    UpdateName(InputData),
    UpdateNotes(InputData),
    FetchData(CRUDElementResponse),
    FetchFailed,
    DeserializationFailed,
    UpdateDataURLTokens(Vec<DataURLToken>),
    UpdateExternalID(ExternalIDParameter),
    UpdateCost(CostParameter),
    UpdateCustom(Vec<CustomParameter>),
    UpdateCurrency(Currency),
    ToggleTrafficSourcePostbackURL,
    UpdateUrl(Url),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    #[prop_or_default]
    pub restored_element: Option<TrafficSource>,
    pub modal_type: ModalType,
}

pub struct CRUDTrafficSource {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub name: String,

    pub external_id_token_data: ExternalIDParameter,
    pub cost_token_data: CostParameter,
    pub custom_token_data: Vec<CustomParameter>,
    pub currency: Currency,
    pub traffic_source_postback_url: Option<Url>,
    pub traffic_source_postback_url_tokens: Vec<DataURLToken>,

    pub notes: String,
    pub fetch_task: Option<FetchTask>,
    pub tt: Box<dyn Bridge<TickTock>>,

    pub traffic_source_postback_url_is_active: bool,
    pub traffic_source_postback_url_for_each_event_is_active: bool,
    pub pixel_redirect_url_is_active: bool,
    pub impression_tracking_is_active: bool,
    pub direct_tracking_is_active: bool,
}

impl Component for CRUDTrafficSource {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tracking_domain = props
            .state
            .borrow()
            .account
            .borrow()
            .domains_configuration
            .main_domain
            .clone();
        let tt = TickTock::bridge(link.callback(|_| Msg::Tick));

        Self {
            link,
            props,
            name: "".to_string(),
            external_id_token_data: ExternalIDParameter {
                parameter: "".to_string(),
                placeholder: "".to_string(),
            },
            cost_token_data: CostParameter {
                parameter: "".to_string(),
                placeholder: "".to_string(),
            },
            custom_token_data: vec![],
            currency: Currency::USD,
            traffic_source_postback_url: None,
            traffic_source_postback_url_tokens: vec![],
            notes: "".to_string(),
            fetch_task: None,
            tt,
            traffic_source_postback_url_is_active: false,
            traffic_source_postback_url_for_each_event_is_active: false,
            pixel_redirect_url_is_active: false,
            impression_tracking_is_active: false,
            direct_tracking_is_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateExternalID(external_id) => {
                self.external_id_token_data = external_id;
            }
            Msg::UpdateCost(cost) => {
                self.cost_token_data = cost;
            }
            Msg::UpdateCustom(custom) => {
                self.custom_token_data = custom;
            }
            Msg::UpdateUrl(url) => self.traffic_source_postback_url = Some(url),
            Msg::UpdateCurrency(currency) => self.currency = currency,
            Msg::ToggleTrafficSourcePostbackURL => {
                self.traffic_source_postback_url_is_active =
                    !self.traffic_source_postback_url_is_active
            }
            Msg::UpdateDataURLTokens(list) => self.traffic_source_postback_url_tokens = list,
            Msg::Tick => {}
            Msg::Click => self.fetch_task = self.fetch(),
            Msg::UpdateName(i) => self.name = i.value,
            Msg::UpdateNotes(i) => self.notes = i.value,
            Msg::FetchData(response) => {
                self.fetch_task = None;
                self.props.state.borrow().crud_update(response);
                hide_uk_modal("#traffic-sources");
                self.tt.send(TickTockRequest::Tick);
            }
            Msg::FetchFailed => {
                self.fetch_task = None;
                notify_primary("Fetch Failed");
                hide_uk_modal("#traffic-sources");
            }
            Msg::DeserializationFailed => {
                self.fetch_task = None;
                notify_primary("Deserialization Failed");
                hide_uk_modal("#traffic-sources");
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(restored_element) = &props.restored_element {
            self.name = restored_element.name.clone();
            self.notes = restored_element.notes.clone();
            self.external_id_token_data = restored_element.external_id_token_data.clone();
            self.cost_token_data = restored_element.cost_token_data.clone();
            self.custom_token_data = restored_element.custom_token_data.clone();
            self.currency = restored_element.currency;
            self.traffic_source_postback_url = restored_element.traffic_source_postback_url.clone();
        } else {
            self.name = "".to_string();
            self.notes = "".to_string();

            self.external_id_token_data = ExternalIDParameter {
                parameter: "".to_string(),
                placeholder: "".to_string(),
            };
            self.cost_token_data = CostParameter {
                parameter: "".to_string(),
                placeholder: "".to_string(),
            };
            self.custom_token_data = vec![];
            self.currency = Currency::USD;
            self.traffic_source_postback_url = None;
            self.traffic_source_postback_url_tokens = vec![];
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let modal_title = if let ModalType::Update = self.props.modal_type {
            "Update Traffic Source"
        } else {
            "New Traffic Source"
        };

        html! {
        <div id="traffic-sources" class="uk-flex-top" uk-modal="bg-close:false;">
           <div class="uk-modal-dialog uk-margin-auto-vertical">
              <button class="uk-modal-close-default" type="button" uk-close=""></button>
              <div class="uk-modal-header">
                 <h2 class="uk-modal-title uk-text-center">{modal_title}</h2>
              </div>
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">

                        <TextInput label="Traffic Source Name:" value=&self.name placeholder="Name" oninput=self.link.callback(Msg::UpdateName) />

                        <TrafficSourceUrlParameterConfig external_id=&self.external_id_token_data cost=&self.cost_token_data custom=&self.custom_token_data eject_external_id=self.link.callback(Msg::UpdateExternalID) eject_cost=self.link.callback(Msg::UpdateCost) eject_custom=self.link.callback(Msg::UpdateCustom) />

                        <CurrencyDropdown callback=self.link.callback(Msg::UpdateCurrency) label="Cost Currency".to_string() />

                        <div class="uk-margin">
                           <ToggleSwitch label="Traffic Source Postback URL" checked=self.traffic_source_postback_url_is_active onchange=self.link.callback(|_|Msg::ToggleTrafficSourcePostbackURL)  />
                        </div>

                        {if self.traffic_source_postback_url_is_active {html!{
                        <>
                        <TrafficSourcePostbackUrlGenerator tokens=&self.traffic_source_postback_url_tokens url=&self.traffic_source_postback_url eject=self.link.callback(Msg::UpdateUrl) />

                        <TrafficSourcePostbackURLTokenSelector selected=&self.traffic_source_postback_url_tokens eject=self.link.callback(Msg::UpdateDataURLTokens) />
                        </>
                        }} else {html!{

                        }}}

                        <div class="uk-margin">
                            {label!("Notes")}
                           <NotesComponent callback=self.link.callback(Msg::UpdateNotes) value=&self.notes />
                        </div>

                   </div>

                 <div class="uk-modal-footer uk-text-right">
                    <button class="uk-button uk-button-default uk-modal-close" type="button">{"Cancel"}</button>
                    <button onclick=self.link.callback(|_|Msg::Click) class="uk-button uk-button-primary" type="button">{"Save"}</button>
                 </div>
              </div>
           </div>
        </div>


                                            }
    }
}

impl CRUDTrafficSource {
    pub fn fetch(&self) -> Option<FetchTask> {
        let data = if let ModalType::Create = self.props.modal_type {
            CRUDElementRequest::Create(PrimeElementBuild::TrafficSource(TrafficSource {
                traffic_source_id: Uuid::new_v4(),
                account_id: self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .account_id
                    .clone(),
                name: self.name.clone(),
                clearance: Clearance::Everyone,
                external_id_token_data: self.external_id_token_data.clone(),
                cost_token_data: self.cost_token_data.clone(),
                custom_token_data: self.custom_token_data.clone(),
                currency: self.currency,
                traffic_source_postback_url: self.traffic_source_postback_url.clone(),
                traffic_source_postback_url_on_custom_event: vec![],
                pixel_redirect_url: None,
                track_impressions: false,
                direct_tracking: false,
                notes: self.notes.clone(),
                archived: false,
                last_updated: Utc::now(),
            }))
        } else {
            let restored_element = self
                .props
                .restored_element
                .clone()
                .expect("Failed to unwrap restored element");

            CRUDElementRequest::Update(vec![PrimeElementBuild::TrafficSource(TrafficSource {
                traffic_source_id: restored_element.traffic_source_id,
                account_id: self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .account_id
                    .clone(),
                name: self.name.clone(),
                clearance: Clearance::Everyone,
                external_id_token_data: self.external_id_token_data.clone(),
                cost_token_data: self.cost_token_data.clone(),
                custom_token_data: self.custom_token_data.clone(),
                currency: self.currency,
                traffic_source_postback_url: self.traffic_source_postback_url.clone(),
                traffic_source_postback_url_on_custom_event: vec![],
                pixel_redirect_url: None,
                track_impressions: false,
                direct_tracking: false,
                notes: self.notes.clone(),
                archived: false,
                last_updated: Utc::now(),
            })])
        };

        let request = Request::post(API_CRUD_ELEMENT)
            .header("Content-Type", "application/json")
            .body(Json(&data))
            .unwrap();
        let callback = self.link.callback(
            move |response: Response<Json<Result<CRUDElementResponse, AError>>>| {
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
            },
        );

        let fetch_task = FetchService::fetch(request, callback).expect("f43ss");
        Some(fetch_task)
    }
}
