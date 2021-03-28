use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_funnel_view_basic::RHSFunnelViewBasic;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::offer_dropdown::OfferDropdown;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
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
    Select(Offer),
    RemoveOffer(usize),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub eject_selected_offers: Callback<Vec<WeightedOffer>>,
    pub offers: Vec<WeightedOffer>,
}

pub struct OfferSelector {
    link: ComponentLink<Self>,
    props: Props,
    weight: String,
    offers: Vec<WeightedOffer>,
}

impl Component for OfferSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let offers = props.offers.clone();

        Self {
            link,
            props,
            weight: "".to_string(),
            offers,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateWeight((pos, i)) => {
                if let Ok(weight) = i.value.parse::<u8>() {
                    self.offers.get_mut(pos).map(|s| s.weight = weight);
                    self.props.eject_selected_offers.emit(self.offers.clone());
                } else {
                    notify_danger("Please enter a number between 0-255")
                }
            }

            Msg::Select(offer) => {
                self.offers.push(WeightedOffer { weight: 100, offer });
                self.props.eject_selected_offers.emit(self.offers.clone());
            }

            Msg::RemoveOffer(pos) => {
                self.offers.remove(pos);
                self.props.eject_selected_offers.emit(self.offers.clone());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.offers = props.offers.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                <div class="uk-margin-top uk-margin-bottom-remove">
                                    {label!("o", "Offers")}
                                </div>
                                {divider!(2)}

                                {self.render_offers()}

                                <div>{label!("g", "Select Offer")}<OfferDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(Msg::Select) selected=None /></div>
        </>
        }
    }
}

impl OfferSelector {
    pub fn render_offers(&self) -> VList {
        let mut nodes = VList::new();

        for (idx, offer) in self.offers.iter().enumerate() {
            let name = offer.offer.name.clone();
            let weight = offer.weight;
            nodes.push(html!{
                                <>
                                <div class="uk-margin uk-flex uk-flex-middle uk-text-center uk-child-width-1-3" uk-grid="">
                                    <div>{label!("Name")}<p>{format!("#{} - {}", idx+1, name)}</p></div>
                                    <div>{label!("Weight")}<input type="number" class="uk-input" value=weight.to_string() oninput=self.link.callback(move|i:InputData|Msg::UpdateWeight((idx,i))) /></div>
                                    <div class="uk-flex-middle">{label!("Remove")}<div><button class="uk-button uk-button-small" onclick=self.link.callback(move |_| Msg::RemoveOffer(idx)) >{"X"}</button></div></div>
                                </div>
                                {divider!()}
                                </>
            })
        }

        nodes
    }
}
