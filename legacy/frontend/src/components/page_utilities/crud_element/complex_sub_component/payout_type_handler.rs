use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::{notify_danger, notify_primary, RootComponent};
use ad_buy_engine::data::conversion::{
    ConversionTrackingMethod, PayoutType, WhiteListedPostbackIPs,
};
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::data::lists::{Currency, DataURLToken};
use ad_buy_engine::ipnet::IpNet;
use rust_decimal::Decimal;
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
    ParsePayoutValue(InputData),
    UpdatePayoutType(PayoutType),
    Ignore,
    UpdateCurrency(Currency),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub payout_type: PayoutType,
    #[prop_or_default]
    pub payout_value: Decimal,
    #[prop_or_default]
    pub payout_currency: Currency,
    pub eject_payout_type: Callback<PayoutType>,
    pub eject_payout_value: Callback<Decimal>,
    pub eject_payout_currency: Callback<Currency>,
}

pub struct PayoutTypeHandler {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub payout_value: Decimal,
}

impl Component for PayoutTypeHandler {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let payout_value = props.payout_value.clone();

        Self {
            link,
            props,
            payout_value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateCurrency(cur) => {
                self.props.eject_payout_currency.emit(cur);
                false
            }
            Msg::ParsePayoutValue(i) => {
                if let Ok(payout) = i.value.parse::<Decimal>() {
                    self.props.eject_payout_value.emit(payout);
                } else {
                    notify_danger("Please Enter Only Numbers")
                }
                false
            }
            Msg::UpdatePayoutType(payout_type) => {
                self.props.eject_payout_type.emit(payout_type);
                false
            }
            Msg::Ignore => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.payout_value = props.payout_value.clone();

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut payout_type_options = VList::new();
        let mut currency_options = VList::new();

        for pt in PayoutType::iter() {
            payout_type_options.push(html!{<option onclick=self.link.callback(move |_| Msg::UpdatePayoutType(pt.clone())) >{pt.to_string()}</option>})
        }

        for cur in Currency::iter() {
            currency_options.push(html!{<option onclick=self.link.callback(move |_| Msg::UpdateCurrency(cur.clone())) >{cur.to_string()}</option>})
        }

        html! {
        <>
                    <div class="uk-margin">
                        {label!("Payout Type")}
                        <select class="uk-select">
                            {payout_type_options}
                        </select>
                    </div>

                    <div class="uk-margin">
                        {label!("Default Payout Value")}
                        <input class="uk-input" value=&self.payout_value.to_string() type="text" placeholder="0" oninput=self.link.callback(Msg::ParsePayoutValue) />
                    </div>

                    <div class="uk-margin">
                        {label!("Currency")}
                        <select class="uk-select">
                            {currency_options}
                        </select>
                    </div>

                </>
                                    }
    }
}
