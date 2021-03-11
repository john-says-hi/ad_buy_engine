use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::notify_primary;
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

pub struct ReferrerHanldingBtn {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    props: Props,
}

impl Component for ReferrerHanldingBtn {
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
                    .borrow_mut()
                    .set_app_route_and_save_to_browser(AppRoute::ReferrerHandling);
                self.router
                    .send(ChangeRoute(AppRoute::ReferrerHandling.into()));
            }
            Msg::Ignore => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.link.send_message(Msg::Ignore);
        false
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(|_| Msg::Click);

        html! {
            <MatTab icon="campaign" label="Referrer Handling" stacked=true is_min_width_indicator=true oninteracted=callback />
        }
    }
}
