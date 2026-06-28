use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::routes::route_helpers::app_route_matches;
use crate::utils::routes::AppRoute;
use crate::{notify_primary, RootComponent};
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;

pub enum Msg {
    OnSelect(Browser),
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct BrowserDrop {
    link: ComponentLink<Self>,
    props: Props,
    router: Box<dyn Bridge<RouteAgent>>,
    active: bool,
}

impl Component for BrowserDrop {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));
        let active_route = state_clone!(props.state).borrow().return_app_route();
        let active = dropdown_is_active!(AppRoute::Browser AppRoute::BrowserVersion, active_route);

        Self {
            link,
            props,
            router,
            active,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
            Msg::OnSelect(brow) => match brow {
                Browser::ByType => {
                    self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::Browser,
                            );

                    self.router.send(ChangeRoute(AppRoute::Browser.into()));
                }

                Browser::Version => {
                    self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::BrowserVersion,
                            );
                    self.router
                        .send(ChangeRoute(AppRoute::BrowserVersion.into()))
                }
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let active_route = state_clone!(props.state).borrow().return_app_route();
        self.active = dropdown_is_active!(AppRoute::Browser AppRoute::BrowserVersion, active_route);
        true
    }

    fn view(&self) -> Html {
        let active_route = self.props.state.borrow().return_app_route();

        html! {
                <li class={if self.active {"uk-active"}else { "" }}>
                    <span class={if self.active {"active-tab fa fa-chrome uk-display-block uk-text-center"}else { "fa fa-chrome uk-display-block uk-text-center"} } onclick=callback!(self, |_| Msg::OnSelect(Browser::ByType))></span>
                    <a class={if self.active {"active-tab uk-display-block"}else { "uk-display-block"} } onclick=callback!(self, |_| Msg::OnSelect(Browser::ByType))>{"Browsers"}</a>

                    <div class="uk-navbar-dropdown"  uk-drop="pos: bottom-center;" >
                        <ul class="uk-nav uk-navbar-dropdown-nav">

                            <li class={if tab_is_active!(AppRoute::Browser, active_route) {"uk-active"}else { "" } }>
                                <a class={if tab_is_active!(AppRoute::Browser, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Browser::ByType))><span class={if tab_is_active!(AppRoute::Browser, active_route) {"active-tab fa fa-chrome"}else { "fa fa-chrome"} }></span>{" By Type"}</a>
                            </li>

                            <li class={if tab_is_active!(AppRoute::BrowserVersion, active_route) {"uk-active"}else { "" }}>
                                <a class={if tab_is_active!(AppRoute::BrowserVersion, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Browser::Version))><span class={if tab_is_active!(AppRoute::BrowserVersion, active_route) {"active-tab fa fa-code-fork"}else { "fa fa-code-fork"} }></span>{" Browser Version"}</a>
                            </li>

                        </ul>
                    </div>
                </li>
        }
    }
}

enum Browser {
    ByType,
    Version,
}
