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
use crate::{notify_danger, notify_primary, RootComponent};
use ad_buy_engine::constant::apis::private::API_CRUD_ELEMENT;
use ad_buy_engine::constant::browser_storage_keys::OFFER_SOURCES;
use ad_buy_engine::data::account::domains_configuration::CustomDomainName;
use ad_buy_engine::data::conversion::{
    ConversionCapConfig, ConversionTrackingMethod, PayoutType, WhiteListedPostbackIPs,
};
use ad_buy_engine::data::custom_events::{CustomConversionEvent, CustomConversionEventToken};
use ad_buy_engine::data::elements::crud::{
    CRUDElementRequest, CRUDElementResponse, CreatableElement, PrimeElementBuild,
};
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
use ad_buy_engine::data::lists::{Currency, DataURLToken, Language, Vertical};
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

pub enum Msg {
    Ignore,
    Tick,
    Click,
    UpdateName(InputData),
    UpdateNotes(InputData),
    FetchData(CRUDElementResponse),
    FetchFailed,
    DeserializationFailed,
    UpdateDataURLTokens(Vec<DataURLToken>),
    UpdateCountry(Country),
    UpdateTags(Vec<String>),
    UpdateUrl(Url),
    UpdateTrackingDomain(Url),
    UpdateNumCTA(InputData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    #[prop_or_default]
    pub restored_element: Option<LandingPage>,
    pub modal_type: ModalType,
}

pub struct CRUDLandingPage {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub name: String,
    pub country: Country,
    pub tags: Vec<String>,
    pub landing_page_url: Option<Url>,
    pub landing_page_url_tokens: Vec<DataURLToken>,
    pub number_of_calls_to_action: String,
    pub tracking_domain: Url,
    pub notes: String,
    pub fetch_task: Option<FetchTask>,
    pub tt: Box<dyn Bridge<TickTock>>,
}

impl Component for CRUDLandingPage {
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
            country: Country::Global,
            tags: vec![],
            landing_page_url: None,
            landing_page_url_tokens: vec![],
            number_of_calls_to_action: 1.to_string(),
            tracking_domain,
            notes: "".to_string(),
            fetch_task: None,
            tt,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTrackingDomain(url) => {
                self.tracking_domain = url;
            }
            Msg::UpdateNumCTA(i) => {
                if !i.value.is_empty() {
                    if let Ok(num) = i.value.parse::<u8>() {
                        self.number_of_calls_to_action = num.to_string();
                    } else {
                        notify_danger("Number of CTAs must be between 1-255")
                    }
                } else {
                    self.number_of_calls_to_action = i.value;
                }
            }

            Msg::UpdateUrl(url) => self.landing_page_url = Some(url),
            Msg::UpdateTags(tags) => self.tags = tags,
            Msg::UpdateCountry(country) => self.country = country,
            Msg::UpdateDataURLTokens(list) => self.landing_page_url_tokens = list,
            Msg::Ignore => {}
            Msg::Tick => {}
            Msg::Click => {
                if let Some(ur) = &self.landing_page_url {
                    self.fetch_task = self.fetch()
                } else {
                    notify_danger("Please enter a valid Offer URL.")
                }
            }
            Msg::UpdateName(i) => self.name = i.value,
            Msg::UpdateNotes(i) => self.notes = i.value,
            Msg::FetchData(response) => {
                self.fetch_task = None;
                self.props.state.borrow().crud_update(response);
                hide_uk_modal("#landing-pages");
                self.tt.send(TickTockRequest::Tick);
            }
            Msg::FetchFailed => {
                self.fetch_task = None;
                notify_primary("Fetch Failed");
                hide_uk_modal("#landing-pages");
            }
            Msg::DeserializationFailed => {
                self.fetch_task = None;
                notify_primary("Deserialization Failed");
                hide_uk_modal("#landing-pages");
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(restored_element) = &props.restored_element {
            self.name = restored_element.name.clone();
            self.notes = restored_element.notes.clone();
            self.tags = restored_element.tags.clone();
            self.landing_page_url = Some(restored_element.url.clone());
            self.landing_page_url_tokens = restored_element.url_tokens.clone();
            self.number_of_calls_to_action = restored_element.number_of_calls_to_action.to_string();
        } else {
            self.name = "".to_string();
            self.notes = "".to_string();
            self.tags = vec![];
            self.landing_page_url = None;
            self.landing_page_url_tokens = vec![];
            self.number_of_calls_to_action = 1.to_string();
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let modal_title = if let ModalType::Update = self.props.modal_type {
            "Update Lander"
        } else {
            "New Lander"
        };
        let ctas = if let Ok(num) = self.number_of_calls_to_action.parse::<u8>() {
            num
        } else {
            1
        };

        html! {
        <div id="landing-pages" class="uk-flex-top" uk-modal="bg-close:false;">
           <div class="uk-modal-dialog uk-margin-auto-vertical">
              <button class="uk-modal-close-default" type="button" uk-close=""></button>
              <div class="uk-modal-header">
                 <h2 class="uk-modal-title uk-text-center">{modal_title}</h2>
              </div>
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">

                        <CountryDropdown selected=&self.country label="Country " eject=self.link.callback(Msg::UpdateCountry) />

                        <TextInput label="Lander Name:" value=&self.name placeholder="Name" oninput=self.link.callback(Msg::UpdateName) />

                        <TagsSelector tags=&self.tags eject=self.link.callback(Msg::UpdateTags) />

                        <LanderUrlGenerator url_tokens=&self.landing_page_url_tokens offer_url=&self.landing_page_url eject=self.link.callback(Msg::UpdateUrl) />

                        <LanderURLTokenSelector selected=&self.landing_page_url_tokens eject=self.link.callback(Msg::UpdateDataURLTokens) />

                        <TextInput label="Number of CTAs:" value=self.number_of_calls_to_action.to_string() placeholder="1" oninput=self.link.callback(Msg::UpdateNumCTA) />

                        <TrackingDomainDropdown state=Rc::clone(&self.props.state) callback=self.link.callback(Msg::UpdateTrackingDomain) />

                        <LandingPageClickURLGenerator number_of_ctas=ctas tracking_domain=&self.tracking_domain  />

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

impl CRUDLandingPage {
    pub fn fetch(&self) -> Option<FetchTask> {
        if let Ok(num) = self.number_of_calls_to_action.parse::<u8>() {
            let data = if let ModalType::Create = self.props.modal_type {
                CRUDElementRequest::Create(PrimeElementBuild::LandingPage(LandingPage {
                    landing_page_id: Uuid::new_v4(),
                    account_id: self
                        .props
                        .state
                        .borrow()
                        .account
                        .borrow()
                        .account_id
                        .clone(),
                    name: self.name.clone(),
                    tags: self.tags.clone(),
                    url: self.landing_page_url.clone().expect("g543srt"),
                    url_tokens: self.landing_page_url_tokens.clone(),
                    language: Language::Any,
                    clearance: Clearance::Everyone,
                    notes: self.notes.clone(),
                    weight: 100,
                    archived: false,
                    last_updated: Utc::now(),
                    country: self.country.clone(),
                    vertical: Vertical::Many,
                    number_of_calls_to_action: num,
                }))
            } else {
                let restored_element = self
                    .props
                    .restored_element
                    .clone()
                    .expect("Failed to unwrap restored element");

                CRUDElementRequest::Update(vec![PrimeElementBuild::LandingPage(LandingPage {
                    landing_page_id: restored_element.landing_page_id,
                    account_id: self
                        .props
                        .state
                        .borrow()
                        .account
                        .borrow()
                        .account_id
                        .clone(),
                    name: self.name.clone(),
                    tags: self.tags.clone(),
                    url: self.landing_page_url.clone().expect("G45d"),
                    url_tokens: self.landing_page_url_tokens.clone(),
                    language: Language::Any,
                    clearance: Clearance::Everyone,
                    notes: self.notes.clone(),
                    weight: 100,
                    archived: false,
                    last_updated: Utc::now(),
                    country: self.country.clone(),
                    vertical: Vertical::Many,
                    number_of_calls_to_action: num,
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
        } else {
            notify_danger("Invalid num of ctas");
            None
        }
    }
}
