use crate::appstate::app_state::{AppState, STATE};
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::custom_events::CustomConversionEvent;
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::data::lists::Currency;
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;
use url::Url;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VList;

use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Select(CustomConversionEvent),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<CustomConversionEvent>,
    #[prop_or_default]
    pub selected: Vec<CustomConversionEvent>,
    pub state: STATE,
}

pub struct CustomEventDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub events: Vec<CustomConversionEvent>,
}

impl Component for CustomEventDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let events = props
            .state
            .borrow()
            .account
            .borrow()
            .custom_conversions
            .clone();

        Self {
            link,
            props,
            events,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(data) => self.props.callback.emit(data),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut options = VList::new();
        options.push(html! {<option >{"Select an Event"}</option>});
        let selected = self.props.selected.clone();

        if !self.props.selected.is_empty() {
            for item in self
                .events
                .iter()
                .filter(|s| {
                    for item in selected.iter() {
                        if &item == s {
                            return false;
                        }
                    }
                    true
                })
                .cloned()
            {
                let name = item.name.clone();
                options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{name}</option>});
            }
        } else {
            for item in self.events.iter().cloned() {
                let name = item.name.clone();
                options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{name}</option>});
            }
        }

        html! {
        <div class="uk-margin">
            <select class="uk-select">
                {options}
            </select>
        </div>
                            }
    }
}
