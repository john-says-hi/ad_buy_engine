use crate::appstate::app_state::AppState;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Click,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    pub eject: Callback<()>,
    pub label: String,
}

pub struct PlusButton {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for PlusButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.props.eject.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <button onclick=self.link.callback(|_| Msg::Click) class="uk-button uk-button-default uk-background-primary uk-light uk-flex-right"><span class="fas fa-plus uk-margin-small-right"></span>{format!("New {}",&self.props.label)}</button>
                        }
    }
}
