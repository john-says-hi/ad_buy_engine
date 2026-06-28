use crate::components::tab_state::{ActivatedTab, TabState};
use crate::utils::routes::AppRoute;
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::funnel::Funnel;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::offer::Offer;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use ad_buy_engine::data::sync::SyncHistoryLedger;
use ad_buy_engine::data::visit::Visit;
use chrono::NaiveDateTime;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use yew::format::Json;
use yew_services::storage::Area;
use yew_services::StorageService;

pub type STATE = Rc<RefCell<AppState>>;

use crate::appstate::lists::{FilterElementOptions, ReportDateRange, RowLimitOptions};
use crate::appstate::selected::SelectedElement;
use crate::components::account_tab_section::custom_conversions::modal::ModalType;
use either::Either;
use url::Url;
pub use yew::format::Text;

#[derive(Deserialize, Serialize, Clone)]
pub struct AppState {
    pub tab_state: RefCell<TabState>,
    pub account: RefCell<Account>,
    pub sync_ledger: RefCell<SyncHistoryLedger>,
    pub offer_sources: RefCell<Vec<OfferSource>>,
    pub offers: RefCell<Vec<Offer>>,
    pub landing_pages: RefCell<Vec<LandingPage>>,
    pub funnels: RefCell<Vec<Funnel>>,
    pub traffic_sources: RefCell<Vec<TrafficSource>>,
    pub campaigns: RefCell<Vec<Campaign>>,
    pub newest_visit_date: RefCell<i64>,
    pub selected_elements: RefCell<Vec<SelectedElement>>,
    pub crud_modal_type: RefCell<ModalType>,
}

impl AppState {
    pub fn return_all_tracking_urls_no_filter(&self) -> Vec<Url> {
        let state = self
            .account
            .borrow()
            .domains_configuration
            .return_all_tracking_urls_no_filter();
        state
    }

    pub fn return_app_route(&self) -> AppRoute {
        let tab_state = &*self.tab_state.borrow();
        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                main_tab.app_route.clone()
            }
            ActivatedTab::ReportTabState(tab) => tab.app_route.clone(),
        }
    }

    pub fn return_date_range_text(&self) -> String {
        let tab_state = &*self.tab_state.borrow();
        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                main_tab.return_date_range_text()
            }
            ActivatedTab::ReportTabState(tab) => tab.return_date_range_text(),
        }
    }
    pub fn return_filter_option_text(&self) -> String {
        let tab_state = &*self.tab_state.borrow();
        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                main_tab.return_filter_option_text()
            }
            ActivatedTab::ReportTabState(tab) => tab.return_filter_option_text(),
        }
    }

    pub fn return_row_limit_text(&self) -> String {
        let tab_state = &*self.tab_state.borrow();
        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                main_tab.return_row_limit_text()
            }
            ActivatedTab::ReportTabState(tab) => tab.return_row_limit_text(),
        }
    }
    pub fn set_date_range(&self, new: ReportDateRange) {
        let tab_state = &*self.tab_state.borrow();
        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let mut main_tab = tab_state.main_tab.borrow_mut();
                main_tab.set_date_range_and_save_to_browser(new)
            }
            ActivatedTab::ReportTabState(mut tab) => tab.set_date_range_and_save_to_browser(new),
        }
    }

    pub fn set_filter_options(&self, new: FilterElementOptions) {
        let tab_state = &*self.tab_state.borrow();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let mut main_tab = tab_state.main_tab.borrow_mut();
                main_tab.set_filter_option_and_save_to_browser(new)
            }
            ActivatedTab::ReportTabState(mut tab) => tab.set_filter_option_and_save_to_browser(new),
        }
    }

    pub fn set_row_limit(&self, new: RowLimitOptions) {
        let tab_state = &*self.tab_state.borrow();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let mut main_tab = tab_state.main_tab.borrow_mut();
                main_tab.set_row_limit_and_save_to_browser(new)
            }
            ActivatedTab::ReportTabState(mut tab) => tab.set_row_limit_and_save_to_browser(new),
        }
    }

    pub fn set_app_route_and_save_to_browser(&self, route: AppRoute) {
        let tab_state = &*self.tab_state.borrow();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let state = self.tab_state.borrow();
                let mut main_tab = &mut *state.main_tab.borrow_mut();
                main_tab.set_app_route_and_save_to_browser(route)
            }
            ActivatedTab::ReportTabState(mut tab) => tab.set_app_route_and_save_to_browser(route),
        }
    }

    pub fn should_render_third_column(&self) -> bool {
        let tab_state = &*self.tab_state.borrow();
        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => tab_state.main_tab.borrow().should_render_third_column(),
            ActivatedTab::ReportTabState(report_tab) => report_tab.should_render_third_column(),
        }
    }

    pub fn return_selected_tab_index_for_page_controller(&self) -> u32 {
        let tab_state = &*self.tab_state.borrow();
        let activated_tab = &*tab_state.active_tab.borrow();

        match activated_tab {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                main_tab.return_selected_tab_index_for_page_controller()
            }
            ActivatedTab::ReportTabState(report_tab) => {
                report_tab.return_selected_tab_index_for_page_controller()
            }
        }
    }
}
