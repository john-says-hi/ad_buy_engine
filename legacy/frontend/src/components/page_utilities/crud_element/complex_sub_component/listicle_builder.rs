use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_funnel_view_basic::RHSFunnelViewBasic;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::landing_page_dropdown::LandingPageDropdown;
use crate::components::page_utilities::crud_element::dropdowns::offer_dropdown::OfferDropdown;
use crate::{notify_danger, notify_warning, notify_primary, notify_debug};
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType, MatrixPair};
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

use yew_services::storage::Area;
use yew_services::StorageService;
use crate::components::page_utilities::crud_element::dropdowns::pre_landing_page_dropdown::PreLandingPageDropdown;

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub eject_listicle: Callback<Sequence>,
    pub active_sequence: Sequence,
    pub psp:Vec<WeightedLandingPage>,
    pub landing_pages:Vec<Vec<WeightedLandingPage>>,
    pub offers: Vec<Vec<WeightedOffer>>,
    // pub pairs: Vec<MatrixPair>,
}

// #[derive(Copy, Clone)]
// pub struct OfferPosition {
//     pub     landing_page_group_index: usize,
//     pub index_in_landing_page: usize,
// }


// pub struct OfferGrouping {
//     pub     landing_page_group_index: usize,
//     pub index_in_landing_page: usize,
//     pub offer: Offer,
// }

// pub struct LandingPageGrouping {
//     pub group_index: usize,
//     pub landing_page: LandingPage,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct OptionalPair {
//     pub landing_page: Option<LandingPage>,
//     pub offer: Vec<Option<Offer>>,
// }

// impl From<OptionalPair> for MatrixPair {
//     fn from(optional_pair:OptionalPair)->Self{
//         Self{
//             landing_page:optional_pair.landing_page.expect("g534s"),
//             offer:optional_pair.offer.iter().map(|s| s.clone().expect("Gr34s")).collect::<Vec<Offer>>(),
//         }
//     }
// }

// impl From<MatrixPair> for OptionalPair {
//     fn from(lp:MatrixPair)->Self{
//         Self{
//             landing_page:Some(lp.landing_page),
//             offer: lp.offer.iter().map(|s| Some(s.clone())).collect::<Vec<Option<Offer>>>(),
//         }
//     }
// }

pub enum Msg {
    Submit,
    SelectPreLandingPage(LandingPage),
    SelectLandingPage(LandingPageGrouping),
    SelectOffer(OfferGrouping),
    RemoveOffer(OfferPosition),
}

pub struct MatrixBuilder {
    link: ComponentLink<Self>,
    props: Props,
    pre_landing_page:Vec<WeightedLandingPage>,
    landing_pages: Vec<Vec<WeightedLandingPage>>,
    offers: Vec<Vec<WeightedOffer>>,
    // pairs: Vec<OptionalPair>,
}

impl Component for MatrixBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let pre_landing_page = props.psp.clone();
        let pairs = props.pairs.iter().map(|s| s.clone().into()).collect::<Vec<OptionalPair>>();
        
        Self {
            
            link, props,pre_landing_page,pairs }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RemoveOffer(pos)=>{
                self.pairs.get_mut(pos.landing_page_group_index).map(|s| s.offer.get_mut(pos.index_in_landing_page).map(|mut s| *s = None).unwrap_or_else(|| notify_danger("E:dreg55"))).unwrap_or_else(|| notify_danger("E:G5g5r"));
            }
            
            Msg::Submit=>{
                if self.all_fields_filled_out() {
                    let mut new_seq = self.props.active_sequence.clone();
                        new_seq.pre_landing_page = self.pre_landing_page.clone();
                        new_seq.listicle_pairs = self.pairs.iter().map(|s| s.clone().into()).collect::<Vec<MatrixPair>>();
                    
                    notify_primary("Saved!");
                    self.props.eject_listicle.emit(new_seq);
                } else {
                    notify_warning("Please fill out all fields")
                }
            }
            
            Msg::SelectOffer(offer_group)=>{
                if let Some(pair) = self.pairs.get_mut(offer_group.landing_page_group_index) {
                    if let Some(option_offer)=pair.offer.get_mut(offer_group.index_in_landing_page) {
                        option_offer.replace(offer_group.offer);
                    } else {
                        notify_danger("Err: Offer not found in group")
                    }
                } else {
                    notify_danger("Err: Pair not found")
                }
                self.link.send_message(Msg::Submit);
            }
            
            Msg::SelectLandingPage(landing_page_group)=>{
                 if let Some(pair)= self.pairs.get_mut(landing_page_group.group_index) {
                     let num_of_ctas=landing_page_group.landing_page.number_of_calls_to_action;
                     pair.landing_page=Some(landing_page_group.landing_page);
                     
                     let mut offers = vec![];
                     for cta in 0..num_of_ctas {
                         offers.push(None);
                     }
                     
                     pair.offer=offers;
                 } else {
                     notify_danger("Err: Pair not found")
                 }
            }
            
            Msg::SelectPreLandingPage(lp) => {
                let mut pairs
                    =vec![];
                for pair in 0..lp.number_of_calls_to_action {
                    pairs.push(OptionalPair{
                        landing_page:None,
                        offer:vec![],
                    })
                }
                self.pairs=pairs;
                self.pre_landing_page=Some(lp);
                notify_debug(format!("before: {}",self.pairs.len()));
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.pre_landing_page=props.psp.clone();
        self.pairs=props.pairs.iter().map(|s| s.clone().into()).collect::<Vec<OptionalPair>>();
    
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                <div class="uk-margin-top uk-margin-bottom-remove">
                                    {label!("o", "Matrix Setup")}
                                </div>
                                {divider!(2)}

                                {self.render_pre_lander()}
        </>
        }
    }
}

impl MatrixBuilder {
    pub fn render_pre_lander(&self) -> VNode {
    
        let node = if let Some(pre_lander)= &self.pre_landing_page {
            html!{
            <>
                                <div>{label!("g", "Select Pre Landing Page")}<PreLandingPageDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(Msg::SelectPreLandingPage) selected=Some(pre_lander.clone()) /></div>
                                {divider!()}
                                {self.render_lander_pairs()}
            </>
            }
        } else {
            html!{
                                <div>{label!("g", "Select Pre Landing Page")}<PreLandingPageDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(Msg::SelectPreLandingPage) selected=None /></div>
            }
        };
        
        VNode::from(node)
    }
    
    pub fn offer_or_drop(&self, offer_option:&Option<Offer>, lander_index: usize, offer_index: usize) -> VNode {
        if let Some(offer) = offer_option.clone() {
            let pos = OfferPosition {
                landing_page_group_index: lander_index,
                index_in_landing_page: offer_index
            };
            
            VNode::from(html!{
                <div>
                    <span class="uk-margin-right-large">{format!("Name: {}", &offer.name)}</span>
                    <button onclick=callback!(self, move |_| Msg::RemoveOffer(pos)) class="uk-button uk-button-small uk-margin-left-large">{"Remove"}</button>
                </div>
            })
        } else {
            VNode::from(html!{
                <OfferDropdown
                    state=Rc::clone(&self.props.state)
                    eject=self.link.callback(move |offer:Offer| Msg::SelectOffer(OfferGrouping{landing_page_group_index: lander_index, index_in_landing_page: offer_index, offer}))
                    selected=offer_option.clone()
                />
            })
        }
    }
    
    pub fn render_lander_pairs(&self) -> VNode {
        let mut nodes =VList::new();
    
        let pre_lander = self.pre_landing_page.clone().unwrap();
        let num_of_pairs = pre_lander.number_of_calls_to_action;
    
        for (group_idx,landing_page_group) in self.pairs.iter().enumerate() {
            let selected_lander=landing_page_group.landing_page.clone();
            let group_number = group_idx as u8 + 1;
        
            let mut offer_nodes=VList::new();
            if let Some(landing_page)= &landing_page_group.landing_page {
                let num_of_offers= landing_page.number_of_calls_to_action;
            
                for (offer_pos_in_lander, option_offer) in landing_page_group.offer.iter().enumerate() {
                
                    offer_nodes.push(html!{
                            <div class="uk-margin-small">
                                {label!("g", format!("Offer {} of {}", offer_pos_in_lander+1,num_of_offers))}
                                {self.offer_or_drop(option_offer, group_idx, offer_pos_in_lander)}
                            </div>
                        })
                }
            }
        
            nodes.push(html! {
                <div class="uk-margin">
                    <div class="uk-margin-small">
                        {label!("o", format!("Pair {} of {}", group_number.to_string(), num_of_pairs))}
                        {label!("g", "Landing Page")}
                        <LandingPageDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(move |lp:LandingPage| Msg::SelectLandingPage(LandingPageGrouping{group_index: group_idx, landing_page: lp})) selected=selected_lander />
                    </div>
                    
                    {offer_nodes}
                </div>
                })
        }
        
        VNode::from(nodes)
    }
        
        pub fn all_fields_filled_out(&self) ->bool {
        self.pre_lander_ready() && self.landers_are_ready() && self.offers_are_ready()
    }
    
    pub fn pre_lander_ready(&self) ->bool {
        self.pre_landing_page.is_some()
    }
    
    pub fn landers_are_ready(&self) ->bool {
        for pair in self.pairs.iter() {
            if pair.landing_page.is_none() {
                return false;
            }
        }
        true
    }
    
    pub fn offers_are_ready(&self) ->bool {
        for pair in self.pairs.iter() {
            for offer in pair.offer.iter() {
                if offer.is_none() {
                    return false;
                }
            }
        }
        true
    }
}
