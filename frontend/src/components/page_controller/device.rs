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

pub enum Msg {
    OnSelect(Device),
    Ignore,
}

enum Device {
    Type,
    Brand,
    Model,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct DevicesDrop {
    link: ComponentLink<Self>,
    props: Props,
    router: Box<dyn Bridge<RouteAgent>>,
    active:bool
}

impl Component for DevicesDrop {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));
        let ar=state_clone!(props.state).borrow().return_app_route();
        let active= dropdown_is_active!(AppRoute::Devices AppRoute::Model AppRoute::Brand, ar);
        
        Self {
            link,
            props,
            router,
            active
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore=>{}
        
            Msg::OnSelect(data)=>{
                match data {
                Device::Type=>{
                    self.props
                        .state
                        .borrow_mut()
                        .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                            AppRoute::Devices,
                        );
                    self.router.send(ChangeRoute(AppRoute::Devices.into()));
                }
                    
                    Device::Brand=>{
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::Brand,
                            );
                        self.router.send(ChangeRoute(AppRoute::Brand.into()))
                    }
                    
                    Device::Model =>{
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::Model,
                            );
                        self.router.send(ChangeRoute(AppRoute::Model.into()))
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        let ar=state_clone!(props.state).borrow().return_app_route();
        self.active= dropdown_is_active!(AppRoute::Devices AppRoute::Model AppRoute::Brand, ar);
        true
    }
    
    fn view(&self) -> Html {
        let active_route = self.props.state.borrow().return_app_route();
        
        html! {
                <li class={if self.active {"uk-active"}else { "" }}>
                    <span class={if self.active {"active-tab fa fa-laptop uk-display-block uk-text-center"}else { "fa fa-laptop uk-display-block uk-text-center"} } onclick=callback!(self, |_| Msg::OnSelect(Device::Type))></span>
                    <a class={if self.active {"active-tab uk-display-block"}else { "uk-display-block"} } onclick=callback!(self, |_| Msg::OnSelect(Device::Type))>{"Device"}</a>
                    
                    <div class="uk-navbar-dropdown"  uk-drop="pos: bottom-center;" >
                        <ul class="uk-nav uk-navbar-dropdown-nav">
                        
                            <li class={if tab_is_active!(AppRoute::Devices, active_route) {"uk-active"}else { "" } }>
                                <a class={if tab_is_active!(AppRoute::Devices, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Device::Type))><span class={if tab_is_active!(AppRoute::Devices, active_route) {"active-tab fa fa-laptop"}else { "fa fa-laptop"} }></span>{" By Type"}</a>
                            </li>
                            
                            <li class={if tab_is_active!(AppRoute::Brand, active_route) {"uk-active"}else { "" }}>
                                <a class={if tab_is_active!(AppRoute::Brand, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Device::Brand))><span class={if tab_is_active!(AppRoute::Brand, active_route) {"active-tab fa fa-shopping-bag"}else { "fa fa-shopping-bag"} }></span>{" By Brand"}</a>
                            </li>
                            
                            <li class={if tab_is_active!(AppRoute::Model, active_route) {"uk-active"}else { "" }}>
                                <a class={if tab_is_active!(AppRoute::Model, active_route) {"active-tab"}else { ""} } onclick=callback!(self, |_| Msg::OnSelect(Device::Model))><span class={if tab_is_active!(AppRoute::Model, active_route) {"active-tab fa fa-codepen"}else { "fa fa-codepen"} }></span>{" By Model"}</a>
                            </li>
                            
                        </ul>
                    </div>
                </li>
        }
    }
}
