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
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <div class="uk-margin uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1  uk-grid-row-collapse" uk-grid="">
                <nav class="uk-margin-top" uk-navbar="">
                    <div class="uk-navbar-left">
                        <ul class="uk-navbar-nav uk-flex-wrap uk-flex-center">

                           <CustomConversionBtn state=Rc::clone(&self.props.state) />
                           <ReferrerHanldingBtn state=Rc::clone(&self.props.state) />

                        </ul>
                    </div>
                </nav>
        </div>
                                                  }
    }
}
