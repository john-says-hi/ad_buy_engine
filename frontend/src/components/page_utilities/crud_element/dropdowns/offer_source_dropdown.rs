use crate::appstate::app_state::{AppState, STATE};
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::offer_source::OfferSource;
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
    Select(OfferSource),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    #[prop_or_default]
    pub selected: Option<OfferSource>,
    pub eject: Callback<OfferSource>,
    #[prop_or_default]
    pub label: String,
}

pub struct OfferSourceDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub available_offer_sources: Vec<OfferSource>,
}

impl Component for OfferSourceDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let available_offer_sources = props.state.borrow().offer_sources.borrow().clone();

        Self {
            link,
            props,
            available_offer_sources,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(data) => self.props.eject.emit(data),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut options = VList::new();

        if let Some(item) = self.props.selected.clone() {
            let name = item.name.clone();
            options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{name}</option>});
        } else {
            options.push(html! {<option >{"Select an Offer Source"}</option>});
        }

        for item in self.available_offer_sources.iter().cloned() {
            let name = item.name.clone();
            if let Some(selected_item) = &self.props.selected {
                if selected_item.offer_source_id != item.offer_source_id {
                    options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{name}</option>});
                }
            } else {
                options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(item.clone())) >{name}</option>});
            }
        }

        let label = if self.props.label.is_empty() {
            html! {}
        } else {
            label!(&self.props.label)
        };

        html! {
        <div class="uk-margin" uk-tooltip="title:This is used to track performance">
            {label}
            <select class="uk-select">
                {options}
            </select>
        </div>
                            }
    }
}
