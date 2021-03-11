pub mod name_field;
pub mod row;
pub mod select_field;

use row::Row;

use crate::agents::tick_tock::TickTock;
use crate::appstate::app_state::{AppState, STATE};
use crate::appstate::lists::PrimeElement;
use crate::components::tab_state::ActivatedTab;
use crate::notify_primary;
use crate::utils::routes::AppRoute;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};

pub enum Msg {
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
}

pub struct DataTableBody {
    link: ComponentLink<Self>,
    props: Props,
    tt: Box<dyn Bridge<TickTock>>,
}

impl Component for DataTableBody {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tt = TickTock::bridge(link.callback(|_| Msg::Ignore));
        Self { link, props, tt }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <tbody>
           {self.generate_rows()}
        </tbody>
                        }
    }
}

impl DataTableBody {
    fn generate_rows(&self) -> VNode {
        let mut nodes = VList::new();
        let state = self.props.state.borrow();
        let tab_state = state.tab_state.borrow();
        let active_state = &*tab_state.active_tab.borrow();

        match active_state {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                let prime_element_group = main_tab.prime_grouping_columns.clone();

                match prime_element_group.count_columns() {
                    1 => match prime_element_group.first_column {
                        PrimeElement::OfferSources => {
                            let items = state.offer_sources.borrow();

                            for (idx, elem) in items.iter().enumerate() {
                                nodes.push(html! {
                                <Row state=Rc::clone(&self.props.state) name=&elem.name prime_element=PrimeElement::OfferSources index=idx />
                                })
                            }
                        }

                        PrimeElement::Offers => {
                            let items = state.offers.borrow();

                            for (idx, elem) in items.iter().enumerate() {
                                nodes.push(html! {
                                <Row state=Rc::clone(&self.props.state) name=&elem.name prime_element=PrimeElement::Offers index=idx />
                                })
                            }
                        }

                        PrimeElement::Landers => {
                            let items = state.landing_pages.borrow();

                            for (idx, elem) in items.iter().enumerate() {
                                nodes.push(html! {
                                <Row state=Rc::clone(&self.props.state) name=&elem.name prime_element=PrimeElement::Landers index=idx />
                                })
                            }
                        }

                        PrimeElement::TrafficSources => {
                            let items = state.traffic_sources.borrow();

                            for (idx, elem) in items.iter().enumerate() {
                                nodes.push(html! {
                                <Row state=Rc::clone(&self.props.state) name=&elem.name prime_element=PrimeElement::TrafficSources index=idx />
                                })
                            }
                        }

                        PrimeElement::Funnels => {
                            let items = state.funnels.borrow();

                            for (idx, elem) in items.iter().enumerate() {
                                nodes.push(html! {
                                <Row state=Rc::clone(&self.props.state) name=&elem.name prime_element=PrimeElement::Funnels index=idx />
                                })
                            }
                        }

                        PrimeElement::Campaigns => {
                            let items = state.campaigns.borrow();

                            for (idx, elem) in items.iter().enumerate() {
                                nodes.push(html! {
                                <Row state=Rc::clone(&self.props.state) name=&elem.name prime_element=PrimeElement::Campaigns index=idx />
                                })
                            }
                        }

                        _ => {}
                    },

                    2 => {} // compound row first, second, optional third ??

                    3 => {}

                    _ => {}
                }
            }

            ActivatedTab::ReportTabState(tab) => {}
        }

        VNode::from(nodes)
    }
}
