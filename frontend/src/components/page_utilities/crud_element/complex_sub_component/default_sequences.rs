use crate::appstate::app_state::AppState;
use crate::components::page_utilities::crud_element::complex_sub_component::plus_button::PlusButton;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::{notify_danger, notify_primary};
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::constant::utility::UUID_PLACEHOLDER;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VList;
use yew_material::MatSwitch;
use yew_services::storage::Area;
use yew_services::StorageService;
use crate::components::page_utilities::crud_element::toggle_switch::ToggleSwitch;

pub enum Msg {
    UpdateWeight(InputData),
    SetActiveElement(ActiveElement),
    RemoveSequence(ActiveElement),
    CreateSequence,
    ToggleActive(ActiveElement),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub toggle_sequence_active: Callback<ActiveElement>,
    pub update_weight: Callback<u8>,
    pub create_sequence: Callback<ActiveElement>,
    pub set_active_element: Callback<ActiveElement>,
    pub remove_sequence: Callback<ActiveElement>,
    pub default_sequences: Vec<Sequence>,
    pub active_element: ActiveElement,
}

pub struct DefaultSequences {
    link: ComponentLink<Self>,
    props: Props,
    weight: String,
}

impl Component for DefaultSequences {
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
            Msg::ToggleActive(ae) => self.props.toggle_sequence_active.emit(ae),

            Msg::RemoveSequence(ae) => self.props.remove_sequence.emit(ae),

            Msg::CreateSequence => self
                .props
                .create_sequence
                .emit(ActiveElement::DefaultSequence(
                    Uuid::parse_str(UUID_PLACEHOLDER).expect("E:54rvs"),
                )),

            Msg::SetActiveElement(ae) => self.props.set_active_element.emit(ae),

            Msg::UpdateWeight(i) => {
                if let Ok(weight) = i.value.parse::<u8>() {
                    self.props.update_weight.emit(weight);
                } else {
                    notify_danger("Please enter a number between 0-255")
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.weight.clear();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                {divider!()}
                                <div class="uk-margin-top-small">
                                    <h3 class="uk-flex-left">{"Default Sequences"}</h3>
                                    <button onclick=self.link.callback(|_| Msg::CreateSequence) class="uk-button uk-button-small uk-button-success"><span class="fas fa-plus uk-margin-small-right"></span>{" New Sequence"}</button>
                                </div>

                                <hr class="uk-divider-small" />

                                {self.render_default_sequences()}
        </>
        }
    }
}

impl DefaultSequences {
    pub fn render_default_sequences(&self) -> VList {
        let mut nodes = VList::new();
        
        if self.props.default_sequences.is_empty() {self.link.send_message(Msg::CreateSequence)}
        
        for sequence in self.props.default_sequences.iter() {
            let sequence_name = sequence.name.clone();
            let sequence_id = sequence.id.clone();
            let is_active = sequence.sequence_is_active;
            let weight = sequence.weight;
            let sequence_selected_style =
                if let ActiveElement::DefaultSequence(seq_id) = self.props.active_element {
                    if seq_id == sequence_id {
                        "border: 2px solid blue;"
                    } else {
                        "border: 2px solid #f4f3f2"
                    }
                } else {
                    ""
                };
            let active_element_sequence = ActiveElement::DefaultSequence(sequence_id.clone());
            let active_element_sequence_two = ActiveElement::DefaultSequence(sequence_id.clone());
            let active_element_sequence_three = ActiveElement::DefaultSequence(sequence_id.clone());

            nodes.push(html!{
                                <div class="uk-margin-small" style=sequence_selected_style onclick=self.link.callback(move |_| Msg::SetActiveElement(active_element_sequence.clone())) >
                                    {label!("g", sequence_name)}
                                    <div class="uk-child-width-1-3" uk-grid="">
                                        <div>{label!("s", "Weight")}<input uk-tooltip="title: Weight" class="uk-input" type="number" value=weight oninput=self.link.callback(|i:InputData| Msg::UpdateWeight(i)) /></div>
                                        <div>{label!("s", "Remove")}<button uk-tooltip="title: Remove Sequence" class="uk-button" onclick=self.link.callback(move |_| Msg::RemoveSequence(active_element_sequence_two.clone())) >{"X"}</button></div>
                                        <div><ToggleSwitch label="Active" size=Some("small".to_string()) checked=is_active onchange=self.link.callback(move |_| Msg::ToggleActive(active_element_sequence_three.clone())) /></div>
                                    </div>
                                </div>
            })
        }

        nodes
    }
}
