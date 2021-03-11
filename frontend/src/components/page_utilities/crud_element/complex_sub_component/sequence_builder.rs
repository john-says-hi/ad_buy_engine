use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::funnel_view_basic_data::FunnelViewBasicData;
use crate::components::page_utilities::crud_element::complex_sub_component::landing_page_selector::LandingPageSelector;
use crate::components::page_utilities::crud_element::complex_sub_component::listicle_builder::ListicleBuilder;
use crate::components::page_utilities::crud_element::complex_sub_component::offer_selector::OfferSelector;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::referrer_handling_dropdown::ReferrerHandlingDropdown;
use crate::components::page_utilities::crud_element::dropdowns::sequence_type_dropdown::SequenceTypeDropdown;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::elements::landing_page::{LandingPage, WeightedLandingPage};
use ad_buy_engine::data::elements::offer::WeightedOffer;
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
use ad_buy_engine::Country;
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_material::MatSwitch;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    UpdateSequenceName(InputData),
    UpdateSequenceReferrerHandling(ReferrerHandling),
    UpdateSequenceType(SequenceType),
    UpdateOffers(Vec<WeightedOffer>),
    UpdateLandingPages(Vec<WeightedLandingPage>),
    UpdateSequence(Sequence),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub active_element: ActiveElement,
    pub default_sequences: Vec<Sequence>,
    pub conditional_sequences: Vec<ConditionalSequence>,
    pub update_sequence: Callback<Sequence>,
    pub update_name: Callback<InputData>,
    pub update_referrer_handling: Callback<ReferrerHandling>,
    pub update_sequence_type: Callback<SequenceType>,
}

pub struct SequenceBuilder {
    link: ComponentLink<Self>,
    props: Props,
    weight: String,
}

impl Component for SequenceBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            weight: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateSequence(seq) => self.props.update_sequence.emit(seq),

            Msg::UpdateLandingPages(lps) => {
                if let Some(mut sequence) = self.return_active_sequence().cloned() {
                    sequence.landing_pages = lps;
                    self.props.update_sequence.emit(sequence);
                } else {
                    notify_danger("Err: Could not extract active sequence")
                }
            }

            Msg::UpdateOffers(offers) => {
                if let Some(mut sequence) = self.return_active_sequence().cloned() {
                    sequence.offers = offers;
                    self.props.update_sequence.emit(sequence);
                } else {
                    notify_danger("Err: Could not extract active sequence")
                }
            }

            Msg::UpdateSequenceType(seq_type) => self.props.update_sequence_type.emit(seq_type),

            Msg::UpdateSequenceName(i) => self.props.update_name.emit(i),

            Msg::UpdateSequenceReferrerHandling(data) => {
                self.props.update_referrer_handling.emit(data)
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        self.weight.clear();
        true
    }

    fn view(&self) -> Html {
        let referrer_handling_selected = if let Some(sequence) = self.return_active_sequence() {
            sequence.referrer_handling.clone()
        } else {
            ReferrerHandling::DoNothing
        };

        html! {
        <>
                                <div class="uk-margin">
                                    <h4>{"Name"}</h4>
                                    <input type="text" class="uk-input" oninput=self.link.callback(Msg::UpdateSequenceName) />
                                </div>

                                <div class="uk-margin">
                                    <h4>{"Sequence Type"}</h4>
                                    <SequenceTypeDropdown eject=self.link.callback(Msg::UpdateSequenceType) />
                                </div>

                                <div class="uk-margin">
                                    <h4>{"Referrer Handling"}</h4>
                                    <ReferrerHandlingDropdown state=Rc::clone(&self.props.state) selected=referrer_handling_selected callback=self.link.callback(Msg::UpdateSequenceReferrerHandling) />
                                </div>

                                <hr class="uk-divider" />

                                {self.render_view()}
        </>
        }
    }
}

impl SequenceBuilder {
    pub fn render_view(&self) -> VNode {
        if let Some(sequence) = self.return_active_sequence() {
            match sequence.sequence_type {
                SequenceType::OffersOnly => VNode::from(html! {
                    <OfferSelector state=Rc::clone(&self.props.state) eject_selected_offers=self.link.callback(Msg::UpdateOffers) />
                }),

                SequenceType::LandingPageAndOffers => VNode::from(html! {
                <>
                    <LandingPageSelector state=Rc::clone(&self.props.state) eject_selected_landing_pages=self.link.callback(Msg::UpdateLandingPages) />
                    <OfferSelector state=Rc::clone(&self.props.state) eject_selected_offers=self.link.callback(Msg::UpdateOffers) />
                </>
                }),

                SequenceType::Listicle => VNode::from(html! {
                    <ListicleBuilder state=Rc::clone(&self.props.state) eject_listicle=self.link.callback(Msg::UpdateSequence) active_sequence=sequence.clone() />
                }),
            }
        } else {
            VNode::from(html! {})
        }
    }

    pub fn return_active_sequence(&self) -> Option<&Sequence> {
        match self.props.active_element {
            ActiveElement::DefaultSequence(seq_id) => {
                self.props.default_sequences.iter().find(|s| s.id == seq_id)
            }
            ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => {
                if let Some(condi) = self
                    .props
                    .conditional_sequences
                    .iter()
                    .find(|s| s.id == condi_id)
                {
                    condi.sequences.iter().find(|s| s.id == seq_id)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
