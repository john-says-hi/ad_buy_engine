use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::funnel_view_basic_data::FunnelViewBasicData;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::landing_page_dropdown::LandingPageDropdown;
use crate::components::page_utilities::crud_element::dropdowns::offer_dropdown::OfferDropdown;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::elements::landing_page::{LandingPage, WeightedLandingPage};
use ad_buy_engine::data::elements::offer::{Offer, WeightedOffer};
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
use ad_buy_engine::Country;
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_material::MatSwitch;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    UpdateWeight((usize, InputData)),
    Select(LandingPage),
    RemoveLandingPage(usize),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub eject_selected_landing_pages: Callback<Vec<WeightedLandingPage>>,
}

pub struct LandingPageSelector {
    link: ComponentLink<Self>,
    props: Props,
    weight: String,
    landing_pages: Vec<WeightedLandingPage>,
}

impl Component for LandingPageSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            weight: "".to_string(),
            landing_pages: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateWeight((pos, i)) => {
                if let Ok(weight) = i.value.parse::<u8>() {
                    self.landing_pages.get_mut(pos).map(|s| s.weight = weight);
                    self.props
                        .eject_selected_landing_pages
                        .emit(self.landing_pages.clone());
                } else {
                    notify_danger("Please enter a number between 0-255")
                }
            }

            Msg::Select(landing_page) => {
                self.landing_pages.push(WeightedLandingPage {
                    weight: 100,
                    landing_page,
                });
                self.props
                    .eject_selected_landing_pages
                    .emit(self.landing_pages.clone());
            }

            Msg::RemoveLandingPage(pos) => {
                self.landing_pages.remove(pos);
                self.props
                    .eject_selected_landing_pages
                    .emit(self.landing_pages.clone());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                <div class="uk-margin">
                                    <h4>{"Landing Pages"}</h4>
                                </div>

                                <hr class="uk-divider" />

                                {self.render_landers()}

                                <div class="uk-margin"><LandingPageDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(Msg::Select) selected=None /></div>
        </>
        }
    }
}

impl LandingPageSelector {
    pub fn render_landers(&self) -> VList {
        let mut nodes = VList::new();

        for (idx, landing_page) in self.landing_pages.iter().enumerate() {
            let name = landing_page.landing_page.name.clone();
            let weight = landing_page.weight;
            nodes.push(html!{
                                <div class="uk-margin">
                                    <h5>{name}</h5>
                                    <button class="uk-button" onclick=self.link.callback(move |_| Msg::RemoveLandingPage(idx)) >{"X"}</button>
                                    <input type="number" class="uk-input" value=weight.to_string() oninput=self.link.callback(move|i:InputData|Msg::UpdateWeight((idx,i))) />
                                </div>
            })
        }

        nodes
    }
}
