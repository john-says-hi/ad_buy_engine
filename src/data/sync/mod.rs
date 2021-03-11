use crate::AError;
use chrono::{Local, NaiveDateTime};
use serde_json::Error;
use std::str::FromStr;

pub mod sync_update;
pub mod visit_sync;

#[derive(Deserialize, Serialize, Clone)]
pub struct SyncHistoryLedger {
    pub elements_last_synced: i64,
    pub visits_last_synced: i64,
    pub account_last_synced: i64,
}

impl FromStr for SyncHistoryLedger {
    type Err = AError;
    
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

impl ToString for SyncHistoryLedger {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl SyncHistoryLedger {
    pub fn update_account_update_date(&mut self) {
        self.account_last_synced = Local::now().timestamp();
    }
}

impl Default for SyncHistoryLedger {
    fn default() -> Self {
        Self {
            elements_last_synced: Local::now().timestamp(),
            visits_last_synced: Local::now().timestamp(),
            account_last_synced: Local::now().timestamp(),
        }
    }
}
