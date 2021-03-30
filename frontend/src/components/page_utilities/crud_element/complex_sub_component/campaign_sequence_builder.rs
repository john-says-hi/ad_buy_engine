use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::landing_page_selector::LandingPageSelector;
use crate::components::page_utilities::crud_element::complex_sub_component::listicle_builder::ListicleBuilder;
use crate::components::page_utilities::crud_element::complex_sub_component::offer_selector::OfferSelector;
use crate::components::page_utilities::crud_element::complex_sub_component::rhs_funnel_view_basic::RHSFunnelViewBasic;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::referrer_handling_dropdown::ReferrerHandlingDropdown;
use crate::components::page_utilities::crud_element::dropdowns::sequence_type_dropdown::SequenceTypeDropdown;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::elements::landing_page::{LandingPage, WeightedLandingPage};
use ad_buy_engine::data::elements::offer::WeightedOffer;
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
use ad_buy_engine::Country;
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;
use uuid::Uuid;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};

use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    UpdateSequenceType(SequenceType),
    UpdateOffers(Vec<Vec<WeightedOffer>>),
    UpdateLandingPages(Vec<WeightedLandingPage>),
    UpdateSequence(Sequence),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub update_sequence: Callback<Sequence>,
    pub restored_sequence: Option<Sequence>,
}

pub struct CampaignSequenceBuilder {
    link: ComponentLink<Self>,
    props: Props,
    sequence: Sequence,
}

impl Component for CampaignSequenceBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let sequence = if let Some(seq) = props.restored_sequence.clone() {
            seq
        } else {
            Sequence::default()
        };

        Self {
            link,
            props,
            sequence,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateSequence(seq) => {
                self.sequence = seq;
                self.props.update_sequence.emit(self.sequence.clone())
            }

            Msg::UpdateLandingPages(lps) => {
                self.sequence.landing_pages = lps;
                self.equalize_offer_groups();
                self.props.update_sequence.emit(self.sequence.clone());
            }

            Msg::UpdateOffers(offers) => {
                self.sequence.offers = offers;
                self.props.update_sequence.emit(self.sequence.clone());
            }

            Msg::UpdateSequenceType(seq_type) => {
                self.sequence.sequence_type = seq_type;
                self.props.update_sequence.emit(self.sequence.clone());
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(seq) = props.restored_sequence.clone() {
            self.sequence = seq;
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                {self.sequence_type()}
                                {self.render_view()}
        </>
        }
    }
}

impl CampaignSequenceBuilder {
    pub fn equalize_offer_groups(&mut self) {
        let current_len = self.sequence.offers.len();
        let mut highest_len = current_len;

        self.sequence.landing_pages.iter().map(|s| {
            if s.landing_page.number_of_calls_to_action as usize > current_len {
                highest_len = s.landing_page.number_of_calls_to_action as usize;
            }
        });

        notify_danger(format!("Current: {} Highest: {}", current_len, highest_len).as_str());

        if current_len < highest_len {
            let difference_to_add = highest_len - current_len;
            for new_group in 1..difference_to_add {
                notify_danger(format!("New Group usize: {}", new_group).as_str());
                self.sequence.offers.push(vec![])
            }
        } else {
            let difference_to_subtract = current_len - highest_len;
            for rm_group in (1..difference_to_subtract).rev() {
                self.sequence.offers.remove(rm_group);
            }
        }
    }

    pub fn sequence_type(&self) -> VNode {
        let mut oc = "uk-button uk-button-small".to_string();
        let mut loc = "uk-button uk-button-small".to_string();
        let mut lc = "uk-button uk-button-small".to_string();

        match self.sequence.sequence_type {
            SequenceType::OffersOnly => oc.push_str(" uk-button-success"),
            SequenceType::LandingPageAndOffers => loc.push_str(" uk-button-success"),
            SequenceType::Listicle => lc.push_str(" uk-button-success"),
        }

        html! {
            <div class="uk-flex uk-flex-left uk-margin-small">
                    <div class="uk-margin-small">
                        {label!("Sequence Type")}
                        <div uk-switcher="">
                            <button class=oc onclick=callback!(self, |_| Msg::UpdateSequenceType(SequenceType::OffersOnly))>{"Offers Only"}</button>
                            <button class=loc onclick=callback!(self, |_| Msg::UpdateSequenceType(SequenceType::LandingPageAndOffers))>{"Landing Pages & Offers"}</button>
                            <button class=lc onclick=callback!(self, |_| Msg::UpdateSequenceType(SequenceType::Listicle))>{"Listicle"}</button>
                        </div>
                    </div>
            </div>
        }
    }

    pub fn render_view(&self) -> VNode {
        match self.sequence.sequence_type {
            SequenceType::OffersOnly => {
                notify_danger("o");
                let offers = self.sequence.offers.clone();

                VNode::from(html! {
                    <OfferSelector offers=offers state=Rc::clone(&self.props.state) eject_selected_offers=self.link.callback(Msg::UpdateOffers) />
                })
            }

            SequenceType::LandingPageAndOffers => {
                notify_danger("lp o");
                let offers = self.sequence.offers.clone();
                let landers = self.sequence.landing_pages.clone();

                VNode::from(html! {
                <>
                    <LandingPageSelector landers=landers state=Rc::clone(&self.props.state) eject_selected_landing_pages=self.link.callback(Msg::UpdateLandingPages) />
                    <OfferSelector offers=offers state=Rc::clone(&self.props.state) eject_selected_offers=self.link.callback(Msg::UpdateOffers) />
                </>
                })
            }

            SequenceType::Listicle => {
                notify_danger("l");
                let psp = self.sequence.pre_landing_page.clone();
                let pairs = self.sequence.listicle_pairs.clone();

                VNode::from(html! {
                    <ListicleBuilder psp=psp pairs=pairs state=Rc::clone(&self.props.state) eject_listicle=self.link.callback(Msg::UpdateSequence) active_sequence=self.sequence.clone() />
                })
            }
        }
    }
}
