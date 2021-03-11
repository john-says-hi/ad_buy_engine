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
use yew_material::MatTextField;
use yew_material::{MatListItem, MatSelect};
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    OnInput(InputData),
    OnBlur,
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub url_tokens: Vec<DataURLToken>,
    #[prop_or_default]
    pub offer_url: Option<Url>,
    pub eject: Callback<Url>,
}

pub struct LanderUrlGenerator {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub url_string: String,
    pub offer_url: Option<Url>,
}

impl Component for LanderUrlGenerator {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            url_string: "".to_string(),
            offer_url: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnBlur => {
                if let Ok(mut url) = Url::parse(&self.url_string) {
                    if let Some(query) = url.query() {
                        self.props.eject.emit(url);
                    } else {
                        url.set_query(Some(""));
                        self.props.eject.emit(url);
                    }
                } else {
                    notify_danger("Please Enter a Valid URL")
                }
                false
            }
            Msg::OnInput(data) => {
                self.url_string = data.value;
                true
            }
            Msg::Ignore => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(url) = &props.offer_url {
            self.url_string = url.to_string();
            self.offer_url = Some(url.clone());
        } else {
            self.offer_url = None;
        }

        if self.props.url_tokens.len() != props.url_tokens.len() {
            let more_difference = props
                .url_tokens
                .iter()
                .find(|s| !self.props.url_tokens.contains(s));

            let less_difference = self
                .props
                .url_tokens
                .iter()
                .find(|s| !props.url_tokens.contains(s));

            if props.url_tokens.len() > self.props.url_tokens.len() {
                if let Some(token) = more_difference {
                    self.url_string.push_str(token.to_string().as_str());
                    if let Ok(url) = Url::parse(&self.url_string) {
                        self.props.eject.emit(url);
                    }
                }
            } else {
                if let Some(token) = less_difference {
                    let token_to_replace = token.to_string();
                    self.url_string = self.url_string.replace(&token_to_replace, "");
                    if let Ok(url) = Url::parse(&self.url_string) {
                        self.props.eject.emit(url);
                    }
                }
            }
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
                <div class="uk-margin uk-margin-bottom-large">

                    <h3>{"Lander URL"}</h3>

                    {
          html!{<input class="uk-input" value=&self.url_string type="text" placeholder="i.e. https://xyz/com" oninput=self.link.callback(Msg::OnInput) onblur=self.link.callback(|_| Msg::OnBlur) />}
        }


                </div>
                                    }
    }
}
