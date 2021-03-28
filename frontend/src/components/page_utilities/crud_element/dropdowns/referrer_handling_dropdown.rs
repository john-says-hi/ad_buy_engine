use crate::appstate::app_state::{AppState, STATE};
use crate::components::tab_state::ActivatedTab;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use crate::RootComponent;
use ad_buy_engine::data::custom_events::CustomConversionEvent;
use ad_buy_engine::data::elements::crud::CreatableElement;
use ad_buy_engine::data::elements::traffic_source::traffic_source_params::ExternalIDParameter;
use ad_buy_engine::data::lists::referrer_handling::{ReferrerHandling, ReplaceReferrerList};
use ad_buy_engine::data::lists::Currency;
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;
use url::Url;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VList;
use yew_material::MatTextField;
use yew_material::{MatListItem, MatSelect};
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    SelectList(ReplaceReferrerList),
    Select(ReferrerHandling),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<ReferrerHandling>,
    #[prop_or_default]
    pub selected: Option<ReferrerHandling>,
    pub state: STATE,
}

pub struct ReferrerHandlingDropdown {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub list_name: String,
    pub selected_list: Option<ReplaceReferrerList>,
    pub referrer_replace_list_active: bool,
    pub available_lists: Vec<ReplaceReferrerList>,
}

impl Component for ReferrerHandlingDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let list_name = if let Some(ReferrerHandling::Replace(list)) = &props.selected {
            list.name_of_list.clone()
        } else {
            "".to_string()
        };

        let available_lists = props
            .state
            .borrow()
            .account
            .borrow()
            .referrer_handling_list
            .clone();

        Self {
            link,
            props,
            list_name,
            referrer_replace_list_active: false,
            selected_list: None,
            available_lists,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectList(list) => {
                self.list_name = list.name_of_list.clone();
                self.selected_list = Some(list.clone());
                self.props.callback.emit(ReferrerHandling::Replace(list));
            }

            Msg::Select(data) => {
                if let ReferrerHandling::Replace(list) = &data {
                    self.referrer_replace_list_active = true;
                } else {
                    self.referrer_replace_list_active = false;
                }
                self.props.callback.emit(data)
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let referrer_handling = if let Some(rh) = self.props.selected.clone() {
            rh
        } else {
            ReferrerHandling::DoNothing
        };

        let mut dn = "uk-button uk-button-small".to_string();
        let mut ra = "uk-button uk-button-small".to_string();
        let mut r = "uk-button uk-button-small".to_string();

        match referrer_handling {
            ReferrerHandling::DoNothing => dn.push_str(" uk-button-success"),
            ReferrerHandling::RemoveAll => ra.push_str(" uk-button-success"),
            ReferrerHandling::Replace(_) => r.push_str(" uk-button-success"),
        }

        let mut component = VList::new();
        let def_rep_list = ReplaceReferrerList::default(); //todo needs to handle replacing default list
        component.push(html! {
            <div class="uk-flex uk-flex-left uk-margin-small">
                    <div class="uk-margin-small">
                        {label!("Referrer Handling")}
                        <div uk-switcher="">
                            <button class=dn onclick=callback!(self, |_| Msg::Select(ReferrerHandling::DoNothing))>{"Do Nothing"}</button>
                            <button class=ra onclick=callback!(self, |_| Msg::Select(ReferrerHandling::RemoveAll))>{"Remove All"}</button>
                            <button class=r onclick=callback!(self, move |_| Msg::Select(ReferrerHandling::Replace(def_rep_list.clone())))>{"Replace"}</button>
                        </div>
                    </div>
            </div>
        });

        // let mut options = VList::new();
        //
        // if let Some(rh) = self.props.selected.clone() {
        //     let rh_clone = rh.clone();
        //     options.push(html!{<option onclick=self.link.callback(move |_| Msg::Select(rh.clone())) >{rh_clone.to_string()}</option>});
        //
        //     for referrer_handling_option in ReferrerHandling::iter().filter(|s| s != &rh_clone) {
        //         let referrer_handling_option_clone = referrer_handling_option.clone();
        //         options.push(html! {<option onclick=self.link.callback(move |_| Msg::Select(referrer_handling_option_clone.clone())) >{referrer_handling_option.to_string()}</option>})
        //     }
        // } else {
        //     for referrer_handling_option in ReferrerHandling::iter() {
        //         let referrer_handling_option_clone = referrer_handling_option.clone();
        //         options.push(html! {<option onclick=self.link.callback(move |_| Msg::Select(referrer_handling_option_clone.clone())) >{referrer_handling_option.to_string()}</option>})
        //     }
        // }

        let referrer_list_select = if self.referrer_replace_list_active {
            let mut list_options = VList::new();

            if self.available_lists.is_empty() {
                list_options
                    .push(html! {<option >{"No Lists Created, Please Make a List "}</option>});
            }

            list_options.push(html! {<option >{"Select a Referrer List"}</option>});
            for list in self.available_lists.iter().cloned() {
                let list_name = list.name_of_list.clone();

                list_options.push(html! {<option onclick=self.link.callback(move |_| Msg::SelectList(list.clone())) >{list_name}</option>});
            }

            html! {
            <div class="uk-margin">
                <select class="uk-select">
                    {list_options}
                </select>
            </div>
            }
        } else {
            html! {}
        };

        html! {
        <div class="">
            {component}
            {referrer_list_select}
        </div>
                            }
    }
}
