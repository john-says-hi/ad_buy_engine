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

pub struct TrafficBtn {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    props: Props,
}

impl Component for TrafficBtn {
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
            Msg::Click => {
                self.props
                    .state
                    .borrow()
                    .selected_elements
                    .borrow_mut()
                    .clear();
                self.props
                    .state
                    .borrow_mut()
                    .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(AppRoute::Traffic);
                self.router.send(ChangeRoute(AppRoute::Traffic.into()))
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.link.send_message(Msg::Ignore);
        true
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(|_| Msg::Click);

        html! {
            <MatTab icon="double_arrow" label="Traffic Sources" stacked=true is_min_width_indicator=true oninteracted=callback />
        }
    }
}
