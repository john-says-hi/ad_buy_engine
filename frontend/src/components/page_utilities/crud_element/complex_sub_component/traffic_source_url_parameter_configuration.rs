use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::{notify_danger, notify_primary, RootComponent};
use ad_buy_engine::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::{
    CostParameter, CustomParameter, ExternalIDParameter,
};
use ad_buy_engine::ipnet::IpNet;
use std::cell::RefCell;
use std::net::IpAddr;
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_material::{MatTextField, MatSwitch};
use yew_material::{MatListItem, MatSelect};
use yew_services::storage::Area;
use yew_services::StorageService;
use crate::components::primitives::TextInput;

pub enum Msg {
    UpdateCostParameter(InputData),
    UpdateCostPlaceholder(InputData),
    UpdateExternalIDParameter(InputData),
    UpdateExternalIDPlaceholder(InputData),
    UpdateCustomVariableName((usize,InputData)),
    UpdateCustomVariableParameter((usize,InputData)),
    UpdateCustomVariablePlaceholder((usize,InputData)),
    ToggleCustomVariableIsActive(usize),
    Ignore,
    RemoveCustomVariable(usize),
    AddCustomVariable,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub external_id: ExternalIDParameter,
    pub cost: CostParameter,
    pub custom: Vec<CustomParameter>,
    pub eject_external_id: Callback<ExternalIDParameter>,
    pub eject_cost: Callback<CostParameter>,
    pub eject_custom: Callback<Vec<CustomParameter>>,
}

pub struct TrafficSourceUrlParameterConfig {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub external_id: ExternalIDParameter,
    pub cost: CostParameter,
    pub custom: Vec<CustomParameter>,
}

impl Component for TrafficSourceUrlParameterConfig {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            external_id: ExternalIDParameter {
                parameter: "".to_string(),
                placeholder: "".to_string(),
            },
            cost: CostParameter {
                parameter: "".to_string(),
                placeholder: "".to_string(),
            },
            custom: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddCustomVariable=>{
                if self.custom.len()>=10 {
                    notify_danger("Max 10 allowed")
                } else{
                    self.custom.push(CustomParameter{
                        name: "".to_string(),
                        parameter: "".to_string(),
                        placeholder: "".to_string(),
                        is_tracked: false
                    });
                    self.props.eject_custom.emit(self.custom.clone())
                }

            }
            Msg::UpdateCostParameter(i)=>{
                self.cost.parameter=i.value;
                self.props.eject_cost.emit(self.cost.clone());
            }
            Msg::UpdateCostPlaceholder(i)=>{
                self.cost.placeholder=i.value;
                self.props.eject_cost.emit(self.cost.clone());
            }
            Msg::UpdateExternalIDParameter(i)=>{
                self.external_id.parameter=i.value;
                self.props.eject_external_id.emit(self.external_id.clone());
            }
            Msg::UpdateExternalIDPlaceholder(i)=>{
                self.external_id.placeholder=i.value;
                self.props.eject_external_id.emit(self.external_id.clone());
            }
            Msg::UpdateCustomVariableName((usize,i))=>{
                self.custom.get_mut(usize).map(|s| s.name=i.value);
                self.props.eject_custom.emit(self.custom.clone());
            }
            Msg::UpdateCustomVariableParameter((usize,i))=>{
                self.custom.get_mut(usize).map(|s| s.parameter=i.value);
                self.props.eject_custom.emit(self.custom.clone());
            }
            
            Msg::UpdateCustomVariablePlaceholder((usize,i))=>{
                self.custom.get_mut(usize).map(|s| s.placeholder=i.value);
                self.props.eject_custom.emit(self.custom.clone());
            }
            
            
            Msg::ToggleCustomVariableIsActive(usize)=>{
                self.custom.get_mut(usize).map(|s|s.is_tracked= !s.is_tracked);
                self.props.eject_custom.emit(self.custom.clone());
            
            }
            
            Msg::Ignore => {},
            Msg::RemoveCustomVariable(pos) => {
                self.custom.remove(pos);
                self.props.eject_custom.emit(self.custom.clone());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.cost = props.cost.clone();
        self.custom = props.custom.clone();
        self.external_id = props.external_id.clone();

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut nodes = VList::new();
        
        html! {
        <div class="uk-margin">
        
                          <h4>{"Traffic Source Parameters"}</h4>
                            <table class="uk-table uk-table-small">
                            
                                <thead>
                                    <tr>
                                        <th>{"Name"}</th>
                                        <th>{"Parameter"}</th>
                                        <th>{"Token"}</th>
                                        <th>{"Active"}</th>
                                    </tr>
                                </thead>
                                
                                <tbody>
                                
                                   <tr>
                                      <td>{"External ID"}</td>
                                      <td>
                                         <TextInput value=&self.external_id.parameter oninput=self.link.callback(|i:InputData|Msg::UpdateExternalIDParameter(i))  />
                                      </td>
                                     <td>
                                         <TextInput value=&self.external_id.placeholder oninput=self.link.callback(|i:InputData|Msg::UpdateExternalIDPlaceholder(i))  />
                                      </td>
                                      
                                      <td>
                                      </td>
                                   </tr>
                                   
                                   <tr>
                                      <td>{"Cost"}</td>
                                      <td>
                                         <TextInput value=&self.cost.parameter oninput=self.link.callback(|i:InputData|Msg::UpdateCostParameter(i))  />
                                      </td>
                                     <td>
                                         <TextInput value=&self.cost.placeholder oninput=self.link.callback(|i:InputData|Msg::UpdateCostPlaceholder(i))  />
                                      </td>
                                      
                                      <td>
                                      </td>
                                   </tr>
                                   
                                   <tr>
                                        <button onclick=self.link.callback(|_|Msg::AddCustomVariable) class="uk-button uk-background-primary">{"+ Add Custom Variable"}</button>
                                   </tr>
                                   
                                   {self.gen_custom_vars()}
                                   
                                </tbody>
                                
                            </table>

        </div>
                            }
    }
}

impl TrafficSourceUrlParameterConfig {
    fn gen_custom_vars(&self)->VList {
        let mut nodes = VList::new();
        
        for (idx,custom_variable ) in self.custom.iter().enumerate() {
            
            nodes.push(html!{
                                   <tr>
                                      <td>
                                         <TextInput value=&custom_variable.name oninput=self.link.callback(move |i:InputData| Msg::UpdateCustomVariableName((idx,i )))  />
                                      </td>
                                      <td>
                                         <TextInput value=&custom_variable.parameter oninput=self.link.callback(move |i:InputData|Msg::UpdateCustomVariableParameter((idx, i)))  />
                                      </td>
                                      <td>
                                         <TextInput value=&custom_variable.placeholder oninput=self.link.callback(move |i:InputData|Msg::UpdateCustomVariablePlaceholder((idx, i)))  />
                                      </td>
                                      
                                      <td>
                                            <div class="uk-margin">
                                               <MatSwitch checked=custom_variable.is_tracked onchange=self.link.callback(move |_|Msg::ToggleCustomVariableIsActive(idx)) />
                                            </div>
                                      </td>
                                      
                                   </tr>
            })
        }
        
        nodes
    }
}