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
    OnSelect(DayParting),
    Ignore,
}

enum DayParting {
    Hour,
    DayOfWeek,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct DayPartingDrop {
    link: ComponentLink<Self>,
    props: Props,
    router: Box<dyn Bridge<RouteAgent>>,
    active: bool,
}

impl Component for DayPartingDrop {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));
        let ar = state_clone!(props.state).borrow().return_app_route();
        let active = dropdown_is_active!(AppRoute::DayOfWeek AppRoute::HourOfDay, ar);

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
                DayParting::DayOfWeek => {
                    self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::DayOfWeek,
                            );
                    self.router.send(ChangeRoute(AppRoute::DayOfWeek.into()))
                }

                DayParting::Hour => {
                    self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::HourOfDay,
                            );
                    self.router.send(ChangeRoute(AppRoute::HourOfDay.into()));
                }
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let ar = state_clone!(props.state).borrow().return_app_route();
        self.active = dropdown_is_active!(AppRoute::DayOfWeek AppRoute::HourOfDay, ar);
        true
    }

    fn view(&self) -> Html {
        let active_route = self.props.state.borrow().return_app_route();

        html! {
                <li class={if self.active {"uk-active"}else { "" }}>
                    <span class={if self.active {"active-tab fa fa-bar-chart uk-display-block uk-text-center"}else { "fa fa-bar-chart uk-display-block uk-text-center"} } onclick=callback!(self, |_| Msg::OnSelect(DayParting::DayOfWeek))></span>
                    <a class={if self.active {"active-tab uk-display-block"}else { "uk-display-block"} } onclick=callback!(self, |_| Msg::OnSelect(DayParting::DayOfWeek))>{"Day Parting"}</a>

                    <div class="uk-navbar-dropdown"  uk-drop="pos: bottom-center;" >
                        <ul class="uk-nav uk-navbar-dropdown-nav">

                            <li class={if tab_is_active!(AppRoute::DayOfWeek, active_route) {"uk-active"}else { "" } }>
                                <a class={if tab_is_active!(AppRoute::DayOfWeek, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(DayParting::DayOfWeek))><span class={if tab_is_active!(AppRoute::DayOfWeek, active_route) {"active-tab fa fa-bar-chart"}else { "fa fa-bar-chart"} }></span>{" Day of Week"}</a>
                            </li>

                            <li class={if tab_is_active!(AppRoute::HourOfDay, active_route) {"uk-active"}else { "" }}>
                                <a class={if tab_is_active!(AppRoute::HourOfDay, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(DayParting::Hour))><span class={if tab_is_active!(AppRoute::HourOfDay, active_route) {"active-tab fa fa-hourglass-half"}else { "fa fa-hourglass-half"} }></span>{" Hour of Day"}</a>
                            </li>

                        </ul>
                    </div>
                </li>
        }
    }
}
