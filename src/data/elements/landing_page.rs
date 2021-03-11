use crate::data::lists::{DataURLToken, Language, Vertical};
use crate::data::work_space::Clearance;
use crate::{AError, Country};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeightedLandingPage {
    pub weight: u8,
    pub landing_page: LandingPage,
}

impl FromStr for LandingPage {
    type Err = AError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

impl ToString for LandingPage {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LandingPage {
    pub landing_page_id: Uuid,
    pub account_id: Uuid,
    pub is_pre_landing_page: bool,
    pub clearance: Clearance,
    pub country: Country,
    pub name: String,
    pub tags: Vec<String>,
    pub url: Url,
    pub url_tokens: Vec<DataURLToken>,
    pub number_of_calls_to_action: u8,
    pub vertical: Vertical,
    pub language: Language,
    pub notes: String,
    pub archived: bool,
    pub last_updated: DateTime<Utc>,
}
