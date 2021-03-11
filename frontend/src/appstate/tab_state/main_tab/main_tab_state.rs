use crate::appstate::column::PrimeGroupingColumns;
use crate::appstate::lists::{
    FilterElementOptions, PrimeElement, ReportDateRange, RowLimitOptions,
};
use crate::components::data_table::data_state_logic_models::table_state::TableData;
use crate::utils::routes::AppRoute;
use ad_buy_engine::constant::browser_storage_keys::MAIN_TAB_RESTORE_KEY;
use uuid::Uuid;
use yew::format::Json;
use yew_services::storage::Area;
use yew_services::StorageService;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct MainTabState {
    pub report_date_range: ReportDateRange,
    pub filter_options: FilterElementOptions,
    pub row_limit: RowLimitOptions,
    pub prime_grouping_columns: PrimeGroupingColumns,
    pub app_route: AppRoute,
}

impl MainTabState {
    pub fn return_date_range_text(&self) -> String {
        self.report_date_range.to_string()
    }
    pub fn return_filter_option_text(&self) -> String {
        self.filter_options.to_string()
    }
    pub fn return_row_limit_text(&self) -> String {
        self.row_limit.to_string()
    }

    pub fn set_app_route(&mut self, new: AppRoute) {
        self.app_route = new;
    }

    pub fn set_app_route_and_save_to_browser(&mut self, new: AppRoute) {
        self.app_route = new;
        self.store_to_browser()
    }

    pub fn set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
        &mut self,
        new: PrimeElement,
    ) {
        let route: AppRoute = new.into();
        self.reset_second_and_third_columns();
        self.prime_grouping_columns.first_column = new;
        self.app_route = route;
        self.store_to_browser()
    }

    pub fn set_second_prime_column(&mut self, new: PrimeElement) {
        self.prime_grouping_columns.second_column = new;
    }

    pub fn set_third_prime_column(&mut self, new: PrimeElement) {
        self.prime_grouping_columns.third_column = new;
    }

    pub fn set_row_limit_and_save_to_browser(&mut self, new: RowLimitOptions) {
        self.row_limit = new;
        self.store_to_browser()
    }

    pub fn set_filter_option_and_save_to_browser(&mut self, new: FilterElementOptions) {
        self.filter_options = new;
        self.store_to_browser()
    }

    pub fn set_date_range_and_save_to_browser(&mut self, new: ReportDateRange) {
        self.report_date_range = new;
        self.store_to_browser()
    }

    pub fn store_to_browser(&mut self) {
        if self.app_route == AppRoute::FourZeroFour {
            self.app_route = AppRoute::Campaign
        }
        StorageService::new(Area::Local)
            .expect("G453s")
            .store(MAIN_TAB_RESTORE_KEY, Json(&self))
    }

    pub fn restore_from_browser() -> Self {
        if let Json(Ok(main_tab)) = StorageService::new(Area::Local)
            .expect("$gss")
            .restore(MAIN_TAB_RESTORE_KEY)
        {
            let mut tab_holder: MainTabState = main_tab;
            tab_holder.reset_second_and_third_columns();
            tab_holder
        } else {
            MainTabState::default()
        }
    }

    pub fn reset_second_and_third_columns(&mut self) {
        self.prime_grouping_columns.second_column = PrimeElement::Nothing;
        self.prime_grouping_columns.third_column = PrimeElement::Nothing;
    }

    pub fn should_render_third_column(&self) -> bool {
        self.prime_grouping_columns.second_column != PrimeElement::Nothing
    }

    pub fn return_selected_tab_index_for_page_controller(&self) -> u32 {
        match self.app_route {
            AppRoute::Dashboard => 0,
            AppRoute::Campaign => 1,
            AppRoute::Offers => 2,
            AppRoute::Landers => 3,
            AppRoute::Sequences => 4,
            AppRoute::Funnels => 5,
            AppRoute::Traffic => 6,
            AppRoute::OfferSources => 7,
            _ => 100,
        }
    }
}
