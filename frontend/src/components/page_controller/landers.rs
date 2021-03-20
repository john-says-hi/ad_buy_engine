use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::routes::AppRoute;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_material::list::GraphicType;
use yew_material::{MatListItem, MatMenu, MatSelect, MatTab, MatTabBar};
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;

pub enum Msg {
    Click,
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct LanderBtn {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    active:bool,
}

impl Component for LanderBtn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));
        let cloned_state = Rc::clone(&props.state);
        let active = tab_is_active!(AppRoute::Landers, cloned_state);

        Self {
            link,
            router,
            props,
            active
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.active=true;
                self.props
                    .state
                    .borrow()
                    .selected_elements
                    .borrow_mut()
                    .clear();
                self.props
                    .state
                    .borrow_mut()
                    .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(AppRoute::Landers);
                self.router.send(ChangeRoute(AppRoute::Landers.into()))
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let cloned_state = Rc::clone(&props.state);
        self.active = tab_is_active!(AppRoute::Landers, cloned_state);
        true
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(|_| Msg::Click);
        let a_class = if self.active{"active-tab uk-active uk-display-block"}else { "uk-display-block" };
        let icon_class = if self.active{"active-tab fa fa-money uk-display-block uk-text-center"}else { "fa fa-money uk-display-block uk-text-center" };
        html! {
        <li onclick=callback>
            <span class=icon_class></span>
            <a class=a_class>{"Landers"}</a>
        </li>
        }
    }
}
