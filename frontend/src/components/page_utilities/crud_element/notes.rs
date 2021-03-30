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

pub enum Msg {
    OnInput(InputData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<InputData>,
    pub value: String,
}

pub struct NotesComponent {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub value: String,
}

impl Component for NotesComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = props.value.clone();
        Self { link, props, value }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnInput(data) => self.props.callback.emit(data),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        // <div class="uk-margin">
           <TextArea oninput=self.link.callback(|i:InputData|Msg::OnInput(i)) rows="4" value=&self.value />
        // </div>
                            }
    }
}
