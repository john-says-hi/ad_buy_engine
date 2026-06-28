use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::landing_page_selector::LandingPageSelector;
// use crate::components::page_utilities::crud_element::complex_sub_component::listicle_builder::MatrixBuilder;
use super::super::crud_funnels::Msg as FunnelMsg;
use crate::components::page_utilities::crud_element::complex_sub_component::matrix_builder::{
    MatrixBuilder, RootMatrix,
};
use crate::components::page_utilities::crud_element::complex_sub_component::offer_selector::OfferSelector;
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_funnel_view_basic::RHSFunnelViewBasic;
use crate::components::page_utilities::crud_element::crud_funnels::{ActiveElement, CRUDFunnel};
use crate::components::page_utilities::crud_element::dropdowns::referrer_handling_dropdown::ReferrerHandlingDropdown;
use crate::components::page_utilities::crud_element::dropdowns::sequence_type_dropdown::SequenceTypeDropdown;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::matrix::{Matrix, MatrixData};
use ad_buy_engine::data::elements::offer::Offer;
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
use ad_buy_engine::Country;
use either::Either;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use strum::IntoEnumIterator;
use uuid::Uuid;
use web_sys::Element;
use yew::format::Json;
use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    UpdateSequenceName(InputData),
    UpdateSequenceReferrerHandling(ReferrerHandling),
    UpdateSequenceType(SequenceType),
    UpdateSequence(Sequence),
    OnBlurName,
    UpdateRootMatrix(Arc<RwLock<Matrix>>),
    ToggleRHSExpand,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub active_element: ActiveElement,
    pub default_sequences: Vec<Sequence>,
    pub conditional_sequences: Vec<ConditionalSequence>,
    pub update_sequence: Callback<Sequence>,
    pub update_name: Callback<String>,
    pub update_referrer_handling: Callback<ReferrerHandling>,
    pub update_sequence_type: Callback<SequenceType>,
    pub funnel_link: Scope<CRUDFunnel>,
    pub expand: bool,
}

pub struct RHSSequenceBuilder {
    link: ComponentLink<Self>,
    props: Props,
    weight: String,
    name: String,
    sequence_type: SequenceType,
    referrer_handling: ReferrerHandling,
}

impl Component for RHSSequenceBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let name = match props.active_element {
            ActiveElement::DefaultSequence(id) => props
                .default_sequences
                .iter()
                .find(|s| s.id == id)
                .map(|s| s.name.clone())
                .unwrap_or("".to_string()),
            ActiveElement::ConditionalSequence((cid, Some(sid))) => props
                .conditional_sequences
                .iter()
                .find(|s| s.id == cid)
                .map(|s| {
                    s.sequences
                        .iter()
                        .find(|s| s.id == sid)
                        .map(|s| s.name.clone())
                        .unwrap_or("".to_string())
                })
                .unwrap_or("".to_string()),
            _ => format!(""),
        };

        Self {
            link,
            props,
            weight: "".to_string(),
            name,
            sequence_type: SequenceType::OffersOnly,
            referrer_handling: ReferrerHandling::RemoveAll,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleRHSExpand => self
                .props
                .funnel_link
                .send_message(FunnelMsg::ToggleRHSExpand),

            Msg::UpdateRootMatrix(root_matrix) => {
                if let Some(mut sequence) = self.return_active_sequence().cloned() {
                    sequence.matrix = arc!(root_matrix);
                    self.props.update_sequence.emit(sequence);
                } else {
                    notify_danger("Err: Could not extract active sequence")
                }
            }

            Msg::UpdateSequence(seq) => self.props.update_sequence.emit(seq),

            Msg::UpdateSequenceType(seq_type) => self.props.update_sequence_type.emit(seq_type),

            Msg::UpdateSequenceName(i) => self.name = i.value,

            Msg::OnBlurName => self.props.update_name.emit(self.name.clone()),

            Msg::UpdateSequenceReferrerHandling(data) => {
                self.props.update_referrer_handling.emit(data)
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match &props.active_element {
            ActiveElement::DefaultSequence(id) => {
                if let Some(new_seq) = props.default_sequences.iter().find(|s| &s.id == id) {
                    if let Some(old_seq) = self.props.default_sequences.iter().find(|s| &s.id == id)
                    {
                        if new_seq.sequence_type != old_seq.sequence_type {
                            let arc_matrix = arc!(new_seq.matrix);
                            let mut matrix = arc_matrix.write().expect("G%Rf");
                            matrix.children_groups.clear();
                            matrix
                                .children_groups
                                .push(vec![Arc::new(RwLock::new(Matrix::void(
                                    Some(arc!(arc_matrix)),
                                    0,
                                    0,
                                    1,
                                )))]);
                        }
                    }
                }
            }
            ActiveElement::ConditionalSequence((cid, Some(id))) => {
                if let Some(found_c) = props.conditional_sequences.iter().find(|s| &s.id == cid) {
                    if let Some(new_seq) = found_c.sequences.iter().find(|s| &s.id == id) {
                        if let Some(old_c) = self
                            .props
                            .conditional_sequences
                            .iter()
                            .find(|s| &s.id == cid)
                        {
                            if let Some(old_seq) = old_c.sequences.iter().find(|s| &s.id == id) {
                                if new_seq.sequence_type != old_seq.sequence_type {
                                    let arc_matrix = arc!(new_seq.matrix);
                                    let mut matrix = arc_matrix.write().expect("G%Rf");
                                    matrix.children_groups.clear();
                                    matrix.children_groups.push(vec![Arc::new(RwLock::new(
                                        Matrix::void(Some(arc!(arc_matrix)), 0, 0, 1),
                                    ))]);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        match props.active_element {
            ActiveElement::DefaultSequence(id) => {
                self.name = props
                    .default_sequences
                    .iter()
                    .find(|s| s.id == id)
                    .map(|s| s.name.clone())
                    .unwrap_or("".to_string());
            }
            ActiveElement::ConditionalSequence((cid, Some(sid))) => {
                self.name = props
                    .conditional_sequences
                    .iter()
                    .find(|s| s.id == cid)
                    .map(|s| {
                        s.sequences
                            .iter()
                            .find(|s| s.id == sid)
                            .map(|s| s.name.clone())
                            .unwrap_or("".to_string())
                    })
                    .unwrap_or("".to_string())
            }
            _ => self.name = format!(""),
        }

        self.props = props;
        self.weight.clear();
        true
    }

    fn view(&self) -> Html {
        let referrer_handling_selected = if let Some(sequence) = self.return_active_sequence() {
            Some(sequence.referrer_handling.clone())
        } else {
            None
        };

        let name = if let Some(sequence) = self.return_active_sequence() {
            sequence.name.clone()
        } else {
            format!("")
        };

        let sequence_type = if let Some(sequence) = self.return_active_sequence() {
            sequence.sequence_type.clone()
        } else {
            SequenceType::OffersOnly
        };

        html! {
        <>
                                <div class="uk-margin uk-flex uk-flex-right">
                                    <span onclick=callback!(self, |_| Msg::ToggleRHSExpand) class=format!("{}", if self.props.expand {"fa fa-compress-arrows-alt fa-lg"} else {"fa fa-expand-arrows-alt fa-lg"})></span>
                                </div>

                                <div class="uk-margin">
                                    {label!("Sequence Name")}
                                    <input type="text" class="uk-input" oninput=self.link.callback(Msg::UpdateSequenceName) value=&self.name onblur=callback!(self, |_| Msg::OnBlurName) />
                                </div>

                                {self.sequence_type()}

                                <ReferrerHandlingDropdown state=Rc::clone(&self.props.state) selected=referrer_handling_selected callback=self.link.callback(Msg::UpdateSequenceReferrerHandling) />

                                {self.render_view()}
        </>
        }
    }
}

impl RHSSequenceBuilder {
    pub fn sequence_type(&self) -> VNode {
        let sequence_type = if let Some(sequence) = self.return_active_sequence() {
            sequence.sequence_type.clone()
        } else {
            SequenceType::OffersOnly
        };

        let mut oc = "uk-button uk-button-small".to_string();
        let mut loc = "uk-button uk-button-small".to_string();
        let mut lc = "uk-button uk-button-small".to_string();
        match sequence_type {
            SequenceType::OffersOnly => oc.push_str(" uk-button-success"),
            SequenceType::LandingPageAndOffers => loc.push_str(" uk-button-success"),
            SequenceType::Matrix => lc.push_str(" uk-button-success"),
        }

        html! {
            <div class="uk-flex uk-flex-left uk-margin-small">
                    <div class="uk-margin-small">
                        {label!("Sequence Type")}
                        <div uk-switcher="">
                            <button class=oc onclick=callback!(self, |_| Msg::UpdateSequenceType(SequenceType::OffersOnly))>{"Offers Only"}</button>
                            <button class=loc onclick=callback!(self, |_| Msg::UpdateSequenceType(SequenceType::LandingPageAndOffers))>{"Landing Pages & Offers"}</button>
                            <button class=lc onclick=callback!(self, |_| Msg::UpdateSequenceType(SequenceType::Matrix))>{"Matrix"}</button>
                        </div>
                    </div>
            </div>
        }
    }

    pub fn render_view(&self) -> VNode {
        if let Some(sequence) = self.return_active_sequence() {
            let local_matrix = arc!(sequence.matrix);
            let root_matrix = arc!(local_matrix);

            VNode::from(html! {
                <MatrixBuilder root_matrix=root_matrix local_matrix=local_matrix state=Rc::clone(&self.props.state) seq_type=sequence.sequence_type sequence_builder_link=Rc::new(self.link.clone()) />
            })
            // match sequence.sequence_type {
            //     SequenceType::OffersOnly => {
            //         let local_matrix = arc!(self.stored_offers_only.unwrap());
            //         let root_matrix = arc!(self.stored_offers_only.unwrap());
            //         VNode::from(html! {
            //             <MatrixBuilder root_matrix=root_matrix local_matrix=local_matrix state=Rc::clone(&self.props.state) seq_type=sequence.sequence_type sequence_builder_link=Rc::new(self.link.clone()) />
            //         })
            //     }
            //     SequenceType::LandingPageAndOffers => {
            //         let local_matrix = arc!(self.stored_landers_and_offers.unwrap());
            //         let root_matrix = arc!(self.stored_landers_and_offers.unwrap());
            //         VNode::from(html! {
            //             <MatrixBuilder root_matrix=root_matrix local_matrix=local_matrix state=Rc::clone(&self.props.state) seq_type=sequence.sequence_type sequence_builder_link=Rc::new(self.link.clone()) />
            //         })
            //     }
            //     SequenceType::Matrix => {
            //         let local_matrix = arc!(self.stored_matrix.unwrap());
            //         let root_matrix = arc!(self.stored_matrix.unwrap());
            //         VNode::from(html! {
            //             <MatrixBuilder root_matrix=root_matrix local_matrix=local_matrix state=Rc::clone(&self.props.state) seq_type=sequence.sequence_type sequence_builder_link=Rc::new(self.link.clone()) />
            //         })
            //     }
            // }
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
