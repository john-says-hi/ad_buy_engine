use crate::appstate::app_state::AppState;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::ConditionalSequence;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VList;
use yew_material::MatSwitch;
use yew_services::storage::Area;
use yew_services::StorageService;
use uuid::Uuid;
use crate::{notify_danger, notify_primary, notify_debug};
use crate::components::page_utilities::crud_element::complex_sub_component::plus_button::PlusButton;

pub enum Msg {
    UpdateWeight(InputData),
    SetActiveElement(ActiveElement),
    RemoveSequence(ActiveElement),
    CreateSequence(ActiveElement),
    CreateConditionalSequence,
    ToggleSequenceActive(ActiveElement),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub toggle_sequence_active: Callback<ActiveElement>,
    pub update_weight: Callback<u8>,
    pub create_sequence: Callback<ActiveElement>,
    pub set_active_element: Callback<ActiveElement>,
    pub remove_sequence:Callback<ActiveElement>,
    pub conditional_sequences: Vec<ConditionalSequence>,
    pub active_element: ActiveElement,
    
}

pub struct ConditionalSequenceConfig {
    link: ComponentLink<Self>,
    props: Props,
    weight:String,
}

impl Component for ConditionalSequenceConfig {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props, weight: "".to_string() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleSequenceActive(ae)=>{

                self.props.toggle_sequence_active.emit(ae)
            },
            Msg::CreateConditionalSequence=>self.props.create_sequence.emit(ActiveElement::Funnel),
            Msg::CreateSequence(ae)=>self.props.create_sequence.emit(ae),
            Msg::RemoveSequence(ae)=>{
                self.props.remove_sequence.emit(ae)
            },
            Msg::SetActiveElement(active_element)=>{
                self.props.set_active_element.emit(active_element);
    
            },
            Msg::UpdateWeight(i)=>{
                    if let Ok(weight)=i.value.parse::<u8>() {
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
                                <div>
                                    <h2 class="uk-flex-left">{"Conditional Sequences"}</h2>
                                    <PlusButton label={"New Conditional Sequence"} eject=self.link.callback(|_|Msg::CreateConditionalSequence) />
                                </div>
                                
                                <hr class="uk-divider" />
                                
                                {self.render_conditional_sequences()}
        </>
        }
    }
}

impl ConditionalSequenceConfig {
    pub fn render_conditional_sequences(&self) -> VList {
        let mut nodes = VList::new();

        for conditional_sequence in self.props.conditional_sequences.iter() {
            let conditional_sequence_name = conditional_sequence.name.clone();
            let conditional_sequence_id = conditional_sequence.id.clone();
            let conditional_sequence_selected_style = if let ActiveElement::ConditionalSequence((id, None))=self.props.active_element {
                if id == conditional_sequence_id {
                    "border: 2px solid blue;"
                } else {
                    ""
                }
            }  else {
                ""
            };
            let active_element_conditional_sequence = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), None));
            let active_element_conditional_sequence_b = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), None));
            let active_element_conditional_sequence_c = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), None));
            let active_element_conditional_sequence_d = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), None));
            
            nodes.push(html! {
                            <>
                                <div style=conditional_sequence_selected_style onclick=self.link.callback(move |_| Msg::SetActiveElement(active_element_conditional_sequence.clone())) >
                                    <h4 class="uk-flex-left">{conditional_sequence_name}</h4>
                                    <div class="uk-flex-right">
                                        <button class="uk-button" onclick=self.link.callback(move |_| Msg::RemoveSequence(active_element_conditional_sequence_b.clone()))  >{"X"}</button>
                                        <MatSwitch checked=conditional_sequence.conditional_sequence_is_active onchange=self.link.callback(move |_| Msg::ToggleSequenceActive(active_element_conditional_sequence_c.clone())) />
                                    </div>
                                </div>
                                
                                <div>
                                    <h4 class="uk-flex-left">{"Sequences:"}</h4>
                                    <div class="uk-flex-right">
                                        <PlusButton label={"Add Sequence"} eject=self.link.callback(move |_| Msg::CreateSequence(active_element_conditional_sequence_d.clone())) />
                                    </div>
                                </div>
                                
                                <hr class="uk-divider" />
                            </>
        });

            for sequence in conditional_sequence.sequences.iter() {
                let sequence_name = sequence.name.clone();
                let sequence_id=sequence.id.clone();
                let is_active = sequence.sequence_is_active;
                let weight=sequence.weight;
                let sequence_selected_style = if let ActiveElement::ConditionalSequence((condi_id, Some(seq_id)))=self.props.active_element {
                    if condi_id == conditional_sequence_id && seq_id == sequence_id {
                        "border: 2px solid blue;"
                    } else {
                        ""
                    }
                }  else {
                    ""
                };
                let active_element_sequence = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), Some(sequence_id.clone())));
                let active_element_sequence_b = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), Some(sequence_id.clone())));
                // notify_debug(format!("a e s b {:?}", active_element_sequence_b.clone()));
                let active_element_sequence_c = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), Some(sequence_id.clone())));
                
                nodes.push(html!{
                                <div style=sequence_selected_style onclick=self.link.callback(move |_| Msg::SetActiveElement(active_element_sequence.clone()))  >
                                    <h4 class="uk-flex-left">{sequence_name}</h4>
                                    <div class="uk-flex-right">
                                        <input uk-tooltip="title: Weight" class="uk-input" type="number" value=weight oninput=self.link.callback(move |i:InputData| Msg::UpdateWeight(i)) />
                                        <button uk-tooltip="title: Remove Sequence" class="uk-button" onclick=self.link.callback(move |_| Msg::RemoveSequence(active_element_sequence_b.clone()) ) >{"X"}</button>
                                        <MatSwitch checked=is_active onchange=self.link.callback(move |_| Msg::ToggleSequenceActive(active_element_sequence_c.clone())) />
                                    </div>
                                </div>
            })
            }
        }

        nodes
    }
}
