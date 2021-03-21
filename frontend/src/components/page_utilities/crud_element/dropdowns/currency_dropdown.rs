use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
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
use yew_material::MatTextField;
use yew_material::{MatListItem, MatSelect};
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Select(Currency),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<Currency>,
    #[prop_or_default]
    pub label: String,
}

pub struct CurrencyDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

impl Component for CurrencyDropdown {
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
        for item in Currency::iter() {
            options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{item.to_string()}</option>});
        }

        let label = if self.props.label.is_empty() {
            html! {}
        } else {
            label!(&self.props.label)
        };

        html! {
        <div class="">
            {label}
            <select class="uk-select">
                {options}
            </select>
        </div>
                            }
    }
}
