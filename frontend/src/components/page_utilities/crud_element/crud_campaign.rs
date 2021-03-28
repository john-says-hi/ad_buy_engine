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
use ad_buy_engine::data::lists::{Currency, Language, Vertical, DataURLToken, CostModel};
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
use ad_buy_engine::data::elements::funnel::{Funnel, ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::lists::click_transition_method::RedirectOption;
use crate::components::page_utilities::crud_element::complex_sub_component::plus_button::PlusButton;
use crate::components::page_utilities::crud_element::complex_sub_component::lhs_conditional_sequences::LHSConditionalSequence;
use crate::components::page_utilities::crud_element::complex_sub_component::lhs_default_sequences::LHSDefaultSequences;
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_funnel_view::RHSFunnelView;
use ad_buy_engine::data::lists::condition::Condition;
use ad_buy_engine::data::elements::campaign::{CampaignDestinationType, Campaign};
use either::Either;
use fancy_regex::Regex;
use crate::components::page_utilities::crud_element::dropdowns::traffic_source_dropdown::TrafficSourceDropdown;
use crate::components::page_utilities::crud_element::dropdowns::cost_model_dropdown::CostModelDropdown;
use crate::components::page_utilities::crud_element::dropdowns::funnel_dropdown::FunnelDropdown;
// use crate::components::page_utilities::crud_element::complex_sub_component::mini_sequence_builder::MiniRHSSequenceBuilder;

pub enum Msg {
    Submit,
    UpdateName(InputData),
    Ignore,
    SelectCountry(Country),
    SelectTrafficSource(TrafficSource),
    SelectCostModel(CostModel),
    SelectDestinationType(CampaignDestinationType),
    SelectFunnel(Funnel),
    UpdateSequence(Sequence),
    UpdateNotes(InputData),
    UpdateCostValue(InputData),
    FetchData(CRUDElementResponse),
    FetchFailed,
    DeserializationFailed,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    #[prop_or_default]
    pub restored_element: Option<Campaign>,
    pub modal_type: ModalType,
}

pub struct CRUDCampaign {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub name: String,
    pub country: Country,
    pub traffic_source: Option<TrafficSource>,
    pub tracking_domain: Url,
    pub destination_type: CampaignDestinationType,
    pub funnel: Option<Funnel>,
    pub sequence: Option<Sequence>,
    pub cost_model: CostModel,
    pub cost_value: Decimal,
    pub tt: Box<dyn Bridge<TickTock>>,

    pub notes: String,
    pub fetch_task: Option<FetchTask>,
}

impl Component for CRUDCampaign {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tt = TickTock::bridge(link.callback(|_| Msg::Ignore));

        let url = props
            .state
            .borrow()
            .account
            .borrow()
            .domains_configuration
            .main_domain
            .clone();

        Self {
            link,
            props,
            name: "New Campaign".to_string(),
            country: Country::Global,
            traffic_source: None,
            tracking_domain: url,
            destination_type: CampaignDestinationType::Funnel,
            funnel: None,
            sequence: None,
            cost_model: CostModel::CPC,
            cost_value: Decimal::from(0),
            notes: "".to_string(),
            fetch_task: None,
            tt,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
            Msg::UpdateCostValue(i) => {
                if let Ok(decimal) = i.value.parse::<Decimal>() {
                    self.cost_value = decimal;
                } else {
                    notify_danger("Please enter valid number")
                }
            }

            Msg::UpdateSequence(seq) => self.sequence = Some(seq),

            Msg::SelectFunnel(funnel) => self.funnel = Some(funnel),

            Msg::SelectDestinationType(dest) => self.destination_type = dest,

            Msg::SelectTrafficSource(ts) => {
                self.traffic_source = Some(ts);
                self.name = self.generate_campaign_name(
                    &self.country.to_string(),
                    &self.traffic_source.clone().unwrap().name,
                    None,
                );
            }

            Msg::SelectCountry(c) => {
                self.country = c;
                self.name = self.generate_campaign_name(
                    &self.country.to_string(),
                    if let Some(ts) = &self.traffic_source {
                        ts.name.as_str()
                    } else {
                        ""
                    },
                    None,
                )
            }

            Msg::SelectCostModel(cm) => self.cost_model = cm,

            Msg::Submit => match self.fetch() {
                Ok(task) => self.fetch_task = task,
                Err(e) => notify_danger(e),
            },

            Msg::UpdateName(i) => {
                self.name = self.generate_campaign_name(
                    &self.country.to_string(),
                    if let Some(ts) = &self.traffic_source {
                        ts.name.as_str()
                    } else {
                        ""
                    },
                    Some(i),
                );
            }

            Msg::UpdateNotes(i) => self.notes = i.value,

            Msg::FetchData(response) => {
                self.fetch_task = None;
                self.props.state.borrow().crud_update(response);
                self.tt.send(TickTockRequest::Tick);
                hide_uk_modal("#campaigns");
            }

            Msg::FetchFailed => {
                self.fetch_task = None;
                notify_primary("Fetch Failed");
                hide_uk_modal("#campaigns");
            }

            Msg::DeserializationFailed => {
                self.fetch_task = None;
                notify_primary("Deserialization Failed");
                hide_uk_modal("#campaigns");
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let rc_state = Rc::clone(&props.state);
        let state = rc_state.borrow();
        if let Some(restored_element) = &props.restored_element {
            let campaign = restored_element.clone();
            self.name = campaign.name.clone();
            self.notes = campaign.notes.clone();
            self.country = campaign.country.clone();
            self.traffic_source = Some(campaign.traffic_source.clone());
            self.tracking_domain = state
                .account
                .borrow()
                .domains_configuration
                .main_domain
                .clone();
            self.destination_type = campaign.campaign_destination;
            self.cost_value = campaign.cost_value;

            match &campaign.campaign_core {
                Either::Left(funnel) => {
                    self.funnel = Some(funnel.clone());
                    self.sequence = None;
                }
                Either::Right(sequence) => {
                    self.sequence = Some(sequence.clone());
                    self.funnel = None;
                }
            }
            self.cost_model = campaign.cost_model;
        } else {
            self.name = self.generate_campaign_name(&Country::Global.to_string(), "", None);
            self.traffic_source = None;
            self.country = Country::Global;
            self.notes = "".to_string();
            self.traffic_source = None;
            self.tracking_domain = state
                .account
                .borrow()
                .domains_configuration
                .main_domain
                .clone();
            self.destination_type = CampaignDestinationType::Funnel;
            self.funnel = None;
            self.sequence = None;
            self.cost_model = CostModel::CPC;
            self.cost_value = Decimal::from(0);
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let modal_title = if let ModalType::Update = self.props.modal_type {
            "Update Campaign"
        } else {
            "Create Campaign"
        };

        let ts_cb = self.link.callback(Msg::SelectTrafficSource);
        let country_cb = self.link.callback(Msg::SelectCountry);
        let name_cb = self.link.callback(Msg::UpdateName);
        let cost_model_cb = self.link.callback(Msg::SelectCostModel);
        let notes_cb = self.link.callback(Msg::UpdateNotes);

        html! {
        <div id="campaigns" class="uk-flex-top" uk-modal="bg-close:false;">
           <div class="uk-modal-dialog uk-margin-auto-vertical">
              <button class="uk-modal-close-default" type="button" uk-close=""></button>
              <div class="uk-modal-header">
                 <h2 class="uk-modal-title uk-text-center">{modal_title}</h2>
              </div>
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">

                        <div class="uk-margin uk-grid-column-collapse uk-grid-collapse uk-child-width-1-2" uk-grid="">
                            <div class="uk-margin-right-small"><span class="uk-label">{"Traffic Source"}</span><TrafficSourceDropdown state=Rc::clone(&self.props.state) onselect=ts_cb /></div>
                            <CountryDropdown selected=self.country eject=country_cb label="Country" />
                        </div>

                        <div class="uk-margin">
                            <span class="uk-label uk-margin-right-small">{"Name"}</span><input type="text" class="uk-input" value=&self.name oninput=name_cb />
                        </div>

                        <div class="uk-margin uk-grid-column-collapse uk-grid-collapse uk-child-width-1-2" uk-grid="">
                            <div class="uk-margin-right-small"><span class="uk-label">{"Cost Model"}</span><CostModelDropdown onselect=cost_model_cb /></div>
                            <div>{self.render_cost_value()}</div>
                        </div>

                        <NotesComponent callback=notes_cb value=&self.notes />

                        <h4>{"Setup Campaign Destination"}</h4>
                        {self.render_destination_button()}
                        <hr class="uk-divider" />
                        {self.render_destination()}

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

impl CRUDCampaign {
    pub fn render_destination_button(&self) -> VNode {
        let funn_cb = self
            .link
            .callback(|_| Msg::SelectDestinationType(CampaignDestinationType::Funnel));
        let seq_cb = self
            .link
            .callback(|_| Msg::SelectDestinationType(CampaignDestinationType::Sequence));
        let f_btn = if let CampaignDestinationType::Funnel = self.destination_type {
            VNode::from(
                html! {<label><input class="uk-radio" type="radio" name="radio2" checked=true onclick=funn_cb />{"Funnel"}</label>},
            )
        } else {
            VNode::from(
                html! {<label><input class="uk-radio" type="radio" name="radio2" onclick=funn_cb />{"Funnel"}</label>},
            )
        };
        let seq_btn = if let CampaignDestinationType::Sequence = self.destination_type {
            VNode::from(
                html! {<label><input class="uk-radio" type="radio" name="radio2" checked=true onclick=seq_cb />{"Sequence"}</label>},
            )
        } else {
            VNode::from(
                html! {<label><input class="uk-radio" type="radio" name="radio2" onclick=seq_cb />{"Sequence"}</label>},
            )
        };

        VNode::from(html! {
        <div class="uk-margin uk-grid-small uk-child-width-auto uk-grid">
            {f_btn}
            {seq_btn}
        </div>
        })
    }

    pub fn render_destination(&self) -> VNode {
        match self.destination_type {
            CampaignDestinationType::Funnel => {
                let f_cb = self.link.callback(Msg::SelectFunnel);
                VNode::from(
                    html! {<div class="uk-margin"><h3>{"Select Funnel"}</h3><FunnelDropdown state=Rc::clone(&self.props.state) onselect=f_cb /></div>},
                )
            }

            CampaignDestinationType::Sequence => {
                let seq_cb = self.link.callback(Msg::UpdateSequence);
                VNode::from(html! {
                // <MiniRHSSequenceBuilder state=Rc::clone(&self.props.state) update_sequence=seq_cb restored_sequence=&self.sequence />
                })
            }
        }
    }

    pub fn render_cost_value(&self) -> VNode {
        let currency = if let Some(ts) = &self.traffic_source {
            ts.currency.to_string()
        } else {
            Currency::USD.to_string()
        };
        let cost_value_cb = self.link.callback(Msg::UpdateCostValue);
        if CostModel::CPC == self.cost_model
            || CostModel::CPA == self.cost_model
            || CostModel::CPM == self.cost_model
        {
            VNode::from(
                html! {<><span class="uk-label">{"Cost Value"}</span><input type="number" class="uk-input uk-margin" value=self.cost_value.to_string() oninput=cost_value_cb /><span class="uk-margin-small-left">{currency}</span></>},
            )
        } else if CostModel::RevShare == self.cost_model {
            VNode::from(
                html! {<><span class="uk-label">{"Percentage Share"}</span><input type="number" class="uk-input uk-margin" value=self.cost_value.to_string() oninput=cost_value_cb /><span class="uk-margin-small-left">{format!("{}   {}","%", currency)}</span></>},
            )
        } else {
            VNode::from(html! {})
        }
    }

    pub fn fetch(&self) -> Result<Option<FetchTask>, &str> {
        let campaign_core = match self.destination_type {
            CampaignDestinationType::Funnel => {
                if let Some(funnel) = &self.funnel {
                    Either::Left(funnel.clone())
                } else {
                    return Err("No funnel selected");
                }
            }
            CampaignDestinationType::Sequence => {
                if let Some(sequence) = &self.sequence {
                    Either::Right(sequence.clone())
                } else {
                    return Err("No sequence selected");
                }
            }
        };

        let traffic_source = if let Some(ts) = &self.traffic_source {
            ts.clone()
        } else {
            return Err("No traffic source selected");
        };

        let data = if let ModalType::Create = self.props.modal_type {
            CRUDElementRequest::Create(PrimeElementBuild::Campaign(Campaign {
                campaign_id: Uuid::new_v4(),
                account_id: self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .account_id
                    .clone(),
                country: self.country.clone(),
                name: self.name.clone(),
                cost_model: self.cost_model,
                clearance: Clearance::Everyone,
                traffic_source,
                redirect_option: RedirectOption::Redirect,
                campaign_destination: self.destination_type,
                campaign_core,
                notes: self.notes.clone(),
                archived: false,
                last_updated: Utc::now(),
                last_clicked: Utc::now(),
                cost_value: self.cost_value,
                hosts: vec![],
            }))
        } else {
            let restored_element = self
                .props
                .restored_element
                .clone()
                .expect("Failed to unwrap restored element");

            CRUDElementRequest::Update(vec![PrimeElementBuild::Campaign(Campaign {
                campaign_id: restored_element.campaign_id.clone(),
                account_id: self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .account_id
                    .clone(),
                country: self.country.clone(),
                name: self.name.clone(),
                cost_model: self.cost_model,
                clearance: Clearance::Everyone,
                traffic_source,
                redirect_option: RedirectOption::Redirect,
                campaign_destination: self.destination_type,
                campaign_core,
                notes: self.notes.clone(),
                archived: false,
                last_updated: Utc::now(),
                last_clicked: restored_element.last_clicked.clone(),
                cost_value: self.cost_value,
                hosts: vec![],
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
        Ok(Some(fetch_task))
    }

    fn render_campaign_url(&self) -> Html {
        if let Some(restored_campaign) = &self.props.restored_element {
            let mut url = self.tracking_domain.clone();
            url.set_path(restored_campaign.campaign_id.to_string().as_str());
            let q_str = restored_campaign.traffic_source.generate_query();
            url.set_query(if q_str.is_empty() {
                None
            } else {
                Some(q_str.as_str())
            });

            VNode::from(
                html! {<div class="uk-margin"><h4>{"Campaign URL"}</h4><input type="text" class="uk-input" value=url.to_string() /></div>},
            )
        } else {
            VNode::from(
                html! {<div class="uk-margin"><h4>{"Campaign URL"}</h4><p>{"Please Save Campaign to Generate URL"}</p></div>},
            )
        }
    }

    fn generate_campaign_name(
        &self,
        country: &str,
        traffic_sources_name: &str,
        input_data: Option<InputData>,
    ) -> String {
        let mut name = if self.name.is_empty() {
            format!("{} - {} - ", country, traffic_sources_name)
        } else {
            self.name.clone()
        };

        if let Some(i) = input_data {
            let minimum_re = Regex::new(r".*?-.*?- ").expect("RE:f43sdf");

            if let Ok(Some(x)) = minimum_re.find(&self.name) {
                let name_re = Regex::new(r"([^-]*)$").expect("Re:g54se");
                if let Ok(Some(data)) = name_re.find(&self.name) {
                    name = self.name.replace(data.as_str().trim(), i.value.as_str());
                }
            }
        } else {
            let traffic_re = Regex::new(r".*?(?= -)").expect("RE: g534swfdg");
            let country_re = Regex::new(r"(?<=- ).*(?= -)").expect("RE: gt45w45yh");

            if let Ok(Some(data)) = traffic_re.find(&self.name) {
                name = self.name.replace(data.as_str(), traffic_sources_name);
            }

            if let Ok(Some(data)) = country_re.find(&self.name) {
                name = self.name.replace(data.as_str(), country);
            }
        }
        name
    }
}
