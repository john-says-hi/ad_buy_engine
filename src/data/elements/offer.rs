use crate::data::conversion::{
    ConversionCapConfig, ConversionTrackingMethod, ManualPayoutConfig, PayoutType,
};
use crate::data::elements::offer_source::OfferSource;
use crate::data::lists::{Currency, DataURLToken, Language, Vertical};
use crate::data::work_space::Clearance;
use crate::{AError, Country};
use chrono::{DateTime, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeightedOffer {
    pub weight: u8,
    pub offer: Offer,
}

impl From<Offer> for WeightedOffer {
    fn from(o: Offer) -> Self {
        Self {
            weight: 100,
            offer: o,
        }
    }
}

impl FromStr for Offer {
    type Err = AError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

impl ToString for Offer {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Offer {
    pub offer_id: Uuid,
    pub account_id: Uuid,
    pub clearance: Clearance,
    pub offer_source: OfferSource,
    pub country: Country,
    pub name: String,
    pub tags: Vec<String>,
    pub url: Url,
    pub offer_tokens: Vec<DataURLToken>,
    pub conversion_tracking_method: ConversionTrackingMethod,
    pub payout_type: PayoutType,
    pub manual_payout_config: Option<ManualPayoutConfig>,
    pub conversion_cap_config: Option<ConversionCapConfig>,
    pub payout_value: Decimal,
    pub currency: Currency,
    pub language: Language,
    pub vertical: Vertical,
    pub notes: String,
    pub archived: bool,
    pub last_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiveOffer {
    pub offer_id: Uuid,
    pub account_id: Uuid,
    pub offer_source: OfferSource,
    pub url: Url,
    pub offer_tokens: Vec<DataURLToken>,
    pub conversion_tracking_method: ConversionTrackingMethod,
    pub payout: Decimal,
    pub payout_type: PayoutType,
}
