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

pub enum Msg {
    Ignore,
    Tick,
    Click,
    UpdateName(InputData),
    UpdatePayoutCurrency(Currency),
    UpdateNotes(InputData),
    FetchData(CRUDElementResponse),
    FetchFailed,
    DeserializationFailed,
    UpdateOfferURLTokens(Vec<DataURLToken>),
    UpdateOfferSource(OfferSource),
    UpdateCountry(Country),
    UpdateTags(Vec<String>),
    UpdateUrl(Url),
    UpdatePayoutType(PayoutType),
    UpdatePayoutValue(Decimal),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    #[prop_or_default]
    pub restored_element: Option<Offer>,
    pub modal_type: ModalType,
}

pub struct CRUDOffer {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub name: String,
    pub offer_source: Option<OfferSource>,
    pub country: Country,
    pub tags: Vec<String>,
    pub offer_url_string: String,
    pub offer_url: Option<Url>,
    pub offer_url_tokens: Vec<DataURLToken>,
    pub payout_type: PayoutType,
    pub payout_value: Decimal,
    pub payout_currency: Currency,
    pub conversion_cap_config: Option<ConversionCapConfig>,
    pub notes: String,
    pub fetch_task: Option<FetchTask>,
    pub tt: Box<dyn Bridge<TickTock>>,
    pub conversion_cap_active: bool,
}

impl Component for CRUDOffer {
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
            offer_source: None,
            country: Country::Global,
            tags: vec![],
            offer_url_string: "".to_string(),
            offer_url: None,
            offer_url_tokens: vec![],
            payout_type: PayoutType::Auto,
            payout_value: Decimal::new(0, 0),
            payout_currency: Currency::USD,
            conversion_cap_config: None,
            notes: "".to_string(),
            fetch_task: None,
            tt,
            conversion_cap_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdatePayoutType(t) => self.payout_type = t,
            Msg::UpdatePayoutValue(num) => self.payout_value = num,
            Msg::UpdateUrl(url) => self.offer_url = Some(url),
            Msg::UpdateTags(tags) => self.tags = tags,
            Msg::UpdateCountry(country) => self.country = country,
            Msg::UpdateOfferSource(data) => self.offer_source = Some(data),
            Msg::UpdateOfferURLTokens(list) => self.offer_url_tokens = list,
            Msg::Ignore => {}
            Msg::Tick => {}
            Msg::Click => {
                if let Some(os) = &self.offer_source {
                    if let Some(ur) = &self.offer_url {
                        self.fetch_task = self.fetch()
                    } else {
                        notify_danger("Please enter a valid Offer URL.")
                    }
                } else {
                    notify_danger("Please link an Offer Source.")
                }
            }
            Msg::UpdateName(i) => self.name = i.value,
            Msg::UpdatePayoutCurrency(currency) => self.payout_currency = currency,
            Msg::UpdateNotes(i) => self.notes = i.value,
            Msg::FetchData(response) => {
                self.fetch_task = None;
                self.props.state.borrow().crud_update(response);
                hide_uk_modal("#offer");
                self.tt.send(TickTockRequest::Tick);
            }
            Msg::FetchFailed => {
                self.fetch_task = None;
                notify_primary("Fetch Failed");
                hide_uk_modal("#offer");
            }
            Msg::DeserializationFailed => {
                self.fetch_task = None;
                notify_primary("Deserialization Failed");
                hide_uk_modal("#offer");
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(restored_element) = &props.restored_element {
            self.name = restored_element.name.clone();
            self.payout_currency = restored_element.currency;
            self.notes = restored_element.notes.clone();

            self.offer_source = Some(restored_element.offer_source.clone());
            self.tags = restored_element.tags.clone();
            self.offer_url_string = restored_element.url.to_string();
            self.offer_url = Some(restored_element.url.clone());
            self.offer_url_tokens = restored_element.offer_tokens.clone();
            self.payout_type = restored_element.payout_type;
            self.payout_value = restored_element.payout_value;
            self.payout_currency = restored_element.currency;
            if let Some(cap_config) = &restored_element.conversion_cap_config {
                self.conversion_cap_active = true;
                self.conversion_cap_config = Some(cap_config.clone());
            } else {
                self.conversion_cap_active = false;
                self.conversion_cap_config = None;
            }
        } else {
            self.name = "".to_string();

            self.offer_source = None;
            self.tags = vec![];
            self.offer_url_string = String::new();
            self.offer_url = None;
            self.offer_url_tokens = vec![];
            self.payout_type = PayoutType::Auto;
            self.payout_value = Decimal::new(0, 0);
            self.payout_currency = Currency::USD;
            self.conversion_cap_active = false;
            self.conversion_cap_config = None;
            self.payout_currency = Currency::USD;
            self.notes = "".to_string();
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let modal_title = if let ModalType::Update = self.props.modal_type {
            "Update Offer"
        } else {
            "New Offer"
        };

        html! {
        <div id="offer" class="uk-flex-top" uk-modal="bg-close:false;">
           <div class="uk-modal-dialog uk-margin-auto-vertical">
              <button class="uk-modal-close-default" type="button" uk-close=""></button>
              <div class="uk-modal-header">
                 <h2 class="uk-modal-title uk-text-center">{modal_title}</h2>
              </div>
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">

                        <div class="uk-margin">
                            <OfferSourceDropdown state=Rc::clone(&self.props.state) selected=&self.offer_source label="Link Offer Source:" eject=self.link.callback(Msg::UpdateOfferSource) />
                            <CountryDropdown selected=&self.country label="Country " eject=self.link.callback(Msg::UpdateCountry) />
                        </div>

                        <TextInput label="Offer Name:" value=&self.name placeholder="Name" oninput=self.link.callback(Msg::UpdateName) />

                        <TagsSelector tags=&self.tags eject=self.link.callback(Msg::UpdateTags) />

                        <OfferUrlGenerator url_tokens=&self.offer_url_tokens offer_url=&self.offer_url eject=self.link.callback(Msg::UpdateUrl) />

                        <OfferURLTokenSelector selected=&self.offer_url_tokens eject=self.link.callback(Msg::UpdateOfferURLTokens) />

                        <PayoutTypeHandler payout_type=&self.payout_type payout_value=&self.payout_value payout_currency=&self.payout_currency eject_payout_type=self.link.callback(Msg::UpdatePayoutType) eject_payout_value=self.link.callback(Msg::UpdatePayoutValue) eject_payout_currency=self.link.callback(Msg::UpdatePayoutCurrency) />

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

impl CRUDOffer {
    pub fn fetch(&self) -> Option<FetchTask> {
        let data = if let ModalType::Create = self.props.modal_type {
            CRUDElementRequest::Create(PrimeElementBuild::Offer(Offer {
                offer_id: Uuid::new_v4(),
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
                url: self.offer_url.clone().expect("sdfgsdfg"),
                offer_tokens: self.offer_url_tokens.clone(),
                conversion_tracking_method: ConversionTrackingMethod::PostbackURL,
                payout_type: self.payout_type,
                manual_payout_config: None,
                conversion_cap_config: None,
                payout_value: self.payout_value,
                currency: self.payout_currency,
                language: Language::Any,
                clearance: Clearance::Everyone,
                offer_source: self.offer_source.clone().expect("Adrfg3r"),
                notes: self.notes.clone(),
                archived: false,
                last_updated: Utc::now(),
                country: self.country.clone(),
                vertical: Vertical::Many,
            }))
        } else {
            let restored_element = self
                .props
                .restored_element
                .clone()
                .expect("Failed to unwrap restored element");

            CRUDElementRequest::Update(vec![PrimeElementBuild::Offer(Offer {
                offer_id: restored_element.offer_id,
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
                url: self.offer_url.clone().expect("sdfgsdfg"),
                offer_tokens: self.offer_url_tokens.clone(),
                conversion_tracking_method: ConversionTrackingMethod::PostbackURL,
                payout_type: self.payout_type,
                manual_payout_config: None,
                conversion_cap_config: None,
                payout_value: self.payout_value,
                currency: self.payout_currency,
                language: Language::Any,
                clearance: Clearance::Everyone,
                offer_source: self.offer_source.clone().expect("Adrfg3r"),
                notes: self.notes.clone(),
                archived: false,
                last_updated: Utc::now(),
                country: self.country.clone(),
                vertical: Vertical::Many,
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
