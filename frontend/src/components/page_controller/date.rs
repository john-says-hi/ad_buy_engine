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

pub struct DateDrop {
    link: ComponentLink<Self>,
    props: Props,
    router: Box<dyn Bridge<RouteAgent>>,
}

impl Component for DateDrop {
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
                                AppRoute::DateDay,
                            );
                        self.router.send(ChangeRoute(AppRoute::DateDay.into()));
                    }
                    1 => {
                        self.props
                            .state
                            .borrow_mut()
                            .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
                                AppRoute::DateMonth,
                            );
                        self.router.send(ChangeRoute(AppRoute::DateMonth.into()))
                    }
                    _ => {}
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let on_select = self
            .link
            .callback(|data: SelectedDetail| Msg::UpdateOnSelect(data));

        html! {
        <div class="data-dropdown">
                <MatSelect label="Date" outlined=true icon="event" onselected=on_select>
                    <MatListItem value="0" graphic=GraphicType::Icon selected=app_route_matches(AppRoute::DateDay, Rc::clone(&self.props.state))>{"Day"}</MatListItem>
                    <MatListItem value="1" graphic=GraphicType::Icon selected=app_route_matches(AppRoute::DateMonth , Rc::clone(&self.props.state))>{"Month"}</MatListItem>
                </MatSelect>
        </div>
        }
    }
}
