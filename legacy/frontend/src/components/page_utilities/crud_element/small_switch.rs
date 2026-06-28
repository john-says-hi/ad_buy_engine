use crate::appstate::app_state::AppState;
use crate::components::primitives::text_area::TextArea;
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
use yew::virtual_dom::VList;

use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Switch {
    Yes,
    No,
}

pub enum Msg {
    OnClick(Switch),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub onchange: Callback<()>,
    pub checked: bool,
}

pub struct SmallSwitch {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub checked: bool,
}

impl Component for SmallSwitch {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let checked = props.checked.clone();
        Self {
            link,
            props,
            checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnClick(data) => match data {
                Switch::No => self.props.onchange.emit(()),
                Switch::Yes => self.props.onchange.emit(()),
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.checked = props.checked;
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let btn = if !self.checked {
            html! {
                <button class="uk-button uk-button-small uk-button-danger" onclick=callback!(self, |_| Msg::OnClick(Switch::No))>{"No"}</button>
            }
        } else {
            html! {
                <button class="uk-button uk-button-small uk-button-success" onclick=callback!(self, |_| Msg::OnClick(Switch::Yes))>{"Yes"}</button>
            }
        };

        html! {
        <div>
        {btn}
        </div>
                            }
    }
}
