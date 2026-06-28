use crate::appstate::app_state::{AppState, STATE};
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::data::lists::time_zone::TimeZone;
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
    Select(TimeZone),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub selected: TimeZone,
    pub onselect: Callback<TimeZone>,
}

pub struct TimeZoneDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

impl Component for TimeZoneDropdown {
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

        options.push(html! {<option >{self.props.selected.to_string()}</option>});

        for tz in TimeZone::iter() {
            let name = tz.to_string();
            options.push(html! {<option onclick=self.link.callback(move |_|Msg::Select(tz)) >{name}</option>});
        }

        html! {
        <select class="uk-select">
            {options}
        </select>
                        }
    }
}
