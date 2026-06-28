use crate::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use crate::data::custom_events::{CustomConversionEvent, CustomConversionEventToken};
use crate::data::lists::referrer_handling::ReferrerHandling;
use crate::data::lists::{Currency, Vertical};
use crate::data::work_space::Clearance;
use crate::AError;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::net::IpAddr;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiveOfferSource {
    pub offer_source_id: Uuid,
    pub append_click_id: bool,
    pub accept_duplicate_post_backs: bool,
    pub whitelisted_postback_ips: WhiteListedPostbackIPs,
    pub referrer_handling: ReferrerHandling,
}

impl FromStr for OfferSource {
    type Err = AError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

impl ToString for OfferSource {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferSource {
    pub offer_source_id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub clearance: Clearance,
    pub click_id_token: String,
    pub payout_token: String,
    pub conversion_id_token: String,
    pub custom_events: Vec<CustomConversionEventToken>,
    pub tracking_domain: Url,
    pub conversion_tracking_method: ConversionTrackingMethod,
    pub include_additional_parameters_in_postback_url: bool,
    pub payout_currency: Currency,
    pub append_click_id: bool,
    pub accept_duplicate_post_backs: bool,
    pub whitelisted_postback_ips: WhiteListedPostbackIPs,
    pub referrer_handling: ReferrerHandling,
    pub notes: String,
    pub archived: bool,
    pub last_updated: DateTime<Utc>,
}

impl From<OfferSource> for LiveOfferSource {
    fn from(offer_source: OfferSource) -> Self {
        Self {
            offer_source_id: offer_source.offer_source_id,
            append_click_id: offer_source.append_click_id,
            accept_duplicate_post_backs: offer_source.accept_duplicate_post_backs,
            whitelisted_postback_ips: offer_source.whitelisted_postback_ips,
            referrer_handling: offer_source.referrer_handling,
        }
    }
}
