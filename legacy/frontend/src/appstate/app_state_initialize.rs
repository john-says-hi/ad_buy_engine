use crate::appstate::app_state::AppState;
use crate::components::account_tab_section::custom_conversions::modal::ModalType;
use crate::components::tab_state::TabState;
use crate::{notify_danger, notify_primary};
use ad_buy_engine::constant::browser_storage_keys::{
    ACCOUNT_KEY, CAMPAIGNS_KEY, FUNNELS_KEY, LANDING_PAGES_KEY, NEWEST_VISIT_DATE, OFFERS_KEY,
    OFFER_SOURCES, SYNC_HISTORY_KEY, TRAFFIC_SOURCES_KEY,
};
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::funnel::Funnel;
use ad_buy_engine::data::elements::landing_page::LandingPage;
use ad_buy_engine::data::elements::offer::Offer;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::data::elements::traffic_source::TrafficSource;
use ad_buy_engine::data::sync::SyncHistoryLedger;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::AError;
use chrono::{Local, NaiveDateTime, Utc};
use std::cell::RefCell;
use std::sync::atomic::Ordering::AcqRel;
use std::sync::{Arc, RwLock};
use yew::format::Json;
use yew_services::storage::Area;
use yew_services::StorageService;

impl AppState {
    pub fn init() -> Self {
        Self {
            tab_state: RefCell::new(TabState::default()),
            account: RefCell::new(account_restore_or_default()),
            sync_ledger: RefCell::new(sync_history_ledger_restore_or_default()),
            offer_sources: RefCell::new(offer_sources_restore_or_default()),
            offers: RefCell::new(offers_restore_or_default()),
            landing_pages: RefCell::new(landing_pages_restore_or_default()),
            funnels: RefCell::new(funnels_restore_or_default()),
            traffic_sources: RefCell::new(traffic_sources_restore_or_default()),
            campaigns: RefCell::new(campaigns_restore_or_default()),
            newest_visit_date: RefCell::new(newest_visit_date_restore_or_default()),
            selected_elements: RefCell::new(vec![]),
            crud_modal_type: RefCell::new(ModalType::Create),
        }
    }

    pub fn store_account(&self) {
        let data = &*self.account.borrow();
        StorageService::new(Area::Local)
            .expect("f43sa")
            .store(ACCOUNT_KEY, Json(&data))
    }

    pub fn store_sync_ledger(&self) {
        let data = &*self.sync_ledger.borrow();
        StorageService::new(Area::Local)
            .expect("f43sa")
            .store(SYNC_HISTORY_KEY, Json(&data))
    }
    //
    // pub fn store_offer_sources(&self) {
    //     notify_primary("storing sour e");
    //     let data = &*self.offer_sources.borrow();
    //     StorageService::new(Area::Local)
    //         .expect("f43sa")
    //         .store(OFFER_SOURCES, Json(data))
    // }
    //
    // pub fn store_offers(&self) {
    //     let data = self.offers.borrow().clone();
    //     StorageService::new(Area::Local)
    //         .expect("f43sa")
    //         .store(OFFERS_KEY, Json(data))
    // }
    //
    // pub fn store_landing_pages(&self) {
    //     let data = &*self.landing_pages.borrow();
    //     StorageService::new(Area::Local)
    //         .expect("f43sa")
    //         .store(LANDING_PAGES_KEY, Json(data))
    // }
    //
    // pub fn store_funnels(&self) {
    //     let data = &*self.funnels.borrow();
    //     StorageService::new(Area::Local)
    //         .expect("f43sa")
    //         .store(FUNNELS_KEY, Json(data))
    // }
    //
    // pub fn store_campaigns(&self) {
    //     let data = &*self.campaigns.borrow();
    //     StorageService::new(Area::Local)
    //         .expect("f43sa")
    //         .store(CAMPAIGNS_KEY, Json(data))
    // }
    //
    // pub fn store_traffic_sources(&self) {
    //     let data = &*self.traffic_sources.borrow();
    //     StorageService::new(Area::Local)
    //         .expect("f43sa")
    //         .store(TRAFFIC_SOURCES_KEY, Json(data))
    // }

    pub fn store_newest_visit_date(&self) {
        let data = &*self.newest_visit_date.borrow();
        StorageService::new(Area::Local)
            .expect("f43sa")
            .store(NEWEST_VISIT_DATE, Json(data))
    }
}

fn newest_visit_date_restore_or_default() -> i64 {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(NEWEST_VISIT_DATE)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(NEWEST_VISIT_DATE);
        Utc::now().timestamp()
    }
}

fn account_restore_or_default() -> Account {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(ACCOUNT_KEY)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(ACCOUNT_KEY);
        notify_danger("Initializing New Account...");
        Account::default()
    }
}

fn sync_history_ledger_restore_or_default() -> SyncHistoryLedger {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(SYNC_HISTORY_KEY)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(SYNC_HISTORY_KEY);
        SyncHistoryLedger::default()
    }
}

fn offers_restore_or_default() -> Vec<Offer> {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(OFFERS_KEY)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(OFFERS_KEY);
        vec![]
    }
}

fn offer_sources_restore_or_default() -> Vec<OfferSource> {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(OFFER_SOURCES)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(OFFER_SOURCES);
        vec![]
    }
}

fn landing_pages_restore_or_default() -> Vec<LandingPage> {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(LANDING_PAGES_KEY)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(LANDING_PAGES_KEY);
        vec![]
    }
}

fn funnels_restore_or_default() -> Vec<Funnel> {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(FUNNELS_KEY)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(FUNNELS_KEY);
        vec![]
    }
}

fn traffic_sources_restore_or_default() -> Vec<TrafficSource> {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(TRAFFIC_SOURCES_KEY)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(TRAFFIC_SOURCES_KEY);
        vec![]
    }
}

fn campaigns_restore_or_default() -> Vec<Campaign> {
    if let Json(Ok(data)) = StorageService::new(Area::Local)
        .expect("F43rsdfg")
        .restore(CAMPAIGNS_KEY)
    {
        data
    } else {
        StorageService::new(Area::Local)
            .expect("")
            .remove(CAMPAIGNS_KEY);
        vec![]
    }
}
