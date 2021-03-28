use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::condition_view::ConditionView;
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_funnel_view_basic::RHSFunnelViewBasic;
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_sequence_builder::RHSSequenceBuilder;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::lists::condition::Condition;
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
    UpdateCountry(Country),
    UpdateName(String),
    UpdateDefaultReferrerHandling(ReferrerHandling),
    UpdateNotes(InputData),
    UpdateSequence(Sequence),
    UpdateSequenceType(SequenceType),
    UpdateConditions(Vec<Condition>),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub default_sequences: Vec<Sequence>,
    pub conditional_sequences: Vec<ConditionalSequence>,
    pub funnel_name: String,
    pub funnel_country: Country,
    pub default_referrer_handling: ReferrerHandling,
    pub notes: String,
    pub active_element: ActiveElement,
    pub update_sequence: Callback<Sequence>,
    pub update_sequence_conditions: Callback<Vec<Condition>>,
    pub update_name: Callback<String>,
    pub update_country: Callback<Country>,
    pub update_referrer_handling: Callback<ReferrerHandling>,
    pub update_sequence_type: Callback<SequenceType>,
    pub update_notes: Callback<InputData>,
}

pub struct RHSFunnelView {
    link: ComponentLink<Self>,
    props: Props,
    weight: String,
}

impl Component for RHSFunnelView {
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
            Msg::UpdateConditions(condis) => self.props.update_sequence_conditions.emit(condis),
            Msg::UpdateSequenceType(seq_type) => self.props.update_sequence_type.emit(seq_type),
            Msg::UpdateSequence(seq) => self.props.update_sequence.emit(seq),
            Msg::UpdateName(i) => self.props.update_name.emit(i),
            Msg::UpdateCountry(country) => self.props.update_country.emit(country),
            Msg::UpdateDefaultReferrerHandling(rh) => self.props.update_referrer_handling.emit(rh),
            Msg::UpdateNotes(i) => self.props.update_notes.emit(i),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                {self.render_view()}
        </>
        }
    }
}

impl RHSFunnelView {
    pub fn render_view(&self) -> VNode {
        let active_element = self.props.active_element.clone();
        let default_sequences = self.props.default_sequences.clone();
        let conditional_sequences = self.props.conditional_sequences.clone();

        match self.props.active_element {
            ActiveElement::Funnel => VNode::from(html! {
                <RHSFunnelViewBasic state=Rc::clone(&self.props.state) funnel_name=&self.props.funnel_name funnel_country=&self.props.funnel_country default_referrer_handling=&self.props.default_referrer_handling notes=&self.props.notes update_name=self.link.callback(Msg::UpdateName) update_country=self.link.callback(Msg::UpdateCountry) update_referrer_handling=self.link.callback(Msg::UpdateDefaultReferrerHandling) update_notes=self.link.callback(Msg::UpdateNotes) />
            }),

            ActiveElement::DefaultSequence(seq_id) => VNode::from(html! {
                <RHSSequenceBuilder state=Rc::clone(&self.props.state) active_element=active_element conditional_sequences=conditional_sequences update_sequence=self.link.callback(Msg::UpdateSequence) update_name=self.link.callback(Msg::UpdateName) update_referrer_handling=self.link.callback(Msg::UpdateDefaultReferrerHandling) update_sequence_type=self.link.callback(Msg::UpdateSequenceType) default_sequences=&self.props.default_sequences />
            }),

            ActiveElement::ConditionalSequence((condi_id, Some(seq_id))) => VNode::from(html! {
                <RHSSequenceBuilder state=Rc::clone(&self.props.state) active_element=active_element conditional_sequences=conditional_sequences update_sequence=self.link.callback(Msg::UpdateSequence) update_name=self.link.callback(Msg::UpdateName) update_referrer_handling=self.link.callback(Msg::UpdateDefaultReferrerHandling) update_sequence_type=self.link.callback(Msg::UpdateSequenceType) default_sequences=&self.props.default_sequences />
            }),

            ActiveElement::ConditionalSequence((condi_id, None)) => {
                let conditional_sequence_name = self
                    .props
                    .conditional_sequences
                    .iter()
                    .find(|s| s.id == condi_id)
                    .map(|s| s.name.clone())
                    .unwrap_or("debug".to_string());

                let conditions = self
                    .props
                    .conditional_sequences
                    .iter()
                    .find(|s| s.id == condi_id)
                    .map(|s| s.condition_set.clone())
                    .unwrap_or(vec![]);

                VNode::from(html! {
                    <ConditionView update_name=self.link.callback(Msg::UpdateName) update_conditions=self.link.callback(Msg::UpdateConditions) conditional_sequence_name=conditional_sequence_name conditions=conditions />
                })
            }
        }
    }
}
