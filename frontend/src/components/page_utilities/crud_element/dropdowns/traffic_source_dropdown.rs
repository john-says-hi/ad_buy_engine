use crate::appstate::app_state::{AppState, STATE};
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use ad_buy_engine::data::lists::country::Country as ISOCountry;
use ad_buy_engine::data::lists::Currency;
use ad_buy_engine::Country;
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
    Select(TrafficSource),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub onselect: Callback<TrafficSource>,
}

pub struct TrafficSourceDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

impl Component for TrafficSourceDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(data) => self.props.onselect.emit(data),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut options = VList::new();
        let traffic_sources = self.props.state.borrow().traffic_sources.borrow().clone();

        for item in traffic_sources.iter().cloned() {
            let name = item.name.to_string();
            options.push(html! {<option onclick=self.link.callback(move |_|Msg::Select(item.clone())) >{name}</option>});
        }

        html! {
        <div>
            {label!("Traffic Source")}
            <select class="uk-select">
                {options}
            </select>
        </div>
                        }
    }
}
