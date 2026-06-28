use std::cell::RefCell;
use std::rc::Rc;

use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew_services::storage::Area;
use yew_services::StorageService;

use ad_buy_engine::data::elements::crud::CreatableElement;

use crate::agents::tick_tock::{TickTock, TickTockRequest};
use crate::appstate::app_state::AppState;
use crate::components::account_tab_section::custom_conversions::modal::ModalType;
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::{show_uk_modal, toggle_uk_dropdown};
use crate::utils::routes::AppRoute;
use crate::RootComponent;

pub enum Msg {
    Click,
    Update,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct NewElement {
    link: ComponentLink<Self>,
    props: Props,
    href: String,
    tt: Box<dyn Bridge<TickTock>>,
}

impl Component for NewElement {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let href = match props.state.borrow().return_app_route() {
            AppRoute::OfferSources => "#offer-sources".to_string(),
            AppRoute::Offers => "#offer".to_string(),
            AppRoute::Landers => "#landing-pages".to_string(),
            AppRoute::Traffic => "#traffic".to_string(),
            AppRoute::Funnels => "#funnels".to_string(),
            AppRoute::Campaign => "#campaigns".to_string(),
            _ => "".to_string(),
        };
        let tt = TickTock::bridge(link.callback(|_| Msg::Update));
        Self {
            link,
            props,
            href,
            tt,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                *self.props.state.borrow().crud_modal_type.borrow_mut() = ModalType::Create;
                self.tt.send(TickTockRequest::Tick);
                show_uk_modal(&self.href)
            }
            Msg::Update => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.href = match props.state.borrow().return_app_route() {
            AppRoute::OfferSources => "#offer-sources".to_string(),
            AppRoute::Offers => "#offer".to_string(),
            AppRoute::Landers => "#landing-pages".to_string(),
            AppRoute::Traffic => "#traffic-sources".to_string(),
            AppRoute::Funnels => "#funnels".to_string(),
            AppRoute::Campaign => "#campaigns".to_string(),
            _ => "".to_string(),
        };

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let html = if self.creatable_elem_btn_should_render() && self.only_render_if_is_main_tab() {
            html! {
                    <div class="uk-margin-right"><button uk-toggle="" href={&self.href} onclick=self.link.callback(|_| Msg::Click) class="uk-button uk-button-default uk-button-small uk-background-primary uk-light"><span class="fas fa-plus uk-margin-small-right"></span>{format!("New {}" ,self.get_creatable_element_text())}</button></div>
            }
        } else {
            html! {}
        };

        html
    }
}

impl NewElement {
    fn get_creatable_element_text(&self) -> String {
        let state = self.props.state.borrow();
        let tab_state = state.tab_state.borrow();
        let main_tab = tab_state.main_tab.borrow();
        let app_route = &main_tab.app_route;
        let creatable_element: CreatableElement = app_route.clone().into();
        creatable_element.to_string()
    }

    fn only_render_if_is_main_tab(&self) -> bool {
        let state = self.props.state.borrow();
        let tab_state = state.tab_state.borrow();
        let activated_tab = &*tab_state.active_tab.borrow();
        &ActivatedTab::MainTab == activated_tab
    }

    fn creatable_elem_btn_should_render(&self) -> bool {
        let state = self.props.state.borrow();
        let tab_state = state.tab_state.borrow();
        let main_tab = tab_state.main_tab.borrow();
        let app_route = &main_tab.app_route;

        if app_route == &AppRoute::Dashboard {
            return false;
        }

        matches!(
            app_route,
            AppRoute::OfferSources
                | AppRoute::Traffic
                | AppRoute::Funnels
                | AppRoute::Sequences
                | AppRoute::Landers
                | AppRoute::Offers
                | AppRoute::Campaign
        )
    }
}
