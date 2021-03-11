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
    UpdateSequenceReferrerHandling(ReferrerHandling),
    UpdateSequenceType(SequenceType),
    UpdateOffers(Vec<WeightedOffer>),
    UpdateLandingPages(Vec<WeightedLandingPage>),
    UpdateSequence(Sequence),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub update_sequence: Callback<Sequence>,
    pub restored_sequence: Option<Sequence>,
}

pub struct MiniSequenceBuilder {
    link: ComponentLink<Self>,
    props: Props,
    referrer_handling: ReferrerHandling,
    sequence_type: SequenceType,
}

impl Component for MiniSequenceBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let rh = if let Some(restored) = &props.restored_sequence {
            restored.referrer_handling.clone()
        } else {
            ReferrerHandling::DoNothing
        };
        let st = if let Some(restored) = &props.restored_sequence {
            restored.sequence_type.clone()
        } else {
            SequenceType::OffersOnly
        };

        Self {
            link,
            props,
            referrer_handling: rh,
            sequence_type: st,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateSequence(seq) => self.props.update_sequence.emit(seq),

            Msg::UpdateLandingPages(lps) => {
                if let Some(mut sequence) = self.props.restored_sequence.clone() {
                    sequence.landing_pages = lps;
                    self.props.update_sequence.emit(sequence);
                } else {
                    notify_danger("Err: Could not extract active sequence")
                }
            }

            Msg::UpdateOffers(offers) => {
                if let Some(mut sequence) = self.props.restored_sequence.clone() {
                    sequence.offers = offers;
                    self.props.update_sequence.emit(sequence);
                } else {
                    notify_danger("Err: Could not extract active sequence")
                }
            }

            Msg::UpdateSequenceType(seq_type) => {
                self.sequence_type = seq_type;
            }

            Msg::UpdateSequenceReferrerHandling(data) => self.referrer_handling = data,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.referrer_handling = if let Some(restored) = &props.restored_sequence {
            restored.referrer_handling.clone()
        } else {
            ReferrerHandling::DoNothing
        };
        self.sequence_type = if let Some(restored) = &props.restored_sequence {
            restored.sequence_type.clone()
        } else {
            SequenceType::OffersOnly
        };
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let referrer_handling_selected = if let Some(sequence) = &self.props.restored_sequence {
            sequence.referrer_handling.clone()
        } else {
            ReferrerHandling::DoNothing
        };

        html! {
        <>
                                <div class="uk-margin">
                                    <h4>{"Sequence Type"}</h4>
                                    <SequenceTypeDropdown eject=self.link.callback(Msg::UpdateSequenceType) />
                                </div>

                                <div class="uk-margin">
                                    <h4>{"Referrer Handling"}</h4>
                                    <ReferrerHandlingDropdown state=Rc::clone(&self.props.state) selected=&self.referrer_handling callback=self.link.callback(Msg::UpdateSequenceReferrerHandling) />
                                </div>

                                <hr class="uk-divider" />

                                {self.render_view()}
        </>
        }
    }
}

impl MiniSequenceBuilder {
    pub fn render_view(&self) -> VNode {
        let sequence = if let Some(sequence) = &self.props.restored_sequence {
            sequence.clone()
        } else {
            Sequence::default()
        };

        match self.sequence_type {
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
                <ListicleBuilder state=Rc::clone(&self.props.state) eject_listicle=self.link.callback(Msg::UpdateSequence) active_sequence=sequence />
            }),
        }
        // } else {
        //     VNode::from(html! {})
        // }
    }
}
