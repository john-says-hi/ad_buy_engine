mod logout;

use crate::appstate::app_state::STATE;
use crate::utils::routes::AppRoute;
use logout::Logout;
use yew::prelude::*;
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;

pub enum Msg {
    RouteAccount,
    RouteDashboard,
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
}

pub struct AppBar {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent<()>>>,
    props: Props,
}

impl Component for AppBar {
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
            Msg::RouteDashboard => {
                self.props.state.borrow_mut().set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(AppRoute::Dashboard);
                self.router.send(ChangeRoute(AppRoute::Dashboard.into()));
            }

            Msg::RouteAccount => {
                self.props
                    .state
                    .borrow()
                    .set_app_route_and_save_to_browser(AppRoute::Account);
                self.router.send(ChangeRoute(AppRoute::Account.into()));
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <nav class="uk-navbar-container" uk-navbar="">
            <div class="uk-navbar-left"  >

                <img class="logo uk-navbar-item uk-logo" src="/secure/assets/logo.svg"/>

                <ul class="uk-navbar-nav">
                    <li class="uk-active"><a onclick=self.link.callback(|_|Msg::RouteDashboard) >{"Ad Buy Engine"}</a></li>
                </ul>

            </div>

            <div class="uk-navbar-right">

                <ul class="uk-navbar-nav">
                    <li class="uk-navbar-item">
                        <div uk-tooltip="title: This Feature is Not Built Yet"><span class="fas fa-fire uk-margin-small-right" style="color:red;"></span>{"Fuel: "} <a >{"9321351"}</a></div>
                    </li>

                    <li class="uk-navbar-item">
                        <div uk-tooltip="title: This Feature is Not Built Yet">
                                <span class="uk-icon uk-margin" uk-icon="icon: bell"></span>
                        </div>
                    </li>

                    <li class="uk-navbar-item">
                        <div uk-tooltip="title: This Feature is Not Built Yet">
                                <span class="uk-icon uk-margin" uk-icon="icon: info"></span>
                        </div>
                    </li>

                    <li class="uk-navbar-item">
                        <div uk-tooltip="title: This Feature is Not Built Yet">
                                <span class="uk-icon uk-margin" uk-icon="icon: question"></span>
                        </div>
                    </li>

                    <li class="uk-navbar-item">
                        <div>
                                <span onclick=self.link.callback(|_|Msg::RouteAccount) class="uk-icon uk-margin" uk-icon="icon: cog"></span>
                        </div>
                    </li>

                    <Logout />
                </ul>

            </div>
        </nav>
                }
    }
}
