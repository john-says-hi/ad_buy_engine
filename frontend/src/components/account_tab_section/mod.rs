pub mod custom_conversions;
pub mod referrer_handling;

use crate::appstate::app_state::AppState;
use crate::components::account_tab_section::custom_conversions::modal::ModalType;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::show_uk_modal;
use crate::utils::routes::AppRoute;
use crate::{notify_primary, notify_warning};
use ad_buy_engine::data::custom_events::CustomConversionEvent;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_material::list::GraphicType;
use yew_material::{MatListItem, MatMenu, MatSelect, MatTab, MatTabBar};
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;

pub enum Msg {
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct AccountTabDefault {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for AccountTabDefault {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <div class="uk-margin">
        </div>
                }
    }
}
