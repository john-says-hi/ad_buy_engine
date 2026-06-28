use crate::appstate::app_state::AppState;
use ad_buy_engine::constant::browser_storage_keys::USER_ACCOUNT_STATE_KEY;
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::funnel::Funnel;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::offer::Offer;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use ad_buy_engine::data::sync::SyncHistoryLedger;
use ad_buy_engine::AError;
use chrono::{Local, NaiveDateTime};
use std::cell::RefCell;
use yew::format::Json;
use yew_services::storage::Area;
use yew_services::StorageService;
// pub fn create(account: Account) -> Self {
//     Self {
//         account: RefCell::new(account),
//         sync_ledger: RefCell::new(SyncHistoryLedger::default()),
//         offer_sources: RefCell::new(vec![]),
//         offers: RefCell::new(vec![]),
//         landing_pages: RefCell::new(vec![]),
//         funnels: RefCell::new(vec![]),
//         traffic_sources: RefCell::new(vec![]),
//         campaigns: RefCell::new(vec![]),
//         newest_visit_date: RefCell::new(Local::now().naive_local()),
//     }
// }
#[derive(Deserialize, Serialize, Clone)]
pub struct UserAccountState {
    pub account: RefCell<Account>,
    pub sync_ledger: RefCell<SyncHistoryLedger>,
    pub offer_sources: RefCell<Vec<OfferSource>>,
    pub offers: RefCell<Vec<Offer>>,
    pub landing_pages: RefCell<Vec<LandingPage>>,
    pub funnels: RefCell<Vec<Funnel>>,
    pub traffic_sources: RefCell<Vec<TrafficSource>>,
    pub campaigns: RefCell<Vec<Campaign>>,
    pub newest_visit_date: RefCell<NaiveDateTime>,
}

impl AppState {
    // pub fn return_last_update_for_account(&self) -> NaiveDateTime {
    //     self.sync_ledger.borrow().account_last_synced
    // }
    //
    // pub fn update_account(&mut self, account: Account) {
    //     *self.account.borrow_mut() = account;
    //     self.sync_ledger
    //         .borrow_mut()
    //         .update_account_date_of_update()
    // }

    // pub fn restore() -> Vec<Self> {
    //     let mut list = vec![];
    //     if let Json(Ok(user_account_state)) = StorageService::new(Area::Local)
    //         .expect("asdf23gg")
    //         .restore(USER_ACCOUNT_STATE_KEY)
    //     {
    //         list.push(user_account_state)
    //     }
    //     list
    // }
    //
    // pub fn store(&self) {
    //     StorageService::new(Area::Local)
    //         .expect("45g3wesg")
    //         .store(USER_ACCOUNT_STATE_KEY, Json(&self))
    // }
}
