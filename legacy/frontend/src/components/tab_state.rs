use crate::appstate::tab_state::main_tab::main_tab_state::MainTabState;
use crate::appstate::tab_state::report_tab::report_tab_state::ReportTabState;
use ad_buy_engine::constant::browser_storage_keys::REPORT_TABS_RESTORE_KEYS;
use std::cell::{RefCell, RefMut};
use uuid::Uuid;
use yew::format::Json;
use yew_services::storage::Area;
use yew_services::StorageService;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum ActivatedTab {
    MainTab,
    ReportTabState(ReportTabState),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TabState {
    pub main_tab: RefCell<MainTabState>,
    pub local_storage_keys: RefCell<Vec<String>>,
    pub report_tabs: RefCell<Vec<ReportTabState>>,
    pub active_tab: RefCell<ActivatedTab>,
}

impl TabState {
    pub fn store_report_tab_to_browser(&self, key: &str) {
        let report_tab = self.find_report_tab_by_key(key).expect("hg54s");
        StorageService::new(Area::Local)
            .expect("$hss")
            .store(REPORT_TABS_RESTORE_KEYS, Json(&report_tab));
    }

    pub fn find_report_tab_by_key(&self, key: &str) -> Option<ReportTabState> {
        let list = &*self.report_tabs.borrow();
        list.iter().find(|t| t.browser_storage_key == key).cloned()
    }

    pub fn store_all_report_tab_keys_to_browser(&self) {
        let keys = &*self.local_storage_keys.borrow();

        StorageService::new(Area::Local)
            .expect("$hss")
            .store(REPORT_TABS_RESTORE_KEYS, Json(&keys));
    }

    pub fn return_activated_tab(&self) -> ActivatedTab {
        let mut reference = &*self.active_tab.borrow();
        reference.clone()
    }

    pub fn default() -> Self {
        let local_storage_keys: Vec<String> = if let Json(Ok(keys)) =
            StorageService::new(Area::Local)
                .expect("VRsdfg")
                .restore(REPORT_TABS_RESTORE_KEYS)
        {
            keys
        } else {
            vec![]
        };

        let mut report_tabs: Vec<ReportTabState> = vec![];
        let keys_to_check = local_storage_keys.clone();

        for key in keys_to_check {
            let str_key = key.as_str();

            if let Json(Ok(mut tab)) = StorageService::new(Area::Local)
                .expect("gv43s")
                .restore(str_key)
            {
                let mut tab_holder: ReportTabState = tab;
                tab_holder.reset_second_and_third_columns();
                report_tabs.push(tab_holder)
            }
        }

        Self {
            main_tab: RefCell::new(MainTabState::restore_from_browser()),
            local_storage_keys: RefCell::new(local_storage_keys),
            report_tabs: RefCell::new(report_tabs),
            active_tab: RefCell::new(ActivatedTab::MainTab),
        }
    }
}
