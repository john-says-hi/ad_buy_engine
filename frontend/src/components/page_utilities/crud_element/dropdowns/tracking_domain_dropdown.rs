use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use std::cell::RefCell;
use std::rc::Rc;
use url::Url;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_material::MatTextField;
use yew_material::{MatListItem, MatSelect};
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Select(Url),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<Url>,
    pub state: Rc<RefCell<AppState>>,
}

pub struct TrackingDomainDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub available_domains: Vec<Url>,
}

impl Component for TrackingDomainDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let available_domains = props.state.borrow().return_all_tracking_urls_no_filter();

        Self {
            link,
            props,
            available_domains,
        }
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
        for item in self.available_domains.iter() {
            let url = item.clone();
            let str_url = url.clone();
            options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(url.clone())) >{str_url}</option>});
        }

        html! {
        <div class="uk-margin">
            {label!("Tracking Domain:")}
            <select class="uk-select">
                {options}
            </select>
        </div>
                            }
    }
}
