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
use crate::{notify_primary, RootComponent};
use ad_buy_engine::constant::apis::private::API_CRUD_ELEMENT;
use ad_buy_engine::constant::browser_storage_keys::OFFER_SOURCES;
use ad_buy_engine::data::account::domains_configuration::CustomDomainName;
use ad_buy_engine::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use ad_buy_engine::data::custom_events::{CustomConversionEvent, CustomConversionEventToken};
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

use crate::components::page_utilities::crud_element::toggle_switch::ToggleSwitch;
use yew_services::fetch::{FetchTask, Request, Response};
use yew_services::storage::Area;
use yew_services::{FetchService, StorageService};

pub enum Msg {
    Ignore,
    Tick,
    Click,
    UpdateName(InputData),
    UpdateClickIDToken(InputData),
    UpdatePayoutToken(InputData),
    UpdateConversionIDToken(InputData),
    UpdateCustomEvents(Vec<CustomConversionEventToken>),
    UpdateTrackingDomain(Url),
    UpdateTrackingMethod(ConversionTrackingMethod),
    ToggleIncludeAdditionalParams,
    UpdatePayoutCurrency(Currency),
    ToggleAppendClickID,
    ToggleAcceptDuplicatePostback,
    UpdateIPsToWhitelist(WhiteListedPostbackIPs),
    UpdateNotes(InputData),
    UpdateReferrerHandling(ReferrerHandling),
    FetchData(CRUDElementResponse),
    FetchFailed,
    DeserializationFailed,
    ToggleWhiteListedPostbackIPs,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    #[prop_or_default]
    pub restored_element: Option<OfferSource>,
    pub modal_type: ModalType,
}

pub struct CRUDOfferSource {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub name: String,
    pub payout_token: String,
    pub click_id_token: String,
    pub conversion_id_token: String,
    pub custom_events: Vec<CustomConversionEventToken>,
    pub include_all_parameters: bool,
    pub tracking_domain: Url,
    pub tracking_method: ConversionTrackingMethod,
    pub payout_currency: Currency,
    pub append_click_id: bool,
    pub accept_duplicate_postback: bool,
    pub whitelist_postback_ips: bool,
    pub whitelisted_postback_ips: WhiteListedPostbackIPs,
    pub referrer_handling: ReferrerHandling,
    pub notes: String,
    pub fetch_task: Option<FetchTask>,
    pub tt: Box<dyn Bridge<TickTock>>,
}

impl Component for CRUDOfferSource {
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
            payout_token: "".to_string(),
            click_id_token: "".to_string(),
            conversion_id_token: "".to_string(),
            custom_events: vec![],
            include_all_parameters: false,
            tracking_domain,
            tracking_method: ConversionTrackingMethod::PostbackURL,
            payout_currency: Currency::USD,
            append_click_id: true,
            accept_duplicate_postback: true,
            whitelist_postback_ips: false,
            whitelisted_postback_ips: WhiteListedPostbackIPs::default(),
            referrer_handling: ReferrerHandling::DoNothing,
            notes: "".to_string(),
            fetch_task: None,
            tt,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
            Msg::Tick => {}
            Msg::Click => self.fetch_task = self.fetch(),
            Msg::UpdateName(i) => self.name = i.value,
            Msg::UpdateClickIDToken(i) => self.click_id_token = i.value,
            Msg::UpdatePayoutToken(i) => self.payout_token = i.value,
            Msg::UpdateConversionIDToken(i) => self.conversion_id_token = i.value,
            Msg::UpdateCustomEvents(events) => {
                self.custom_events = events;
            }
            Msg::UpdateTrackingDomain(url) => self.tracking_domain = url,
            Msg::UpdateTrackingMethod(conversion_method) => {
                self.tracking_method = conversion_method
            }
            Msg::ToggleIncludeAdditionalParams => {
                self.include_all_parameters = !self.include_all_parameters
            }
            Msg::UpdatePayoutCurrency(currency) => self.payout_currency = currency,
            Msg::ToggleAppendClickID => self.append_click_id = !self.append_click_id,
            Msg::ToggleAcceptDuplicatePostback => {
                self.accept_duplicate_postback = !self.accept_duplicate_postback
            }

            Msg::UpdateNotes(i) => self.notes = i.value,
            Msg::UpdateReferrerHandling(data) => self.referrer_handling = data,
            Msg::FetchData(response) => {
                self.fetch_task = None;
                self.props.state.borrow().crud_update(response);
                hide_uk_modal("#offer-sources");
                self.tt.send(TickTockRequest::Tick);
            }
            Msg::FetchFailed => {
                self.fetch_task = None;
                notify_primary("Fetch Failed");
                hide_uk_modal("#offer-sources");
            }
            Msg::DeserializationFailed => {
                self.fetch_task = None;
                notify_primary("Deserialization Failed");
                hide_uk_modal("#offer-sources");
            }
            Msg::ToggleWhiteListedPostbackIPs => {
                self.whitelisted_postback_ips.ips.clear();
                self.whitelisted_postback_ips.ip_nets.clear();
                self.whitelist_postback_ips = !self.whitelist_postback_ips;
            }

            Msg::UpdateIPsToWhitelist(data) => {
                self.whitelisted_postback_ips = data;
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(restored_element) = &props.restored_element {
            self.name = restored_element.name.clone();
            self.payout_token = restored_element.payout_token.clone();
            self.click_id_token = restored_element.click_id_token.clone();
            self.conversion_id_token = restored_element.conversion_id_token.clone();
            self.custom_events = restored_element.custom_events.clone();
            self.include_all_parameters =
                restored_element.include_additional_parameters_in_postback_url;
            self.tracking_domain = restored_element.tracking_domain.clone();
            self.tracking_method = restored_element.conversion_tracking_method;
            self.payout_currency = restored_element.payout_currency;
            self.append_click_id = restored_element.append_click_id;
            self.accept_duplicate_postback = restored_element.accept_duplicate_post_backs;
            self.notes = restored_element.notes.clone();
            self.whitelist_postback_ips =
                if restored_element.whitelisted_postback_ips.ips.is_empty()
                    && restored_element.whitelisted_postback_ips.ip_nets.is_empty()
                {
                    false
                } else {
                    true
                };
            self.whitelisted_postback_ips = restored_element.whitelisted_postback_ips.clone();
            self.referrer_handling = restored_element.referrer_handling.clone();
        } else {
            self.name = "".to_string();
            self.payout_token = "".to_string();
            self.click_id_token = "".to_string();
            self.conversion_id_token = "".to_string();
            self.custom_events = vec![];
            self.include_all_parameters = false;
            self.tracking_domain = props
                .state
                .borrow()
                .account
                .borrow()
                .domains_configuration
                .main_domain
                .clone();
            self.tracking_method = ConversionTrackingMethod::PostbackURL;
            self.payout_currency = Currency::USD;
            self.append_click_id = true;
            self.accept_duplicate_postback = true;
            self.whitelist_postback_ips = false;
            self.whitelisted_postback_ips = WhiteListedPostbackIPs::default();
            self.referrer_handling = ReferrerHandling::DoNothing;
            self.notes = "".to_string();
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let modal_title = if let ModalType::Update = self.props.modal_type {
            "Update Offer Source"
        } else {
            "New Offer Source"
        };

        html! {
        <div id="offer-sources" class="uk-flex-top" uk-modal="bg-close:false;">
           <div class="uk-modal-dialog uk-margin-auto-vertical">
              <button class="uk-modal-close-default" type="button" uk-close=""></button>
              <div class="uk-modal-header">
                 <h2 class="uk-modal-title uk-text-center">{modal_title}</h2>
              </div>
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">
                        <TextInput label="Name of Source:" value=&self.name placeholder="Name" oninput=self.link.callback(Msg::UpdateName) />

                        <div>
                            {label!("Tracking Parameters")}
                            <table class="uk-table uk-table-small">
                                <thead>
                                    <tr>
                                        <th>{"Name"}</th>
                                        <th>{"Token"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                   <tr>
                                      <td>{"Click ID"}</td>
                                      <td>
                                         <TextInput value=self.click_id_token() oninput=self.link.callback(|i:InputData|Msg::UpdateClickIDToken(i))  />
                                      </td>
                                   </tr>
                                   <tr>
                                      <td>{"Payout"}</td>
                                      <td>
                                         <TextInput value=self.payout_token() oninput=self.link.callback(|i:InputData|Msg::UpdatePayoutToken(i))  />
                                      </td>
                                   </tr>
                                   <tr>
                                      <td>{"Conversion ID"}</td>
                                      <td>
                                         <TextInput value=self.conversion_id_token() oninput=self.link.callback(|i:InputData|Msg::UpdateConversionIDToken(i))  />
                                      </td>
                                   </tr>
                                   <tr>
                                      <td>{"Custom Events"}</td>
                                      <td>
                                        <SelectCustomConversionEvents state=Rc::clone(&self.props.state) events=&self.custom_events callback=self.link.callback(Msg::UpdateCustomEvents) />
                                      </td>
                                   </tr>
                                </tbody>
                            </table>
                        </div>

                        <div>
                            <TrackingDomainDropdown state=Rc::clone(&self.props.state) callback=self.link.callback(|url:Url| Msg::UpdateTrackingDomain(url)) />
                            <TrackingMethodDropdown selected=Some(self.tracking_method) callback=self.link.callback(|c:ConversionTrackingMethod|Msg::UpdateTrackingMethod(c)) />
                        </div>

                       <ToggleSwitch label="Include More Parameters".to_string() checked=self.include_all_parameters onchange=self.link.callback(|_|Msg::ToggleIncludeAdditionalParams) />

                        <div class="uk-margin">
                            {label!("Postback URL")}
                           <TextArea rows="4" value=self.generate_tracking_code() oninput=self.link.callback(|_|Msg::Ignore) />
                        </div>

                        <div class="uk-margin">
                            {label!("Payout Currency")}
                       <CurrencyDropdown callback=self.link.callback(|c:Currency|Msg::UpdatePayoutCurrency(c)) />
                        </div>

                       <ToggleSwitch label="Append Click ID".to_string() checked=self.append_click_id onchange=self.link.callback(|_|Msg::ToggleAppendClickID) />
                       <ToggleSwitch label="Accept Duplicate Postbacks".to_string() checked=self.accept_duplicate_postback onchange=self.link.callback(|_|Msg::ToggleAcceptDuplicatePostback) />
                       <ToggleSwitch label="Whitelist Postback URL IPs".to_string() checked=self.whitelist_postback_ips onchange=self.link.callback(|_|Msg::ToggleWhiteListedPostbackIPs) />
                        {self.whitelist_postback_ips()}

                        <div class="uk-margin">
                           <ReferrerHandlingDropdown callback=self.link.callback(Msg::UpdateReferrerHandling) state=Rc::clone(&self.props.state) selected=self.referrer_handling.clone() />
                        </div>

                        <div class="uk-margin">
                           <span class="uk-label uk-label-large uk-label-primary">{"Notes"}</span>
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

impl CRUDOfferSource {
    pub fn fetch(&self) -> Option<FetchTask> {
        let data = if let ModalType::Create = self.props.modal_type {
            CRUDElementRequest::Create(PrimeElementBuild::OfferSource(OfferSource {
                offer_source_id: Uuid::new_v4(),
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
                click_id_token: self.click_id_token.clone(),
                payout_token: self.payout_token.clone(),
                conversion_id_token: self.conversion_id_token.clone(),
                custom_events: self.custom_events.clone(),
                tracking_domain: self.tracking_domain.clone(),
                conversion_tracking_method: self.tracking_method.clone(),
                include_additional_parameters_in_postback_url: self.include_all_parameters,
                payout_currency: self.payout_currency.clone(),
                append_click_id: self.append_click_id,
                accept_duplicate_post_backs: self.accept_duplicate_postback,
                whitelisted_postback_ips: self.whitelisted_postback_ips.clone(),
                referrer_handling: self.referrer_handling.clone(),
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

            CRUDElementRequest::Update(vec![PrimeElementBuild::OfferSource(OfferSource {
                offer_source_id: restored_element.offer_source_id,
                account_id: self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .account_id
                    .clone(),
                name: self.name.clone(),
                clearance: restored_element.clearance.clone(),
                click_id_token: self.click_id_token.clone(),
                payout_token: self.payout_token.clone(),
                conversion_id_token: self.conversion_id_token.clone(),
                custom_events: self.custom_events.clone(),
                tracking_domain: self.tracking_domain.clone(),
                conversion_tracking_method: self.tracking_method.clone(),
                include_additional_parameters_in_postback_url: self.include_all_parameters,
                payout_currency: self.payout_currency.clone(),
                append_click_id: self.append_click_id,
                accept_duplicate_post_backs: self.accept_duplicate_postback,
                whitelisted_postback_ips: self.whitelisted_postback_ips.clone(),
                referrer_handling: self.referrer_handling.clone(),
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

    pub fn whitelist_postback_ips(&self) -> VNode {
        if self.whitelist_postback_ips {
            html! {
                        <WhitelistPostbackIPsComponent whitelisted_postback_ips=&self.whitelisted_postback_ips callback=self.link.callback(Msg::UpdateIPsToWhitelist) />
            }
        } else {
            html! {}
        }
    }
    pub fn generate_tracking_code(&self) -> String {
        let conversion_tokens = format!(
            "cid={}&pay={}&cvid={}{}",
            self.click_id_token(),
            self.payout_token(),
            self.conversion_id_token(),
            if self.custom_event_token().is_empty() {
                "".to_string()
            } else {
                let mut url_str = String::from("&");
                url_str.push_str(self.custom_event_token().as_str());
                url_str
            }
        );
        let additional_parameters = "p1=REPLACE&p2=REPLACE&p3=REPLACE&p4=REPLACE&p5=REPLACE";
        let mut url = self.tracking_domain.clone();
        match self.tracking_method {
            ConversionTrackingMethod::PostbackURL => {
                url.set_path("postback");
                if self.include_all_parameters {
                    url.set_query(Some(
                        format!("{}&{}", conversion_tokens, additional_parameters).as_str(),
                    ))
                } else {
                    url.set_query(Some(&conversion_tokens));
                }
            }
        }
        url.to_string()
    }

    pub fn click_id_token(&self) -> String {
        if self.click_id_token.is_empty() {
            "REPLACE".to_string()
        } else {
            self.click_id_token.clone()
        }
    }
    pub fn payout_token(&self) -> String {
        if self.payout_token.is_empty() {
            "REPLACE".to_string()
        } else {
            self.payout_token.clone()
        }
    }
    pub fn conversion_id_token(&self) -> String {
        if self.conversion_id_token.is_empty() {
            "REPLACE".to_string()
        } else {
            self.conversion_id_token.clone()
        }
    }

    pub fn custom_event_token(&self) -> String {
        let mut prefix = 1;
        let mut url_string = String::new();
        let len = self.custom_events.len();

        for event in self.custom_events.iter() {
            if prefix as usize == len {
                url_string.push_str(
                    format!(
                        "e{}_{}={}",
                        prefix,
                        &event.event.parameter,
                        event.token.clone().unwrap_or("REPLACE".to_string())
                    )
                    .as_str(),
                );
            } else {
                url_string.push_str(
                    format!(
                        "e{}_{}={}&",
                        prefix,
                        &event.event.parameter,
                        event.token.clone().unwrap_or("REPLACE".to_string())
                    )
                    .as_str(),
                );
            }

            prefix += 1;
        }
        url_string
    }
}
