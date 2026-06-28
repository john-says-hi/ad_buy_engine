use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::utils::routes::AppRoute;
use std::cell::RefCell;
use std::rc::Rc;

pub fn app_route_matches(route: AppRoute, state: Rc<RefCell<AppState>>) -> bool {
    let state = state.borrow();
    let tab_state = state.tab_state.borrow();

    match tab_state.return_activated_tab() {
        ActivatedTab::MainTab => {
            let main_tab = &*tab_state.main_tab.borrow();
            main_tab.app_route == route
        }
        ActivatedTab::ReportTabState(tab) => tab.app_route == route,
    }
}
