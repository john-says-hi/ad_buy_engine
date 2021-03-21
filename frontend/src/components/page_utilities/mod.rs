pub mod create_element_button;
pub mod crud_element;
pub mod date_range;
pub mod filter_options;
pub mod first_column_grouping;
pub mod row_limit;
pub mod second_column_grouping;
pub mod third_column_grouping;
pub mod update_element;
pub mod pagination;
pub mod search;

use search::Search;
use crate::appstate::app_state::AppState;
use crate::components::page_utilities::create_element_button::NewElement;
use crate::RootComponent;
use date_range::DateRange;
use filter_options::FilterOptions;
use pagination::Pagination;
use first_column_grouping::FirstColumnGrouping;
use row_limit::RowLimit;
use second_column_grouping::SecondColumnGrouping;
use std::cell::RefCell;
use std::rc::Rc;
use third_column_grouping::ThirdColumnGrouping;
use update_element::UpdateElement;
use yew::prelude::*;

pub enum Msg {
    Click,
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct PageUtilities {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for PageUtilities {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
    
    fn view(&self) -> Html {
        html! {
            <>
            <div class="uk-child-width-1-2 uk-grid-collapse uk-background-default" uk-grid="">
            
               <div class="uk-flex uk-flex-left uk-flex-stretch uk-flex-wrap">
                  <FirstColumnGrouping state=Rc::clone(&self.props.state)  />
                  <SecondColumnGrouping state=Rc::clone(&self.props.state) onclick=self.link.callback(|_|Msg::Ignore) />
                  <ThirdColumnGrouping state=Rc::clone(&self.props.state) onclick=self.link.callback(|_|Msg::Ignore) />
               </div>
               
               <div class="uk-flex uk-flex-right uk-flex-wrap">
                  <Search state=state_clone!(self.props.state) />
                  <DateRange state=Rc::clone(&self.props.state) />
                  <div class="uk-margin-right-small"><button class="uk-button uk-button-default uk-button-small uk-background-primary uk-light"><span class="fas fa-retweet uk-margin-small-right"></span>{"Refresh"}</button></div>
                  <div uk-tooltip="title: Not built yet"><button class=" uk-disabled uk-button uk-button-default uk-button-small uk-background-primary uk-light"><span class="fas fa-chart-bar uk-margin-small-right"></span>{"Graph"}</button></div>
               </div>
            </div>
            
            <div class=" uk-grid-collapse uk-background-default" uk-grid="">
            
               <div class="uk-flex uk-flex-left uk-flex-stretch uk-width-1-3">
                <Pagination state=state_clone!(self.props.state) />
               </div>
               
               <div class="uk-flex uk-flex-right uk-flex-wrap uk-width-2-3">
                  <NewElement state=Rc::clone(&self.props.state)/>
                  <div uk-tooltip="title: Not built yet" class="uk-margin-right"><button class="uk-disabled uk-button uk-button-default uk-button-small uk-background-primary uk-light"><span class="fas fa-wallet uk-margin-small-right"></span>{"Update Costs"}</button></div>

                  <UpdateElement state=Rc::clone(&self.props.state) />

                  <div uk-tooltip="title: Not built yet" class="uk-margin-right"><button class="uk-disabled uk-button uk-button-default uk-button-small uk-background-primary uk-light"><span class="fas fa-edit uk-margin-small-right"></span>{"Clone"}</button></div>
                  <div uk-tooltip="title: Not built yet" class="uk-margin-right"><button class="uk-disabled uk-button uk-button-default uk-button-small uk-background-primary uk-light"><span class="fas fa-clone uk-margin-small-right"></span>{"Law"}</button></div>
                  <div uk-tooltip="title: Not built yet" class="uk-margin-right"><button class="uk-disabled uk-button uk-button-default uk-button-small uk-background-primary uk-light"><span class="fas fa-clone uk-margin-small-right"></span>{"Export"}</button></div>
                  <RowLimit state=Rc::clone(&self.props.state) />
                  <FilterOptions state=Rc::clone(&self.props.state) />
               </div>
               
            </div>
            </>
        }
    }
}
