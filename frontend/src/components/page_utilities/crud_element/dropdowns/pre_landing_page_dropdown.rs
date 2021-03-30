use crate::appstate::app_state::{AppState, STATE};
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::offer::Offer;
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
    Select(LandingPage),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub eject: Callback<LandingPage>,
    pub selected: Option<LandingPage>,
}

pub struct PreLandingPageDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

impl Component for PreLandingPageDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(data) => self.props.eject.emit(data),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let mut options = VList::new();

        if let Some(selected_lander) = self.props.selected.clone() {
            let name = selected_lander.name.clone();
            options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(selected_lander.clone())) >{name}</option>})
        } else {
            options.push(html! {<option >{"Select Pre Landing Page"}</option>})
        }

        let plps = self.props.state.borrow().landing_pages.borrow().clone();

        for item in plps.iter().cloned().filter(|s| s.is_pre_landing_page) {
            let name = item.name.clone();
            options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{name}</option>});
        }

        html! {
        <select class="uk-select">
            {options}
        </select>
                        }
    }
}
