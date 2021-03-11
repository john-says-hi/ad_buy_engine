use crate::appstate::app_state::AppState;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::lists::DataURLToken;
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VList;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    OnSelect(DataURLToken),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub selected: Vec<DataURLToken>,
    pub eject: Callback<Vec<DataURLToken>>,
}

pub struct LanderURLTokenSelector {
    link: ComponentLink<Self>,
    props: Props,
    selected: Vec<DataURLToken>,
}

impl Component for LanderURLTokenSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            selected: props.selected.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnSelect(token) => {
                if let Some(duplicate_token) = self.selected.iter().position(|s| s == &token) {
                    self.selected.remove(duplicate_token);
                    self.props.eject.emit(self.selected.clone());
                } else {
                    self.selected.push(token);
                    self.props.eject.emit(self.selected.clone());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.selected = props.selected.clone();
        true
    }

    fn view(&self) -> Html {
        let selected_style = "border:2px solid blue;".to_string();
        let mut tokens_as_button_nodes = VList::new();

        for token in DataURLToken::iter().filter(|s| {
            s != &DataURLToken::Parameter1
                && s != &DataURLToken::Parameter2
                && s != &DataURLToken::Parameter3
                && s != &DataURLToken::Parameter4
                && s != &DataURLToken::Parameter5
                && s != &DataURLToken::TransactionID
                && s != &DataURLToken::TimeOfPostback
                && s != &DataURLToken::ConversionCost
                && s != &DataURLToken::CustomEvent
                && s != &DataURLToken::PayoutCurrency
                && s != &DataURLToken::Payout
                && s != &DataURLToken::ClickID
                && s != &DataURLToken::FunnelID
                && s != &DataURLToken::Cost
        }) {
            if let Some(pos) = self.selected.iter().position(|s| s == &token) {
                tokens_as_button_nodes.push(html!{<button style=selected_style class="uk-button uk-button-small uk-margin-small" onclick=self.link.callback(move |_| Msg::OnSelect(token.clone()))  >{token.to_string()}</button>})
            } else {
                tokens_as_button_nodes.push(html!{<button class="uk-button uk-button-small uk-margin-small" onclick=self.link.callback(move |_| Msg::OnSelect(token.clone()))  >{token.to_string()}</button>})
            }
        }

        html! {
        <div class="uk-margin">
            <h4>{"Lander URL Tokens"}</h4>
            {tokens_as_button_nodes}
        </div>
                }
    }
}
