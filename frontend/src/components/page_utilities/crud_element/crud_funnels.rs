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
use crate::{notify_primary, RootComponent, notify_danger, notify_debug};
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
use ad_buy_engine::data::elements::funnel::{Funnel, ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::lists::click_transition_method::RedirectOption;
use crate::components::page_utilities::crud_element::complex_sub_component::plus_button::PlusButton;
use crate::components::page_utilities::crud_element::complex_sub_component::conditional_sequences::ConditionalSequenceConfig;
use crate::components::page_utilities::crud_element::complex_sub_component::default_sequences::DefaultSequences;
use crate::components::page_utilities::crud_element::complex_sub_component::funnel_view_renderer::FunnelViewRenderer;
use ad_buy_engine::data::lists::condition::Condition;

pub enum Msg {
    Ignore,
    Submit,
    UpdateName(InputData),
    UpdateFunnelCountry(Country),
    CreateSequence(ActiveElement),
    UpdateReferrerHandling(ReferrerHandling),
    UpdateSequenceType(SequenceType),
    UpdateSequence(Sequence),
    SetActiveElement(ActiveElement),
    UpdateWeight(u8),
    RemoveSequence(ActiveElement),
    ToggleSequenceActive(ActiveElement),
    UpdateNotes(InputData),
    FetchData(CRUDElementResponse),
    FetchFailed,
    DeserializationFailed,
    UpdateSequenceConditions(Vec<Condition>),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ActiveElement {
    Funnel,
    ConditionalSequence((Uuid, Option<Uuid>)),
    DefaultSequence(Uuid),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    #[prop_or_default]
    pub restored_element: Option<Funnel>,
    pub modal_type: ModalType,
}

pub struct CRUDFunnel {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub funnel_name: String,
    pub country: Country,
    pub default_referrer_handling: ReferrerHandling,
    pub conditional_sequences: Vec<ConditionalSequence>,
    pub default_sequences: Vec<Sequence>,
    pub notes: String,
    pub fetch_task: Option<FetchTask>,
    pub active_element: ActiveElement,
    pub tt: Box<dyn Bridge<TickTock>>,
}

impl Component for CRUDFunnel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tt = TickTock::bridge(link.callback(|_| Msg::Ignore));
        let init_default_seq = Sequence {
            id: Uuid::new_v4(),
            name: "New Default Sequence".to_string(),
            weight: 100,
            sequence_type: SequenceType::OffersOnly,
            redirect_option: RedirectOption::Redirect,
            referrer_handling: ReferrerHandling::DoNothing,
            pre_landing_page: None,
            listicle_pairs: vec![],
            landing_pages: vec![],
            offers: vec![],
            weight_optimization_active: false,
            sequence_is_active: true,
        };

        Self {
            link,
            props,
            funnel_name: "New Funnel".to_string(),
            country: Country::Global,
            default_referrer_handling: ReferrerHandling::DoNothing,
            conditional_sequences: vec![],
            default_sequences: vec![init_default_seq],
            notes: "".to_string(),
            fetch_task: None,
            active_element: ActiveElement::Funnel,
            tt,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateSequenceConditions(conditions) => {
                if let ActiveElement::ConditionalSequence((condi_id, None)) = self.active_element {
                    self.conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                        .map(|s| s.condition_set = conditions);
                }
            }

            Msg::UpdateFunnelCountry(country) => self.country = country,

            Msg::UpdateSequence(sequence) => match self.active_element {
                ActiveElement::DefaultSequence(seq_id) => {
                    if let Some(pos) = self.default_sequences.iter().position(|s| s.id == seq_id) {
                        self.default_sequences.remove(pos);
                        self.default_sequences.insert(pos, sequence);
                    } else {
                        notify_danger("Err: Sequence not found in state")
                    }
                }

                ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => {
                    if let Some(condi_seq) = self
                        .conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                    {
                        if let Some(pos) = condi_seq.sequences.iter().position(|s| s.id == seq_id) {
                            condi_seq.sequences.remove(pos);
                            condi_seq.sequences.insert(pos, sequence);
                        } else {
                            notify_danger("Err: Could not find sequence in conditional sequence")
                        }
                    } else {
                        notify_danger("Err: Could not find Conditional Sequence in state")
                    }
                }

                _ => {}
            },

            Msg::UpdateSequenceType(seq_type) => match self.active_element {
                ActiveElement::DefaultSequence(seq_id) => {
                    self.default_sequences
                        .iter_mut()
                        .find(|s| s.id == seq_id)
                        .map(|s| s.sequence_type = seq_type);
                }

                ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => {
                    self.conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                        .map(|s| {
                            s.sequences
                                .iter_mut()
                                .find(|s| s.id == seq_id)
                                .map(|s| s.sequence_type = seq_type)
                        });
                }

                _ => {}
            },

            Msg::UpdateReferrerHandling(ref_handling) => match self.active_element {
                ActiveElement::Funnel => self.default_referrer_handling = ref_handling,

                ActiveElement::DefaultSequence(seq_id) => {
                    self.default_sequences
                        .iter_mut()
                        .find(|s| s.id == seq_id)
                        .map(|s| s.referrer_handling = ref_handling);
                }

                ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => {
                    self.conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                        .map(|s| {
                            s.sequences
                                .iter_mut()
                                .find(|s| s.id == seq_id)
                                .map(|s| s.referrer_handling = ref_handling)
                        });
                }

                _ => {}
            },

            Msg::ToggleSequenceActive(active_element) => match active_element {
                ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => {
                    self.conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                        .map(|s| {
                            s.sequences
                                .iter_mut()
                                .find(|s| s.id == seq_id)
                                .map(|s| s.sequence_is_active = !s.sequence_is_active)
                        });
                }

                ActiveElement::ConditionalSequence((condi_id, None)) => {
                    self.conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                        .map(|s| {
                            s.conditional_sequence_is_active = !s.conditional_sequence_is_active
                        });
                }

                ActiveElement::DefaultSequence(seq_id) => {
                    self.default_sequences
                        .iter_mut()
                        .find(|s| s.id == seq_id)
                        .map(|s| s.sequence_is_active = !s.sequence_is_active);
                }

                _ => {}
            },

            Msg::RemoveSequence(active_element) => match active_element {
                ActiveElement::ConditionalSequence((condi_id, None)) => {
                    self.conditional_sequences.retain(|s| s.id != condi_id);
                }

                ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => {
                    if let Some(condi) = self
                        .conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                    {
                        condi.sequences.retain(|s| s.id != seq_id)
                    }
                }

                ActiveElement::DefaultSequence(seq_id) => {
                    self.default_sequences.retain(|s| s.id != seq_id);
                }

                _ => {}
            },

            Msg::UpdateWeight(weight) => match self.active_element {
                ActiveElement::DefaultSequence(seq_id) => {
                    self.default_sequences
                        .iter_mut()
                        .find(|s| s.id == seq_id)
                        .map(|s| s.weight = weight);
                }

                ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => {
                    self.conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                        .map(|s| {
                            s.sequences
                                .iter_mut()
                                .find(|s| s.id == seq_id)
                                .map(|s| s.weight = weight)
                        });
                }

                _ => {}
            },

            Msg::CreateSequence(active_element) => {
                let new_id = Uuid::new_v4();

                match active_element {
                    ActiveElement::Funnel => {
                        self.conditional_sequences.push(ConditionalSequence {
                            id: new_id,
                            name: "New Conditional Sequence".to_string(),
                            condition_set: vec![],
                            sequences: vec![],
                            conditional_sequence_is_active: true,
                        });
                        self.active_element =
                            ActiveElement::ConditionalSequence((new_id.clone(), None));
                    }

                    ActiveElement::ConditionalSequence((condi_id, None)) => {
                        self.conditional_sequences
                            .iter_mut()
                            .find(|s| s.id == condi_id)
                            .map(|s| {
                                s.sequences.push(Sequence {
                                    id: new_id,
                                    name: "New Sequence".to_string(),
                                    weight: 100,
                                    sequence_type: SequenceType::OffersOnly,
                                    redirect_option: RedirectOption::Redirect,
                                    referrer_handling: ReferrerHandling::DoNothing,
                                    pre_landing_page: None,
                                    listicle_pairs: vec![],
                                    landing_pages: vec![],
                                    offers: vec![],
                                    weight_optimization_active: false,
                                    sequence_is_active: true,
                                })
                            });
                        self.active_element = ActiveElement::ConditionalSequence((
                            condi_id.clone(),
                            Some(new_id.clone()),
                        ));
                    }

                    ActiveElement::DefaultSequence(_) => {
                        self.default_sequences.push(Sequence {
                            id: new_id,
                            name: "New Default Sequence".to_string(),
                            weight: 100,
                            sequence_type: SequenceType::OffersOnly,
                            redirect_option: RedirectOption::Redirect,
                            referrer_handling: ReferrerHandling::DoNothing,
                            pre_landing_page: None,
                            listicle_pairs: vec![],
                            landing_pages: vec![],
                            offers: vec![],
                            weight_optimization_active: false,
                            sequence_is_active: true,
                        });
                        self.active_element = ActiveElement::DefaultSequence(new_id.clone());
                    }

                    _ => {}
                }
            }

            Msg::SetActiveElement(element) => self.active_element = element,

            Msg::Ignore => {}

            Msg::Submit => self.fetch_task = self.fetch(),

            Msg::UpdateName(i) => match self.active_element {
                ActiveElement::Funnel => self.funnel_name = i.value,

                ActiveElement::DefaultSequence(seq_id) => {
                    self.default_sequences
                        .iter_mut()
                        .find(|s| s.id == seq_id)
                        .map(|s| s.name = i.value);
                }

                ActiveElement::ConditionalSequence((condi_id, None)) => {
                    self.conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                        .map(|s| s.name = i.value);
                }

                ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => {
                    self.conditional_sequences
                        .iter_mut()
                        .find(|s| s.id == condi_id)
                        .map(|s| {
                            s.sequences
                                .iter_mut()
                                .find(|s| s.id == seq_id)
                                .map(|s| s.name = i.value)
                        });
                }
            },

            Msg::UpdateNotes(i) => self.notes = i.value,

            Msg::FetchData(response) => {
                self.fetch_task = None;
                self.props.state.borrow().crud_update(response);
                self.tt.send(TickTockRequest::Tick);
                hide_uk_modal("#funnels");
            }

            Msg::FetchFailed => {
                self.fetch_task = None;
                notify_primary("Fetch Failed");
                hide_uk_modal("#funnels");
            }

            Msg::DeserializationFailed => {
                self.fetch_task = None;
                notify_primary("Deserialization Failed");
                hide_uk_modal("#funnels");
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.active_element = ActiveElement::Funnel;

        if let Some(restored_element) = &props.restored_element {
            self.funnel_name = restored_element.name.clone();
            self.notes = restored_element.notes.clone();
            self.country = restored_element.country.clone();
            self.default_sequences = restored_element.default_sequences.clone();
            self.conditional_sequences = restored_element.conditional_sequences.clone();
            self.default_referrer_handling = restored_element.referrer_handling.clone();
        } else {
            self.default_sequences = vec![];
            self.conditional_sequences = vec![];
            self.default_referrer_handling = ReferrerHandling::DoNothing;
            self.country = Country::Global;
            self.funnel_name = "New Funnel".to_string();
            self.notes = "".to_string();
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let modal_title = if let ModalType::Update = self.props.modal_type {
            "Update Funnel"
        } else {
            "Create Funnel"
        };

        html! {
        <div id="funnels" class="uk-flex-top uk-modal-container" uk-modal="bg-close:false;">
           <div class="uk-modal-dialog uk-margin-auto-vertical">
              <button class="uk-modal-close-default" type="button" uk-close=""></button>
              <div class="uk-modal-header">
                 <h2 class="uk-modal-title uk-text-center">{modal_title}</h2>
              </div>
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-2 uk-grid-divider" uk-grid="">

                        <div class="uk-margin uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1">

                            <div class="uk-margin-top-large uk-margin-bottom-large" onclick=self.link.callback(|_| Msg::SetActiveElement(ActiveElement::Funnel)) >
                                <h2>{format!("{} - {}", self.country.to_string(), &self.funnel_name)}</h2>
                            </div>

                            <div class="uk-margin-top-large uk-margin-bottom-large uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">
                                <ConditionalSequenceConfig active_element=&self.active_element conditional_sequences=&self.conditional_sequences create_sequence=self.link.callback(Msg::CreateSequence) set_active_element=self.link.callback(Msg::SetActiveElement) remove_sequence=self.link.callback(Msg::RemoveSequence) update_weight=self.link.callback(Msg::UpdateWeight) toggle_sequence_active=self.link.callback(Msg::ToggleSequenceActive) />
                            </div>

                            <div class="uk-margin-top-large uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">
                                <DefaultSequences active_element=&self.active_element default_sequences=&self.default_sequences create_sequence=self.link.callback(Msg::CreateSequence) set_active_element=self.link.callback(Msg::SetActiveElement) remove_sequence=self.link.callback(Msg::RemoveSequence) update_weight=self.link.callback(Msg::UpdateWeight) toggle_sequence_active=self.link.callback(Msg::ToggleSequenceActive) />
                             </div>

                        </div>

                        <div class="uk-margin uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1" uk-grid="">
                            <FunnelViewRenderer state=Rc::clone(&self.props.state) default_sequences=&self.default_sequences conditional_sequences=&self.conditional_sequences funnel_name=&self.funnel_name funnel_country=&self.country default_referrer_handling=&self.default_referrer_handling notes=&self.notes active_element=&self.active_element update_sequence=self.link.callback(Msg::UpdateSequence) update_sequence_conditions=self.link.callback(Msg::UpdateSequenceConditions) update_name=self.link.callback(Msg::UpdateName) update_country=self.link.callback(Msg::UpdateFunnelCountry) update_referrer_handling=self.link.callback(Msg::UpdateReferrerHandling) update_sequence_type=self.link.callback(Msg::UpdateSequenceType) update_notes=self.link.callback(Msg::UpdateNotes) />
                        </div>

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

impl CRUDFunnel {
    pub fn fetch(&self) -> Option<FetchTask> {
        let data = if let ModalType::Create = self.props.modal_type {
            CRUDElementRequest::Create(PrimeElementBuild::Funnel(Funnel {
                funnel_id: Uuid::new_v4(),
                account_id: self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .account_id
                    .clone(),
                country: self.country.clone(),
                name: self.funnel_name.clone(),
                clearance: Clearance::Everyone,
                redirect_option: RedirectOption::Redirect,
                referrer_handling: self.default_referrer_handling.clone(),
                notes: self.notes.clone(),
                conditional_sequences: self.conditional_sequences.clone(),
                default_sequences: self.default_sequences.clone(),
                archived: false,
                last_updated: Utc::now(),
            }))
        } else {
            let restored_element = self
                .props
                .restored_element
                .clone()
                .expect("Failed to unwrap restored element");

            CRUDElementRequest::Update(vec![PrimeElementBuild::Funnel(Funnel {
                funnel_id: restored_element.funnel_id,
                account_id: self
                    .props
                    .state
                    .borrow()
                    .account
                    .borrow()
                    .account_id
                    .clone(),
                country: self.country.clone(),
                name: self.funnel_name.clone(),
                clearance: Clearance::Everyone,
                redirect_option: RedirectOption::Redirect,
                referrer_handling: self.default_referrer_handling.clone(),
                notes: self.notes.clone(),
                conditional_sequences: self.conditional_sequences.clone(),
                default_sequences: self.default_sequences.clone(),
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
