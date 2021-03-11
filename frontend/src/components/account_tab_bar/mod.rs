mod custom_conversions;
pub mod refererr_handling;

use crate::appstate::app_state::AppState;
use crate::utils::routes::AppRoute;
use custom_conversions::CustomConversionBtn;

use crate::components::account_tab_bar::refererr_handling::ReferrerHanldingBtn;
use crate::{notify_primary, RootComponent};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Element;
use yew::prelude::*;
use yew::virtual_dom::VNode;
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

pub struct AccountTabBar {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    props: Props,
}

impl Component for AccountTabBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));

        Self {
            link,
            router,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <>
                <MatTabBar >
                   <CustomConversionBtn state=Rc::clone(&self.props.state) />
                   <ReferrerHanldingBtn state=Rc::clone(&self.props.state) />
                </MatTabBar>
        </>

                                          }
    }
}
