use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::{notify_danger, notify_primary, RootComponent};
use ad_buy_engine::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::data::lists::DataURLToken;
use ad_buy_engine::ipnet::IpNet;
use std::cell::RefCell;
use std::net::IpAddr;
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;
use url::Url;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VList;

use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub number_of_ctas: u8,
    pub tracking_domain: Url,
    pub is_pre_sell: bool,
}

pub struct LandingPageClickURLGenerator {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

impl Component for LandingPageClickURLGenerator {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut nodes = VList::new();
        let tok_ident = if self.props.is_pre_sell {
            "learn"
        } else {
            "extra"
        };

        for cta in 0..self.props.number_of_ctas {
            let cta = cta + 1;
            let mut url = self.props.tracking_domain.clone();
            if cta == 1 {
                if self.props.number_of_ctas == 1 {
                    url.set_path(tok_ident);
                } else {
                    url.set_path(format!("{}/{}", tok_ident, cta).as_str());
                }
            } else {
                url.set_path(format!("{}/{}", tok_ident, cta).as_str());
            }
            nodes.push(
                html! {<div><input class="uk-input" value=url.to_string() type="text" /></div>},
            )
        }

        html! {
        <div class="uk-margin uk-margin-bottom-large">
            {label!("Click URLs")}
            {nodes}
        </div>
                            }
    }
}
