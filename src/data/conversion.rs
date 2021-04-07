use crate::data::elements::offer::{LiveOffer, Offer};
use crate::data::lists::country::Country;
use crate::data::lists::time_zone::TimeZone;
use crate::data::lists::Currency;
use chrono::NaiveDateTime;
use ipnet::IpNet;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PrimalWhiteListedPostbackIPs {
    pub ips: [IpAddr; 32],
    pub ip_nets: [IpNet; 32],
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhiteListedPostbackIPs {
    pub ips: Vec<IpAddr>,
    pub ip_nets: Vec<IpNet>,
}

impl Default for WhiteListedPostbackIPs {
    fn default() -> Self {
        Self {
            ips: vec![],
            ip_nets: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConversionCapConfig {
    pub daily_cap: u32,
    pub time_zone: TimeZone,
    pub redirect_offer: Vec<Offer>,
    pub auto_send_to_top_performing_offer: bool,
    pub match_county: bool,
    pub match_language: bool,
    pub match_vertical: bool,
}

// #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
// pub struct PrimalConversionCapConfig<'a> {
//     pub daily_cap: u32,
//     pub time_zone: TimeZone,
//     pub redirect_offer: [&'a PrimalOffer; 32],
//     pub auto_send_to_top_performing_offer: bool,
//     pub match_county: bool,
//     pub match_language: bool,
//     pub match_vertical: bool,
// }

impl Default for ConversionCapConfig {
    fn default() -> Self {
        Self {
            daily_cap: 0,
            time_zone: TimeZone::UTC,
            redirect_offer: vec![],
            auto_send_to_top_performing_offer: false,
            match_county: false,
            match_language: false,
            match_vertical: false,
        }
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, PartialEq, Debug)]
pub enum ConversionTrackingMethod {
    #[strum(serialize = "Postback URL")]
    PostbackURL,
    // #[strum(serialize = "Tracking Pixel")]
    // TrackingPixel,
    // #[strum(serialize = "Drill Down")]
    // TrackingPixelURL,
    // #[strum(serialize = "Drill Down")]
    // TrackingScript,
}

#[derive(Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, PartialEq, Debug)]
pub enum PayoutType {
    Auto,
    Manual,
}

impl Default for PayoutType {
    fn default() -> Self {
        PayoutType::Auto
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManualPayoutConfig {
    default_currency: Currency,
    is_goe_specific: bool,
    default_payout: Decimal,
    geo_specific_payouts: Vec<ManualGeoPayoutProfile>,
}

// #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
// pub struct PrimalManualPayoutConfig {
//     default_currency: Currency,
//     is_goe_specific: bool,
//     default_payout: Decimal,
//     geo_specific_payouts: [ManualGeoPayoutProfile; 32],
// }

impl Default for ManualPayoutConfig {
    fn default() -> Self {
        Self {
            default_currency: Currency::USD,
            is_goe_specific: false,
            default_payout: Decimal::new(0, 0),
            geo_specific_payouts: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct ManualGeoPayoutProfile {
    country: Country,
    payout: Decimal,
    currency: Currency,
}
