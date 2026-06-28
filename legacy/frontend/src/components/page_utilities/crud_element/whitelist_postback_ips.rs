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

pub enum Remove {
    Ip(usize),
    IpNet(usize),
}

pub enum Msg {
    OnInput(InputData),
    OnEnter,
    Ignore,
    Remove(Remove),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub whitelisted_postback_ips: WhiteListedPostbackIPs,
    pub callback: Callback<WhiteListedPostbackIPs>,
}

pub struct WhitelistPostbackIPsComponent {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub ip: String,
    pub whitelisted_postback_ips: WhiteListedPostbackIPs,
    pub is_err: bool,
}

impl Component for WhitelistPostbackIPsComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let whitelisted_postback_ips = props.whitelisted_postback_ips.clone();

        Self {
            link,
            props,
            ip: String::new(),
            is_err: false,
            whitelisted_postback_ips,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnEnter => {
                // if let Ok(ip) = IpAddr::from_str(&self.ip.trim()) {
                if let Ok(ip_net) = IpNet::from_str(&self.ip.trim()) {
                    self.whitelisted_postback_ips.ip_nets.push(ip_net);
                    self.ip.clear();
                    self.props
                        .callback
                        .emit(self.whitelisted_postback_ips.clone());
                    self.is_err = false;
                } else if let Ok(ip) = IpAddr::from_str(&self.ip.trim()) {
                    self.whitelisted_postback_ips.ips.push(ip);
                    self.ip.clear();
                    self.props
                        .callback
                        .emit(self.whitelisted_postback_ips.clone());
                    self.is_err = false;
                } else {
                    self.is_err = true;
                    notify_danger("IpAddr Parse Failed; Please enter a valid IP.");
                }
                true
            }
            Msg::OnInput(data) => {
                self.is_err = false;
                self.ip = data.value;
                true
            }
            Msg::Ignore => false,
            Msg::Remove(remove) => {
                match remove {
                    Remove::Ip(pos) => {
                        self.whitelisted_postback_ips.ips.remove(pos);
                        self.props
                            .callback
                            .emit(self.whitelisted_postback_ips.clone());
                    }
                    Remove::IpNet(pos) => {
                        self.whitelisted_postback_ips.ip_nets.remove(pos);
                        self.props
                            .callback
                            .emit(self.whitelisted_postback_ips.clone());
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.whitelisted_postback_ips = props.whitelisted_postback_ips.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let is_err = if self.is_err {
            "border:2px solid red;"
        } else {
            ""
        };
        let mut nodes = VList::new();

        for (idx, ip) in self.props.whitelisted_postback_ips.ips.iter().enumerate() {
            nodes.push(html! {<button onclick=self.link.callback(move|_|Msg::Remove(Remove::Ip(idx))) uk-tooltip="title:Click to Remove" class="uk-margin-small-right" type="button">{format!("X: {}",ip.to_string())}</button>})
        }

        for (idx, ip_net) in self
            .props
            .whitelisted_postback_ips
            .ip_nets
            .iter()
            .enumerate()
        {
            nodes.push(html! {<button onclick=self.link.callback(move|_|Msg::Remove(Remove::IpNet(idx))) uk-tooltip="title:Click to Remove" class="uk-margin-small-right" type="button">{format!("X: {}",ip_net.to_string())}</button>})
        }

        html! {
        <div class="uk-margin uk-margin-bottom-large">
            <h4>{"Type an IP or IP Range (CIDR) then Press Enter"}</h4>
            <input class="uk-input" value=&self.ip style=is_err type="text" placeholder="Type & Press Enter to Add IP" oninput=self.link.callback(Msg::OnInput) onkeypress=self.link.callback(|event:KeyboardEvent| {
            if event.key() == "Enter" {Msg::OnEnter}else{Msg::Ignore}
            }) />
            <div class="uk-margin-small">
                {nodes}
            </div>
        </div>
                            }
    }
}
