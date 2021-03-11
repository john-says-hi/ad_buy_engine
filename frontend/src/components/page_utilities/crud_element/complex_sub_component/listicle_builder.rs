use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::funnel_view_basic_data::FunnelViewBasicData;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::landing_page_dropdown::LandingPageDropdown;
use crate::components::page_utilities::crud_element::dropdowns::offer_dropdown::OfferDropdown;
use crate::{notify_danger, notify_warning, notify_primary, notify_debug};
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType, ListiclePair};
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
use crate::components::page_utilities::crud_element::dropdowns::pre_landing_page_dropdown::PreLandingPageDropdown;

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub eject_listicle: Callback<Sequence>,
    pub active_sequence: Sequence,
}

pub struct OfferGrouping {
    pub     landing_page_group_index: usize,
    pub index_in_landing_page: usize,
    pub offer: Offer,
}

pub struct LandingPageGrouping {
    pub group_index: usize,
    pub landing_page: LandingPage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OptionalPair {
    pub landing_page: Option<LandingPage>,
    pub offer: Vec<Option<Offer>>,
}

impl From<OptionalPair> for ListiclePair {
    fn from(optional_pair:OptionalPair)->Self{
        Self{
            landing_page:optional_pair.landing_page.expect("g534s"),
            offer:optional_pair.offer.iter().map(|s| s.clone().expect("Gr34s")).collect::<Vec<Offer>>(),
        }
    }
}

pub enum Msg {
    OnBlur,
    SelectPreLandingPage(LandingPage),
    SelectLandingPage(LandingPageGrouping),
    SelectOffer(OfferGrouping),
}

pub struct ListicleBuilder {
    link: ComponentLink<Self>,
    props: Props,
    pre_landing_page:Option<LandingPage>,
    pairs: Vec<OptionalPair>,
}

impl Component for ListicleBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
  
        
        Self {
            
            link, props,pre_landing_page:None,pairs:vec![] }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            
            Msg::OnBlur=>{
                if self.all_fields_filled_out() {
                    let mut sequence=self.props.active_sequence.clone();
                    sequence.offers.clear();
                    sequence.landing_pages.clear();
                    sequence.pre_landing_page=self.pre_landing_page.clone();
                    sequence.listicle_pairs=self.pairs.iter().cloned().map(|s| s.into()).collect::<Vec<ListiclePair>>();
                    self.props.eject_listicle.emit(sequence);
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
            }
            
            Msg::SelectLandingPage(landing_page_group)=>{
                 if let Some(pair)= self.pairs.get_mut(landing_page_group.group_index) {
                     let num_of_ctas=landing_page_group.landing_page.number_of_calls_to_action;
                     pair.landing_page=Some(landing_page_group.landing_page);
                     
                     let mut offers = vec![];
                     for cta in 1..num_of_ctas {
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
                for pair in 1..lp.number_of_calls_to_action {
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
        let pre_landing_page=props.active_sequence.pre_landing_page.clone();
        
        let mut optional_pairs = vec![];
        
         for pair in props.active_sequence.listicle_pairs.iter() {
             let offer =pair.offer.iter().map(|s| Some(s.clone())).collect::<Vec<Option<Offer>>>();
             
            optional_pairs.push(OptionalPair{
                landing_page:Some(pair.landing_page.clone()),
                offer,
            });
        }
        
        self.pre_landing_page=pre_landing_page;
        self.pairs=optional_pairs;
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                <div class="uk-margin">
                                    <h4>{"Listicle Setup"}</h4>
                                </div>

                                <hr class="uk-divider" />

                                {self.render_pre_lander()}
        </>
        }
    }
}

impl ListicleBuilder {
    pub fn render_pre_lander(&self) -> VNode {
    
        let node = if let Some(pre_lander)= &self.pre_landing_page {
            html!{
            <>
                <div class="uk-margin"><PreLandingPageDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(Msg::SelectPreLandingPage) selected=Some(pre_lander.clone()) /></div>
                {self.render_landers()}
            </>
            }
        } else {
            notify_primary("no pre lp");
            html!{<div class="uk-margin"><PreLandingPageDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(Msg::SelectPreLandingPage) selected=None /></div>}
        };
        
        VNode::from(node)
    }
    
    pub fn render_landers(&self) -> VNode {
        notify_primary("render lps");
        let mut nodes =VList::new();
        
        if let Some(pre_lander)= &self.pre_landing_page {
            notify_danger("has pre lp");
            for (group_idx,landing_page_group) in self.pairs.iter().enumerate() {
                notify_primary("as lp group");
                let selected_lander=landing_page_group.landing_page.clone();
                let group_number = group_idx as u8 + 1;
                
                let mut offer_nodes=VList::new();
                if let Some(landing_page)= &landing_page_group.landing_page {
                    for (offer_pos_in_lander, option_offer) in landing_page_group.offer.iter().enumerate() {
                        
                        offer_nodes.push(html!{
                            <div class="uk-margin-small uk-flex-right" onblur=self.link.callback(|_| Msg::OnBlur) >
                                <OfferDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(move |offer:Offer| Msg::SelectOffer(OfferGrouping{landing_page_group_index: group_idx, index_in_landing_page: offer_pos_in_lander, offer})) selected=option_offer.clone() />
                            </div>
                        })
                    }
                }
                
                nodes.push(html!{
                <div class="uk-margin">
                    <div class="uk-margin-small">
                        {group_number.to_string()} {"---> "}  <LandingPageDropdown state=Rc::clone(&self.props.state) eject=self.link.callback(move |lp:LandingPage| Msg::SelectLandingPage(LandingPageGrouping{group_index: group_idx, landing_page: lp})) selected=selected_lander />
                    </div>
                    
                    {offer_nodes}
                </div>
                })
            }
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
