use crate::data::elements::landing_page::LandingPage;
use crate::data::elements::landing_page::WeightedLandingPage;
use crate::data::elements::offer::{Offer, WeightedOffer};
use crate::data::lists::click_transition_method::RedirectOption;
use crate::data::lists::condition::Condition;
use crate::data::lists::{DataURLToken, Language, Vertical};
use crate::data::work_space::{Clearance, WorkSpace};
use crate::{AError, Country};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

impl FromStr for Funnel {
    type Err = AError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

impl ToString for Funnel {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Funnel {
    pub funnel_id: Uuid,
    pub account_id: Uuid,
    pub country: Country,
    pub name: String,
    pub clearance: Clearance,
    pub redirect_option: RedirectOption,
    pub notes: String,
    pub conditional_sequence: Vec<ConditionalSequence>,
    pub default_sequence: Vec<WeightedSequence>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SequenceType {
    LandingPageAndOffers,
    OffersOnly,
    OfferWall,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeightedSequence {
    pub weight: u8,
    pub sequence: Sequence,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConditionalSequence {
    pub name: String,
    pub weight: u8,
    pub active: bool,
    pub condition_set: Vec<Condition>,
    pub sequence: Sequence,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sequence {
    pub name: String,
    pub sequence_type: SequenceType,
    pub redirect_option: RedirectOption,
    pub pre_landing_page: Option<LandingPage>,
    pub landing_pages: Vec<WeightedLandingPage>,
    pub offers: Vec<WeightedOffer>,
}
