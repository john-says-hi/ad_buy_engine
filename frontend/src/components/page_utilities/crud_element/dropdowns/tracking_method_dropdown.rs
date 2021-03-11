use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::conversion::ConversionTrackingMethod;
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VList;
use yew_material::MatTextField;
use yew_material::{MatListItem, MatSelect};
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Select(ConversionTrackingMethod),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or(None)]
    pub selected: Option<ConversionTrackingMethod>,
    pub callback: Callback<ConversionTrackingMethod>,
}

pub struct TrackingMethodDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

impl Component for TrackingMethodDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(data) => self.props.callback.emit(data),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let mut options = VList::new();

        if let Some(option) = self.props.selected {
            options.push(html! {<option onclick=self.link.callback(move |_| Msg::Select(option.clone()))>{option.to_string()}</option>});
        } else {
            options.push(html! {<option>{"Select Tracking Method"}</option>});
        }

        for item in ConversionTrackingMethod::iter().filter(|s| Some(*s) != self.props.selected) {
            options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{item.to_string()}</option>});
        }

        html! {
        <div class="uk-margin">
            <h4>{"Conversion Tracking Method:"}</h4>
            <select class="uk-select">
                {options}
            </select>
        </div>
                            }
    }
}
