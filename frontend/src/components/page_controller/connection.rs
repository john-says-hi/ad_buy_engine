use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::routes::route_helpers::app_route_matches;
use crate::utils::routes::AppRoute;
use crate::{notify_primary, RootComponent};
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_material::list::{GraphicType, ListIndex, SelectedDetail};
use yew_material::select::ActionDetail;
use yew_material::{MatListItem, MatMenu, MatSelect, MatTab, MatTabBar};
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;

enum Conn {
    ByType,
    ISPCarrier,
    MobileCarrier,
    Proxy,
}

pub enum Msg {
    OnSelect(Conn),
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct ConnectionDrop {
    link: ComponentLink<Self>,
    props: Props,
    router: Box<dyn Bridge<RouteAgent>>,
    active:bool,
}

impl Component for ConnectionDrop {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));
        let active=dropdown_is_active!(AppRoute::Connection AppRoute::ISPCarrier AppRoute::MobileCarrier AppRoute::Proxy, state_clone!(props.state));

        Self {
            link,
            props,
            router,
            active
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnSelect(conn)=>{
                match conn {
                    Conn::ByType=>{
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::Connection,
                            );
                        self.router.send(ChangeRoute(AppRoute::Connection.into()));
                    }
                    
                    Conn::ISPCarrier =>{
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::ISPCarrier,
                            );
                        self.router.send(ChangeRoute(AppRoute::ISPCarrier.into()))
                    }
    
                    Conn::MobileCarrier =>{
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::MobileCarrier,
                            );
                        self.router
                            .send(ChangeRoute(AppRoute::MobileCarrier.into()))
                    }
    
                    Conn::Proxy =>{
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::Proxy,
                            );
                        self.router.send(ChangeRoute(AppRoute::Proxy.into()))
                    }
                }
            }
            
            Msg::Ignore=>{}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.active=dropdown_is_active!(AppRoute::Connection AppRoute::ISPCarrier AppRoute::MobileCarrier AppRoute::Proxy, state_clone!(self.props.state));
        true
    }

    fn view(&self) -> Html {
        let active = if self.active{"uk-active"}else { "" };
        html! {
                <li class=active>
                    <a onclick=callback!(self, |_| Msg::OnSelect(Conn::ByType))>{"Connection"}</a>
                    <div class="uk-navbar-dropdown">
                        <ul class="uk-nav uk-navbar-dropdown-nav">
                        
                            <li class={if tab_is_active!(AppRoute::Connection, state_clone!(self.props.state)) {"uk-active"}else { "" }}><a onclick=callback!(self, |_| Msg::OnSelect(Conn::ByType))>{"By Type"}</a></li>
                            <li class={if tab_is_active!(AppRoute::ISPCarrier, state_clone!(self.props.state)) {"uk-active"}else { "" }}><a onclick=callback!(self, |_| Msg::OnSelect(Conn::ISPCarrier))>{"ISP/Carrier"}</a></li>
                            <li class={if tab_is_active!(AppRoute::MobileCarrier, state_clone!(self.props.state)) {"uk-active"}else { "" }}><a onclick=callback!(self, |_| Msg::OnSelect(Conn::MobileCarrier))>{"Mobile Carrier"}</a></li>
                            <li class={if tab_is_active!(AppRoute::Proxy, state_clone!(self.props.state)) {"uk-active"}else { "" }}><a onclick=callback!(self, |_| Msg::OnSelect(Conn::Proxy))>{"Proxy"}</a></li>
                            
                        </ul>
                    </div>
                </li>
        }
    }
}
