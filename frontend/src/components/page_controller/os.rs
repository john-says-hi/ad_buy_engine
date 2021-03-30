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
    OnSelect(OS),
    Ignore,
}

enum OS {
    Type,
    Version,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct OSDrop {
    link: ComponentLink<Self>,
    props: Props,
    router: Box<dyn Bridge<RouteAgent>>,
    active: bool,
}

impl Component for OSDrop {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));
        let ar = state_clone!(props.state).borrow().return_app_route();
        let active = dropdown_is_active!(AppRoute::OS AppRoute::OSVersion, ar);

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

            Msg::OnSelect(data) => match data {
                OS::Type => {
                    self.props
                        .state
                        .borrow_mut()
                        .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(AppRoute::OS);
                    self.router.send(ChangeRoute(AppRoute::OS.into()));
                }
                OS::Version => {
                    self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::OSVersion,
                            );
                    self.router.send(ChangeRoute(AppRoute::OSVersion.into()))
                }
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let ar = state_clone!(props.state).borrow().return_app_route();
        self.active = dropdown_is_active!(AppRoute::OS AppRoute::OSVersion, ar);
        true
    }

    fn view(&self) -> Html {
        let active_route = self.props.state.borrow().return_app_route();

        html! {
                <li class={if self.active {"uk-active"}else { "" }}>
                    <span class={if self.active {"active-tab fa fa-desktop uk-display-block uk-text-center"}else { "fa fa-desktop uk-display-block uk-text-center"} } onclick=callback!(self, |_| Msg::OnSelect(OS::Type))></span>
                    <a class={if self.active {"active-tab uk-display-block"}else { "uk-display-block"} } onclick=callback!(self, |_| Msg::OnSelect(OS::Type))>{"OS"}</a>

                    <div class="uk-navbar-dropdown"  uk-drop="pos: bottom-center;" >
                        <ul class="uk-nav uk-navbar-dropdown-nav">

                            <li class={if tab_is_active!(AppRoute::OS, active_route) {"uk-active"}else { "" } }>
                                <a class={if tab_is_active!(AppRoute::OS, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(OS::Type))><span class={if tab_is_active!(AppRoute::OS, active_route) {"active-tab fa fa-desktop"}else { "fa fa-desktop"} }></span>{" By Type"}</a>
                            </li>

                            <li class={if tab_is_active!(AppRoute::OSVersion, active_route) {"uk-active"}else { "" }}>
                                <a class={if tab_is_active!(AppRoute::OSVersion, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(OS::Version))><span class={if tab_is_active!(AppRoute::OSVersion, active_route) {"active-tab fa fa-code-fork"}else { "fa fa-code-fork"} }></span>{" By Version"}</a>
                            </li>

                        </ul>
                    </div>
                </li>
        }
    }
}
