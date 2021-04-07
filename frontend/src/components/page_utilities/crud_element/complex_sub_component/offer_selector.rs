use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_funnel_view_basic::RHSFunnelViewBasic;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::offer_dropdown::OfferDropdown;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::elements::offer::Offer;
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
use ad_buy_engine::Country;
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};

use ad_buy_engine::constant::COLOR_GRAY;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    UpdateWeight((usize, usize, InputData)),
    Select((usize, Offer)),
    RemoveOffer((usize, usize)),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub eject_selected_offers: Callback<Vec<Vec<Offer>>>,
    pub offers: Vec<Vec<Offer>>,
}

pub struct OfferSelector {
    link: ComponentLink<Self>,
    props: Props,
    weight: String,
    offers: Vec<Vec<Offer>>,
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
            Msg::UpdateWeight((group_pos, offer_pos, i)) => {
                if let Ok(weight) = i.value.parse::<u8>() {
                    self.offers
                        .get_mut(group_pos)
                        .map(|s| {
                            s.get_mut(offer_pos)
                                .map(|s| s.weight = weight)
                                .unwrap_or_else(|| notify_danger("e:G%6d"))
                        })
                        .unwrap_or_else(|| notify_danger("e:g56t4434"));
                    self.props.eject_selected_offers.emit(self.offers.clone());
                } else {
                    notify_danger("Please enter a number between 0-255")
                }
            }

            Msg::Select((group_pos, offer)) => {
                self.offers
                    .get_mut(group_pos)
                    .map(|s| s.push(offer.into()))
                    .unwrap_or_else(|| notify_danger("E:f43f4"));
                self.props.eject_selected_offers.emit(self.offers.clone());
            }

            Msg::RemoveOffer((group_pos, offer_pos)) => {
                self.offers.get_mut(group_pos).map(|s| s.remove(offer_pos));
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
                                {self.header()}
                                {self.body()}
        </>
        }
    }
}

impl OfferSelector {
    pub fn body(&self) -> VNode {
        let mut nodes = VList::new();

        self.offers
            .iter()
            .enumerate()
            .map(|(group_idx, _)| nodes.push(self.render_offer_group(group_idx)));

        VNode::from(nodes)
    }

    pub fn render_offer_group(&self, group_pos: usize) -> VNode {
        let group_number = format!("Offer Group {} of {}", group_pos + 1, self.offers.len() + 1);
        let mut nodes = VList::new();
        nodes.push(html! {
            <>
                {if self.offers.len()==1 {
                html!{
                    {divider!(2)}
                }
                } else {
                    html!{
                    <>
                        {divider!(2)}
                        {label!("p", &group_number)}
                    </>
                    }
                }}
            </>
        });

        for (idx, offer) in self
            .offers
            .get(group_pos)
            .expect("E:45vRT4")
            .iter()
            .enumerate()
        {
            let name = offer.name.clone();
            let weight = offer.weight;
            nodes.push(html!{
                                <>
                                <div class="uk-margin uk-flex uk-flex-middle uk-text-center uk-child-width-1-3" uk-grid="">
                                    <div>{label!("Name")}<p>{format!("{}", name)}</p></div>
                                    <div>{label!("Weight")}<input type="number" class="uk-input" value=weight.to_string() oninput=self.link.callback(move|i:InputData|Msg::UpdateWeight((group_pos, idx,i))) /></div>
                                    <div class="uk-flex-middle">{label!("Remove")}<div><button class="uk-button uk-button-small" onclick=self.link.callback(move |_| Msg::RemoveOffer((group_pos, idx))) >{"X"}</button></div></div>
                                </div>
                                {divider!()}
                                </>
            })
        }

        nodes.push(html!{
            <div>{label!("g", "Select Offer")}<OfferDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(move |o:Offer| Msg::Select((group_pos, o))) selected=None /></div>
        });

        VNode::from(nodes)
    }

    pub fn header(&self) -> VNode {
        if self.offers.len() == 1 {
            html! {
            <>
                <div class="uk-margin-top uk-margin-bottom-remove">
                    {label!("o", "Offers")}
                </div>
            </>
            }
        } else {
            html! {
            <>
                <div class="uk-margin-top uk-margin-bottom-remove">
                    {label!("o", "Offer Groups")}
                </div>
            </>
            }
        }
    }
}
