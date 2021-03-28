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
use ad_buy_engine::constant::{COLOR_BLUE, COLOR_GRAY};
use crate::components::page_utilities::crud_element::toggle_switch::ToggleSwitch;

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

pub struct LHSConditionalSequence {
    link: ComponentLink<Self>,
    props: Props,
    weight:String,
}

impl Component for LHSConditionalSequence {
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
        let condi_seq_len=self.props.conditional_sequences.is_empty();
    
        html! {
        <>
                                <div>
                                    <div>{label!("Conditional Sequences")}</div>
                                    <div class="uk-margin"><button onclick=self.link.callback(|_| Msg::CreateConditionalSequence) class="uk-button uk-button-small uk-button-primary"><span class="fas fa-plus uk-margin-small-right"></span>{" New Conditional Sequence"}</button></div>
                                </div>
                                
                                {if condi_seq_len {html!{}}else{html!{<div class="uk-margin-top-small">{divider!(2)}</div>}}}
                                
                                {self.render_conditional_sequences()}
        </>
        }
    }
}

impl LHSConditionalSequence {
    pub fn render_conditional_sequences(&self) -> VList {
        let mut nodes = VList::new();

        for conditional_sequence in self.props.conditional_sequences.iter() {
            let conditional_sequence_name = conditional_sequence.name.clone();
            let conditional_sequence_id = conditional_sequence.id.clone();
            let is_active = conditional_sequence.conditional_sequence_is_active;
            let conditional_sequence_selected_style = if let ActiveElement::ConditionalSequence((id, None))=self.props.active_element {
                if id == conditional_sequence_id {
                    border!(COLOR_BLUE)
                } else {
                    border!(COLOR_GRAY)
                }
            }  else {
                border!(COLOR_GRAY)
            };
            let active_element_conditional_sequence = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), None));
            let num_of_conditions = conditional_sequence.condition_set.len();
    
            nodes.push(html! {
            
                            <>
                                <div class="uk-margin-small" style=conditional_sequence_selected_style onclick={
                                let ae_c_s = active_element_conditional_sequence.clone();
                                self.link.callback(move |_| Msg::SetActiveElement(ae_c_s.clone()))
                                } >

                                    {label!("o", conditional_sequence_name)}
                                    <div class="uk-child-width-1-3" uk-grid="">
                                    
                                        <div>{label!("s", "Conditions")}<div>{num_of_conditions.to_string()}</div></div>
                                        
                                        <div>{label!("s", "Remove")}<button uk-tooltip="title: Remove Sequence" class="uk-button" onclick={
                                        let ae_c_s =active_element_conditional_sequence.clone();
                                        self.link.callback(move |_| Msg::RemoveSequence(ae_c_s.clone()))
                                        } >{"X"}</button></div>
                                        
                                        <div><ToggleSwitch label="Active" size=Some("small".to_string()) checked=is_active onchange={
                                        let ae_c_s =active_element_conditional_sequence.clone();
                                        self.link.callback(move |_| Msg::ToggleSequenceActive(ae_c_s.clone()))
                                        } /></div>
                                        
                                    </div>
                                </div>
                                
                                <div class="uk-margin-small">{divider!()}</div>
                                
                                <div>
                                    {label!("Sequences:")}
                                    <div class="uk-margin-small">
                                        <button onclick={
                                        let ae_c_s = active_element_conditional_sequence.clone();
                                        self.link.callback(move |_| Msg::CreateSequence(ae_c_s.clone()))
                                        } class="uk-button uk-button-small uk-button-success"><span class="fas fa-plus uk-margin-small-right"></span>{" Add Sequence"}</button>
                                    </div>
                                </div>
                            </>
        });

            for sequence in conditional_sequence.sequences.iter() {
                let sequence_name = sequence.name.clone();
                let sequence_id=sequence.id.clone();
                let is_active = sequence.sequence_is_active;
                let weight=sequence.weight;
                let sequence_selected_style = if let ActiveElement::ConditionalSequence((condi_id, Some(seq_id)))=self.props.active_element {
                    if condi_id == conditional_sequence_id && seq_id == sequence_id {
                        border!(COLOR_BLUE)
                    } else {
                        border!(COLOR_GRAY)
                    }
                }  else {
                    border!(COLOR_GRAY)
                };
                let active_element_sequence = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), Some(sequence_id.clone())));
                // let active_element_sequence_b = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), Some(sequence_id.clone())));
                // notify_debug(format!("a e s b {:?}", active_element_sequence_b.clone()));
                // let active_element_sequence_c = ActiveElement::ConditionalSequence((conditional_sequence_id.clone(), Some(sequence_id.clone())));
                
                nodes.push(html!{
                                <div class="uk-margin-small" style=sequence_selected_style onclick={
                               let ae_s = active_element_sequence.clone();
                                self.link.callback(move |_| Msg::SetActiveElement(ae_s.clone()))
                                }  >
                                    {label!("g", sequence_name)}
                                    
                                    <div class="uk-child-width-1-3" uk-grid="">
                                        <div>{label!("Weight")}<input uk-tooltip="title: Weight" class="uk-input" type="number" value=weight oninput={
                                        self.link.callback(move |i:InputData| Msg::UpdateWeight(i))
                                        } /></div>
                                        
                                        <div>{label!("Remove")}<button uk-tooltip="title: Remove Sequence" class="uk-button" onclick={
                                        let ae_s = active_element_sequence.clone();
                                        self.link.callback(move |_| Msg::RemoveSequence(ae_s.clone()) )
                                        } >{"X"}</button></div>
                                        
                                        <div><ToggleSwitch label="Active" size=Some("small".to_string()) checked=is_active onchange={
                                        let ae_s = active_element_sequence.clone();
                                        self.link.callback(move |_| Msg::ToggleSequenceActive(ae_s.clone()))
                                        } /></div>
                                    </div>
                                </div>
                                // <div style=sequence_selected_style onclick=self.link.callback(move |_| Msg::SetActiveElement(active_element_sequence.clone()))  >
                                //     <h4 class="uk-flex-left">{sequence_name}</h4>
                                //     <div class="uk-flex-right">
                                //         <input uk-tooltip="title: Weight" class="uk-input" type="number" value=weight oninput=self.link.callback(move |i:InputData| Msg::UpdateWeight(i)) />
                                //         <button uk-tooltip="title: Remove Sequence" class="uk-button" onclick=self.link.callback(move |_| Msg::RemoveSequence(active_element_sequence_b.clone()) ) >{"X"}</button>
                                //         <MatSwitch checked=is_active onchange=self.link.callback(move |_| Msg::ToggleSequenceActive(active_element_sequence_c.clone())) />
                                //     </div>
                                // </div>
            })
            }
        }
        nodes.push(html!{
            <div class="uk-margin-top-small">{divider!(2)}</div>
        });
        
        nodes
    }
}
