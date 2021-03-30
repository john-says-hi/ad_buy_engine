use crate::appstate::app_state::{AppState, STATE};
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::elements::crud::CreatableElement;
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
    Select(Offer),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub eject: Callback<Offer>,
    pub selected: Option<Offer>,
}

pub struct OfferDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

impl Component for OfferDropdown {
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

        if let Some(selected_offer) = self.props.selected.clone() {
            // let name = selected_offer.name.clone();
            // options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(selected_offer.clone())) >{name}</option>})
        } else {
            options.push(html! {<option >{"Select Offer"}</option>})
        }

        let offers = self.props.state.borrow().offers.borrow().clone();

        for (idx, item) in offers.iter().cloned().enumerate() {
            let name = item.name.clone();

            options.push(html!{<option value=idx onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{name}</option>});
        }

        html! {
        <select class="uk-select" id="bFhTRvF"  >
            {options}
        </select>
                        }
    }
}
