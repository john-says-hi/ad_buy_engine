use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::{notify_danger, notify_primary, RootComponent};
use ad_buy_engine::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::ipnet::IpNet;
use std::cell::RefCell;
use std::net::IpAddr;
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VList;

use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    OnInput(InputData),
    OnEnter,
    Ignore,
    Remove(usize),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub tags: Vec<String>,
    pub eject: Callback<Vec<String>>,
}

pub struct TagsSelector {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub tags: Vec<String>,
    pub tag: String,
}

impl Component for TagsSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            tags: vec![],
            tag: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnEnter => {
                self.tags.push(self.tag.clone());
                self.props.eject.emit(self.tags.clone());
                self.tag.clear();
                false
            }
            Msg::OnInput(data) => {
                self.tag = data.value;
                true
            }
            Msg::Ignore => false,
            Msg::Remove(pos) => {
                self.tags.remove(pos);
                self.props.eject.emit(self.tags.clone());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.tags = props.tags.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut nodes = VList::new();

        for (idx, tag) in self.tags.iter().enumerate() {
            nodes.push(html! {<button style="border:2px solid blue;" onclick=self.link.callback(move|_|Msg::Remove(idx)) uk-tooltip="title:Click to Remove" class="uk-margin-small" type="button">{format!("{} X",tag.clone())}</button>})
        }

        html! {
        <div class="uk-margin uk-margin-bottom-large">

            {label!("Type a Tag and Press Enter to Save")}
            <div class="">
                {nodes}
            </div>

            <input class="uk-input" value=&self.tag type="text" placeholder="Type your tag" oninput=self.link.callback(Msg::OnInput) onkeypress=self.link.callback(|event:KeyboardEvent| {
            if event.key() == "Enter" {Msg::OnEnter}else{Msg::Ignore}
            }) />

        </div>
                            }
    }
}
