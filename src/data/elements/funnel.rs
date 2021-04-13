use crate::data::elements::campaign::CampaignDestinationType;
use crate::data::elements::landing_page::LandingPage;
use crate::data::elements::matrix::Matrix;
use crate::data::elements::offer::Offer;
use crate::data::lists::click_transition_method::RedirectOption;
use crate::data::lists::condition::Condition;
use crate::data::lists::referrer_handling::ReferrerHandling;
use crate::data::lists::{DataURLToken, Language, Vertical};
use crate::data::work_space::{Clearance, WorkSpace};
use crate::{AError, Country};
use chrono::{DateTime, NaiveDateTime, Utc};
use either::Either;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use strum::IntoEnumIterator;
use traversal;
use traversal::DftLongestPaths;
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
    #[strum(serialize = "Matrix")]
    Matrix,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConditionalSequence {
    pub id: Uuid,
    pub name: String,
    pub condition_set: Vec<Condition>,
    pub sequences: Vec<Sequence>,
    pub conditional_sequence_is_active: bool,
}

// pub type OfferGroup = Vec<Offer>;
// pub type OfferGroups = Vec<OfferGroup>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sequence {
    pub id: Uuid,
    pub name: String,
    pub weight: u8,
    pub sequence_type: SequenceType,
    pub redirect_option: RedirectOption,
    pub referrer_handling: ReferrerHandling,
    pub matrix: Arc<RwLock<Matrix>>,
    pub weight_optimization_active: bool,
    pub sequence_is_active: bool,
}

impl Sequence {
    // pub fn equalize_landing_page_groups_for_pre_landing_page(&mut self) {
    //     let current_len = self.landing_pages.len();
    //     let highest_len = self.highest_cta_in_pre_landing_page_group();
    //
    //     if current_len < highest_len {
    //         let difference_to_add = highest_len - current_len;
    //         for new_group in 1..difference_to_add {
    //             self.landing_pages.push(vec![])
    //         }
    //     } else {
    //         let difference_to_subtract = current_len - highest_len;
    //         for rm_group in (1..difference_to_subtract).rev() {
    //             self.landing_pages.remove(rm_group);
    //         }
    //     }
    // }
    //
    // pub fn equalize_offer_groups_for_landing_page_group(&mut self, group_pos: usize) {
    //     let current_len = self.offers.get(group_pos).expect("%Gsdfg").len();
    //
    //     let highest_len = self.highest_cta_in_landing_page_group(group_pos);
    //
    //     if current_len < highest_len {
    //         let difference_to_add = highest_len - current_len;
    //         for new_group in 1..difference_to_add {
    //             self.offers.get(group_pos).expect("34t").push(vec![]);
    //         }
    //     } else {
    //         let difference_to_subtract = current_len - highest_len;
    //         for rm_group in (1..difference_to_subtract).rev() {
    //             self.offers
    //                 .get(group_pos)
    //                 .expect("G$%sdfgg")
    //                 .remove(rm_group);
    //         }
    //     }
    // }
    //
    // pub fn highest_cta_in_landing_page_group(&mut self, landing_page_group_pos: usize) -> usize {
    //     let mut start = 0usize;
    //
    //     self.landing_pages
    //         .get(landing_page_group_pos)
    //         .expect("G%sdf")
    //         .iter()
    //         .map(|s| {
    //             if s.landing_page.number_of_calls_to_action as usize > start {
    //                 start = s.landing_page.number_of_calls_to_action as usize;
    //             }
    //         });
    //
    //     start
    // }
    //
    // pub fn highest_cta_in_pre_landing_page_group(&mut self) -> usize {
    //     let mut start = 0usize;
    //
    //     self.pre_landing_page.iter().map(|s| {
    //         if s.landing_page.number_of_calls_to_action as usize > start {
    //             start = s.landing_page.number_of_calls_to_action as usize;
    //         }
    //     });
    //
    //     start
    // }
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
            matrix: Matrix::source(),
            weight_optimization_active: false,
            sequence_is_active: false,
        }
    }
}
