use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::dropdowns::custom_event_dropdown::CustomEventDropdown;
use crate::components::primitives::TextInput;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::{notify_danger, notify_primary, RootComponent};
use ad_buy_engine::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use ad_buy_engine::data::custom_events::{CustomConversionEvent, CustomConversionEventToken};
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
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
use yew::virtual_dom::VList;

use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    OnMouseUpSubmit,
    UpdateTokenName((usize, InputData)),
    OnSelect(CustomConversionEvent),
    Ignore,
    Remove(usize),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub events: Vec<CustomConversionEventToken>,
    pub callback: Callback<Vec<CustomConversionEventToken>>,
    pub state: STATE,
}

pub struct SelectCustomConversionEvents {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub event_token: String,
    pub events: Vec<CustomConversionEventToken>,
}

impl Component for SelectCustomConversionEvents {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let events = props.events.clone();

        Self {
            link,
            props,
            event_token: String::new(),
            events,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnMouseUpSubmit => false,
            Msg::OnSelect(data) => {
                self.events.push(CustomConversionEventToken {
                    event: data,
                    token: Some(String::new()),
                });
                true
            }
            Msg::Ignore => false,
            Msg::Remove(idx) => {
                self.events.remove(idx);
                self.props.callback.emit(self.events.clone());
                true
            }
            Msg::UpdateTokenName((idx, i)) => {
                if let Some(event) = self.events.get_mut(idx) {
                    event.token = Some(i.value);
                    self.props.callback.emit(self.events.clone());
                } else {
                    notify_danger("Internal Err: No Event found")
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.events = props.events.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut rows = VList::new();

        let table = if !self.events.is_empty() {
            for (idx, event) in self.events.iter().enumerate() {
                rows.push(html!{
                                   <tr>
                                      <td>{event.event.name.clone()}</td>
                                      <td>
                                         <TextInput value=event.token.clone().unwrap_or("".to_string()) oninput=self.link.callback(move |i:InputData| Msg::UpdateTokenName((idx, i)))  />
                                      </td>
                                      <td><button class="uk-button uk-button-small" onclick=self.link.callback(move |_| Msg::Remove(idx)) >{"X"}</button></td>
                                   </tr>
		        })
            }

            html! {
                            <table class="uk-table uk-table-small">
                                <thead>
                                    <tr>
                                        <th>{"Name"}</th>
                                        <th>{"Token"}</th>
                                        <th>{"Remove"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {rows}
                                </tbody>
                            </table>
            }
        } else {
            html! {}
        };

        html! {
        <div class="">

            <div>
                <h5>{"Select or Remove Custom Events and Type It's Token"}</h5>
                <CustomEventDropdown state=Rc::clone(&self.props.state) selected=self.events.iter().map(|s|s.event.clone()).collect::<Vec<CustomConversionEvent>>() callback=self.link.callback(Msg::OnSelect) />
            </div>

            <div class="uk-margin-small">
                {table}
            </div>

        </div>
                            }
    }
}
