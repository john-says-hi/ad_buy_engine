use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::routes::AppRoute;

impl AppState {
    pub fn return_third_column_text_value(&self) -> String {
        let tab_state = &*self.tab_state.borrow();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                main_tab.prime_grouping_columns.third_column.to_string()
            }
            ActivatedTab::ReportTabState(report_tab_clone) => report_tab_clone
                .prime_grouping_columns
                .third_column
                .to_string(),
        }
    }

    pub fn return_second_column_text_value(&self) -> String {
        let tab_state = &*self.tab_state.borrow();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                main_tab.prime_grouping_columns.second_column.to_string()
            }
            ActivatedTab::ReportTabState(report_tab_clone) => report_tab_clone
                .prime_grouping_columns
                .second_column
                .to_string(),
        }
    }

    pub fn return_first_column_text_value(&self) -> String {
        let tab_state = &*self.tab_state.borrow();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let main_tab = tab_state.main_tab.borrow();
                main_tab.prime_grouping_columns.first_column.to_string()
            }
            ActivatedTab::ReportTabState(report_tab_clone) => report_tab_clone
                .prime_grouping_columns
                .first_column
                .to_string(),
        }
    }

    pub fn set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(
        &mut self,
        route: AppRoute,
    ) {
        let mut tab_state = &mut *self.tab_state.borrow_mut();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let mut main_tab = &mut *tab_state.main_tab.borrow_mut();
                main_tab.set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(route.into());
            }
            ActivatedTab::ReportTabState(mut report_tab_clone) => {
                report_tab_clone
                    .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(route.into());
                *tab_state.active_tab.borrow_mut() = ActivatedTab::ReportTabState(report_tab_clone);
            }
        }
    }

    pub fn set_second_prime_column(&mut self, route: AppRoute) {
        let mut tab_state = &mut *self.tab_state.borrow_mut();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let mut main_tab = &mut *tab_state.main_tab.borrow_mut();
                main_tab.set_second_prime_column(route.into());
            }
            ActivatedTab::ReportTabState(mut report_tab_clone) => {
                report_tab_clone.set_second_prime_column(route.into());
                *tab_state.active_tab.borrow_mut() = ActivatedTab::ReportTabState(report_tab_clone)
            }
        }
    }

    pub fn set_third_prime_column(&self, route: AppRoute) {
        let mut tab_state = &*self.tab_state.borrow();

        match tab_state.return_activated_tab() {
            ActivatedTab::MainTab => {
                let mut main_tab = &mut *tab_state.main_tab.borrow_mut();
                main_tab.set_third_prime_column(route.into());
            }
            ActivatedTab::ReportTabState(mut report_tab_clone) => {
                report_tab_clone.set_third_prime_column(route.into());
                *tab_state.active_tab.borrow_mut() = ActivatedTab::ReportTabState(report_tab_clone);
            }
        }
    }
}
