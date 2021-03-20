use super::data_table::DataTable;
// use super::page_utilities::crud_element::new_campaigns::NewCampaigns;
// use super::page_utilities::crud_element::new_funnels::NewFunnel;
// use super::page_utilities::crud_element::new_landing_pages::NewLandingPage;
// use super::page_utilities::crud_element::new_offer::NewOffer;
use super::page_utilities::crud_element::crud_offer_sources::CRUDOfferSource;
use crate::agents::tick_tock::TickTock;
use crate::appstate::app_state::AppState;
use crate::components::account_tab_section::custom_conversions::modal::ModalType;
use crate::components::app_bar::AppBar;
use crate::components::page_controller::PageController;
use crate::components::page_utilities::crud_element::crud_campaign::CRUDCampaign;
use crate::components::page_utilities::crud_element::crud_funnels::CRUDFunnel;
use crate::components::page_utilities::crud_element::crud_landing_page::CRUDLandingPage;
use crate::components::page_utilities::crud_element::crud_offer::CRUDOffer;
use crate::components::page_utilities::crud_element::crud_traffic_source::CRUDTrafficSource;
use crate::components::page_utilities::PageUtilities;
use crate::utils::routes::AppRoute;
use crate::{notify_primary, RootComponent};
use std::cell::RefCell;
use std::rc::Rc;
use yew::virtual_dom::VNode;
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct MainComponent {
    pub props: Props,
    pub link: ComponentLink<Self>,
    pub tt: Box<dyn Bridge<TickTock>>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub enum Msg {
    Ignore,
}

impl Component for MainComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tt = TickTock::bridge(link.callback(|_| Msg::Ignore));
        MainComponent { props, link, tt }
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
                            <div><PageController state=Rc::clone(&self.props.state)  /></div>
                            {self.render_dashboard()}

                            {self.render_crud_modal()}
                        </div>
                    </>
        }
    }
}

impl MainComponent {
    fn render_dashboard(&self) -> VNode {
        if let AppRoute::Dashboard = self.props.state.borrow().return_app_route() {
            VNode::from(html!{
                <h1>{"Dashboard"}</h1>
            })
        } else {
            VNode::from(html!{
                        <>
                            <div><PageUtilities state=Rc::clone(&self.props.state)  /></div>
                            <div><DataTable state=Rc::clone(&self.props.state) /></div>
                        </>
            })
        }
    }
    
    fn render_crud_modal(&self) -> VNode {
        let state = self.props.state.borrow();
        match state.return_app_route() {
            AppRoute::Offers => match *state.crud_modal_type.borrow() {
                ModalType::Create => VNode::from(
                    html! {<div><CRUDOffer state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                ),
                ModalType::Update => {
                    let selected_element = state.selected_elements.borrow().clone().pop();
                    if let Some(pos) = selected_element {
                        if let Some(restored_element) =
                            state.offers.borrow().get(pos.index).cloned()
                        {
                            VNode::from(
                                html! {<div><CRUDOffer state=Rc::clone(&self.props.state) modal_type=ModalType::Update restored_element=Some(restored_element) /></div>},
                            )
                        } else {
                            VNode::from(
                                html! {<div><CRUDOffer state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                            )
                        }
                    } else {
                        VNode::from(
                            html! {<div><CRUDOffer state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                        )
                    }
                }
            },

            AppRoute::OfferSources => match *state.crud_modal_type.borrow() {
                ModalType::Create => VNode::from(
                    html! {<div><CRUDOfferSource state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                ),
                ModalType::Update => {
                    let selected_element = state.selected_elements.borrow().clone().pop();
                    if let Some(pos) = selected_element {
                        if let Some(restored_element) =
                            state.offer_sources.borrow().get(pos.index).cloned()
                        {
                            VNode::from(
                                html! {<div><CRUDOfferSource state=Rc::clone(&self.props.state) modal_type=ModalType::Update restored_element=Some(restored_element) /></div>},
                            )
                        } else {
                            VNode::from(
                                html! {<div><CRUDOfferSource state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                            )
                        }
                    } else {
                        VNode::from(
                            html! {<div><CRUDOfferSource state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                        )
                    }
                }
            },

            AppRoute::Landers => match *state.crud_modal_type.borrow() {
                ModalType::Create => VNode::from(
                    html! {<div><CRUDLandingPage state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                ),
                ModalType::Update => {
                    let selected_element = state.selected_elements.borrow().clone().pop();
                    if let Some(pos) = selected_element {
                        if let Some(restored_element) =
                            state.landing_pages.borrow().get(pos.index).cloned()
                        {
                            VNode::from(
                                html! {<div><CRUDLandingPage state=Rc::clone(&self.props.state) modal_type=ModalType::Update restored_element=Some(restored_element) /></div>},
                            )
                        } else {
                            VNode::from(
                                html! {<div><CRUDLandingPage state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                            )
                        }
                    } else {
                        VNode::from(
                            html! {<div><CRUDLandingPage state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                        )
                    }
                }
            },

            AppRoute::Traffic => match *state.crud_modal_type.borrow() {
                ModalType::Create => VNode::from(
                    html! {<div><CRUDTrafficSource state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                ),
                ModalType::Update => {
                    let selected_element = state.selected_elements.borrow().clone().pop();
                    if let Some(pos) = selected_element {
                        if let Some(restored_element) =
                            state.traffic_sources.borrow().get(pos.index).cloned()
                        {
                            VNode::from(
                                html! {<div><CRUDTrafficSource state=Rc::clone(&self.props.state) modal_type=ModalType::Update restored_element=Some(restored_element) /></div>},
                            )
                        } else {
                            VNode::from(
                                html! {<div><CRUDTrafficSource state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                            )
                        }
                    } else {
                        VNode::from(
                            html! {<div><CRUDTrafficSource state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                        )
                    }
                }
            },

            AppRoute::Funnels => match *state.crud_modal_type.borrow() {
                ModalType::Create => VNode::from(
                    html! {<div><CRUDFunnel state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                ),
                ModalType::Update => {
                    let selected_element = state.selected_elements.borrow().clone().pop();
                    if let Some(pos) = selected_element {
                        if let Some(restored_element) =
                            state.funnels.borrow().get(pos.index).cloned()
                        {
                            VNode::from(
                                html! {<div><CRUDFunnel state=Rc::clone(&self.props.state) modal_type=ModalType::Update restored_element=Some(restored_element) /></div>},
                            )
                        } else {
                            VNode::from(
                                html! {<div><CRUDFunnel state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                            )
                        }
                    } else {
                        VNode::from(
                            html! {<div><CRUDFunnel state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                        )
                    }
                }
            },

            AppRoute::Campaign => match *state.crud_modal_type.borrow() {
                ModalType::Create => VNode::from(
                    html! {<div><CRUDCampaign state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                ),
                ModalType::Update => {
                    let selected_element = state.selected_elements.borrow().clone().pop();
                    if let Some(pos) = selected_element {
                        if let Some(restored_element) =
                            state.campaigns.borrow().get(pos.index).cloned()
                        {
                            VNode::from(
                                html! {<div><CRUDCampaign state=Rc::clone(&self.props.state) modal_type=ModalType::Update restored_element=Some(restored_element) /></div>},
                            )
                        } else {
                            VNode::from(
                                html! {<div><CRUDCampaign state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                            )
                        }
                    } else {
                        VNode::from(
                            html! {<div><CRUDCampaign state=Rc::clone(&self.props.state) modal_type=ModalType::Create /></div>},
                        )
                    }
                }
            },

            _ => VNode::from(html! {}),
        }
    }
}
