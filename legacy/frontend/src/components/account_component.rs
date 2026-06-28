use crate::agents::tick_tock::TickTock;
use crate::appstate::app_state::AppState;
use crate::components::account_tab_bar::AccountTabBar;
use crate::components::account_tab_section::custom_conversions::CustomConversionSection;
use crate::components::account_tab_section::referrer_handling::ReferrerHandlingAccountSection;
use crate::components::account_tab_section::AccountTabDefault;
use crate::components::app_bar::AppBar;
use crate::components::page_controller::PageController;
use crate::components::page_utilities::PageUtilities;
use crate::utils::routes::AppRoute;
use crate::{notify_primary, RootComponent};
use std::cell::RefCell;
use std::rc::Rc;
use yew::virtual_dom::VNode;
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct AccountComponent {
    pub props: Props,
    pub link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub enum Msg {
    Ignore,
}

impl Component for AccountComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        AccountComponent { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
                    <>
                        <div class="uk-child-width-1-1 uk-grid-collapse uk-background-default" uk-grid="">
                            <div><AppBar state=Rc::clone(&self.props.state) /></div>
                            <div><AccountTabBar state=Rc::clone(&self.props.state)  /></div>
                            {self.render_account_section()}
                        </div>
                    </>
        }
    }
}

impl AccountComponent {
    pub fn render_account_section(&self) -> VNode {
        match self.props.state.borrow().return_app_route() {
            AppRoute::ReferrerHandling => {
                html! {
                    < div > < ReferrerHandlingAccountSection state = Rc::clone(&self.props.state) / > < / div >
                }
            }
            AppRoute::CustomConversions => {
                html! {
                    < div > < CustomConversionSection state = Rc::clone(&self.props.state) / > < / div >
                }
            }
            AppRoute::Account => {
                html! {
                    < div > < AccountTabDefault
                    state = Rc::clone(&self.props.state) / > < / div >
                }
            }
            _ => {
                html! {
                    < div > < AccountTabDefault
                    state = Rc::clone(&self.props.state) / > < / div >
                }
            }
        }
    }
}
