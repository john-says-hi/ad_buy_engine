use crate::data::elements::landing_page::LandingPage;
use crate::data::elements::landing_page::WeightedLandingPage;
use crate::data::elements::offer::{Offer, WeightedOffer};
use crate::data::lists::click_transition_method::RedirectOption;
use crate::data::lists::condition::Condition;
use crate::data::lists::referrer_handling::ReferrerHandling;
use crate::data::lists::{DataURLToken, Language, Vertical};
use crate::data::work_space::{Clearance, WorkSpace};
use crate::{AError, Country};
use chrono::{DateTime, NaiveDateTime, Utc};
use strum::IntoEnumIterator;

use crate::data::elements::campaign::CampaignDestinationType;
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
    pub referrer_handling: ReferrerHandling,
    pub notes: String,
    pub conditional_sequences: Vec<ConditionalSequence>,
    pub default_sequences: Vec<Sequence>,
    pub archived: bool,
    pub last_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, EnumString, ToString, EnumIter, PartialEq)]
pub enum SequenceType {
    #[strum(serialize = "Offers Only")]
    OffersOnly,
    #[strum(serialize = "Landing Pages & Offers")]
    LandingPageAndOffers,
    #[strum(serialize = "Listicle")]
    Listicle,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConditionalSequence {
    pub id: Uuid,
    pub name: String,
    pub condition_set: Vec<Condition>,
    pub sequences: Vec<Sequence>,
    pub conditional_sequence_is_active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sequence {
    pub id: Uuid,
    pub name: String,
    pub weight: u8,
    pub sequence_type: SequenceType,
    pub redirect_option: RedirectOption,
    pub referrer_handling: ReferrerHandling,
    pub pre_landing_page: Option<LandingPage>,
    pub listicle_pairs: Vec<ListiclePair>,
    pub landing_pages: Vec<WeightedLandingPage>,
    pub offers: Vec<Vec<WeightedOffer>>,
    pub weight_optimization_active: bool,
    pub sequence_is_active: bool,
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_string(),
            weight: 0,
            sequence_type: SequenceType::OffersOnly,
            redirect_option: RedirectOption::Redirect,
            referrer_handling: ReferrerHandling::DoNothing,
            pre_landing_page: None,
            listicle_pairs: vec![],
            landing_pages: vec![],
            offers: vec![vec![]],
            weight_optimization_active: false,
            sequence_is_active: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListiclePair {
    pub landing_page: LandingPage,
    pub offer: Vec<Offer>,
}
