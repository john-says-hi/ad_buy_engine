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
    UpdateOnSelect(SelectedDetail),
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
}

impl Component for ConnectionDrop {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));
        Self {
            link,
            props,
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Msg::UpdateOnSelect(data) = msg {
            if let ListIndex::Single(Some(idx)) = data.index {
                match idx {
                    0 => {
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::Connection,
                            );
                        self.router.send(ChangeRoute(AppRoute::Connection.into()));
                    }
                    1 => {
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::ISPCarrier,
                            );
                        self.router.send(ChangeRoute(AppRoute::ISPCarrier.into()))
                    }
                    2 => {
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::MobileCarrier,
                            );
                        self.router
                            .send(ChangeRoute(AppRoute::MobileCarrier.into()))
                    }
                    3 => {
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::Proxy,
                            );
                        self.router.send(ChangeRoute(AppRoute::Proxy.into()))
                    }
                    _ => {}
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.link.send_message(Msg::Ignore);
        true
    }

    fn view(&self) -> Html {
        let on_select = self
            .link
            .callback(|data: SelectedDetail| Msg::UpdateOnSelect(data));

        html! {
            <div class="data-dropdown uk-margin-left-small ">
                <MatSelect label="Connection" outlined=true icon="event" onselected=on_select>
                    <MatListItem value="0" graphic=GraphicType::Icon selected=app_route_matches(AppRoute::Connection, Rc::clone(&self.props.state)) >{"By Type"}</MatListItem>
                    <MatListItem value="1" graphic=GraphicType::Icon selected=app_route_matches(AppRoute::ISPCarrier, Rc::clone(&self.props.state)) >{"ISP / Carrier"}</MatListItem>
                    <MatListItem value="2" graphic=GraphicType::Icon selected=app_route_matches(AppRoute::MobileCarrier, Rc::clone(&self.props.state))>{"Mobile Carrier"}</MatListItem>
                    <MatListItem value="3" graphic=GraphicType::Icon  selected=app_route_matches(AppRoute::Proxy, Rc::clone(&self.props.state))>{"By Proxy"}</MatListItem>
                </MatSelect>
            </div>
        }
    }
}
