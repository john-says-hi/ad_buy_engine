use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::country_dropdown::CountryDropdown;
use crate::components::page_utilities::crud_element::dropdowns::referrer_handling_dropdown::ReferrerHandlingDropdown;
use crate::components::page_utilities::crud_element::notes::NotesComponent;
use crate::notify_danger;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
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

pub enum Msg {
    UpdateCountry(Country),
    UpdateFunnelName(InputData),
    UpdateDefaultReferrerHandling(ReferrerHandling),
    UpdateNotes(InputData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub funnel_name: String,
    pub funnel_country: Country,
    pub default_referrer_handling: ReferrerHandling,
    pub notes: String,
    pub update_name: Callback<String>,
    pub update_country: Callback<Country>,
    pub update_referrer_handling: Callback<ReferrerHandling>,
    pub update_notes: Callback<InputData>,
}

pub struct RHSFunnelViewBasic {
    link: ComponentLink<Self>,
    props: Props,
    weight: String,
    name: String,
}

impl Component for RHSFunnelViewBasic {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let name = props.funnel_name.clone();

        Self {
            link,
            props,
            weight: "".to_string(),
            name,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateFunnelName(i) => self.props.update_name.emit(i.value),
            Msg::UpdateCountry(country) => self.props.update_country.emit(country),
            Msg::UpdateDefaultReferrerHandling(rh) => self.props.update_referrer_handling.emit(rh),
            Msg::UpdateNotes(i) => self.props.update_notes.emit(i),
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.name = props.funnel_name.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                <div class="uk-margin">
                                    <h2 class="uk-flex-left">{"Funnel Setup"}</h2>
                                </div>

                                <div class="uk-margin">
                                    <CountryDropdown selected=Some(self.props.funnel_country) eject=self.link.callback(Msg::UpdateCountry) label="Country".to_string() />
                                    <div class="uk-margin">{label!("Name")}<input type="text" class="uk-input" value=&self.name oninput=self.link.callback(Msg::UpdateFunnelName) /></div>
                                </div>

                                <div class="uk-margin">
                                    <h4 class="uk-flex-left">{format!("{} - {}", self.props.funnel_country.to_string(), self.props.funnel_name)}</h4>
                                </div>

                                <div class="uk-margin">
                                    <ReferrerHandlingDropdown callback=self.link.callback(Msg::UpdateDefaultReferrerHandling) selected=&self.props.default_referrer_handling state=Rc::clone(&self.props.state) />
                                </div>

                                <div class="">
                                    {label!("Notes")}<NotesComponent callback=self.link.callback(Msg::UpdateNotes) value=&self.props.notes />
                                </div>

        </>
        }
    }
}
