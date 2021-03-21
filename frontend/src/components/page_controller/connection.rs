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
        let app_route = state_clone!(props.state).borrow().return_app_route();
        let active=dropdown_is_active!(AppRoute::Connection AppRoute::ISPCarrier AppRoute::MobileCarrier AppRoute::Proxy, app_route);

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
        let active_tab = state_clone!(props.state).borrow().return_app_route();
        self.active=dropdown_is_active!(AppRoute::Connection AppRoute::ISPCarrier AppRoute::MobileCarrier AppRoute::Proxy, active_tab);
        true
    }

    fn view(&self) -> Html {
        let active_tab = self.props.state.borrow().return_app_route();
        
        html! {
                <li class={if self.active {"uk-active"}else { "" }}>
                    <span class={if self.active {"active-tab fa fa-connectdevelop uk-display-block uk-text-center"}else { "fa fa-connectdevelop uk-display-block uk-text-center"} } onclick=callback!(self, |_| Msg::OnSelect(Conn::ByType))></span>
                    <a class={if self.active {"active-tab uk-display-block"}else { "uk-display-block"} } onclick=callback!(self, |_| Msg::OnSelect(Conn::ByType))>{"Connection"}</a>
                    
                    <div class="uk-navbar-dropdown"  uk-drop="pos: bottom-center;" >
                        <ul class="uk-nav uk-navbar-dropdown-nav">
                        
                            <li class={if tab_is_active!(AppRoute::Connection, active_tab) {"uk-active"}else { "" } }>
                                <a class={if tab_is_active!(AppRoute::Connection, active_tab) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Conn::ByType))><span class={if tab_is_active!(AppRoute::Connection, active_tab) {"active-tab fa fa-connectdevelop"}else { "fa fa-connectdevelop"} }></span>{" By Type"}</a>
                            </li>
                            
                            <li class={if tab_is_active!(AppRoute::ISPCarrier, active_tab) {"uk-active"}else { "" }}>
                                <a class={if tab_is_active!(AppRoute::ISPCarrier, active_tab) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Conn::ISPCarrier))><span class={if tab_is_active!(AppRoute::ISPCarrier, active_tab) {"active-tab fa fa-crosshairs"}else { "fa fa-crosshairs"} }></span>{" ISP/Carrier"}</a>
                            </li>
                            
                            <li class={if tab_is_active!(AppRoute::MobileCarrier, active_tab) {"uk-active"}else { "" }}>
                                <a class={if tab_is_active!(AppRoute::MobileCarrier, active_tab) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Conn::MobileCarrier))><span class={if tab_is_active!(AppRoute::MobileCarrier, active_tab) {"active-tab fa fa-mobile"}else { "fa fa-mobile"} }></span>{" Mobile Carrier"}</a>
                            </li>
                            
                            <li class={if tab_is_active!(AppRoute::Proxy, active_tab) {"uk-active"}else { "" }}>
                                <a class={if tab_is_active!(AppRoute::Proxy, active_tab) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Conn::Proxy))><span class={if tab_is_active!(AppRoute::Proxy, active_tab) {"active-tab fa fa-ban"}else { "fa fa-ban"} }></span>{" Proxy"}</a>
                            </li>
                            
                        </ul>
                    </div>
                </li>
        }
    }
}
