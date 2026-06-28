use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::funnel::FunnelSequence;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlToken {
    pub name: String,
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationType {
    Funnel,
    DirectSequence,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferSourceDraft {
    pub name: String,
    pub tokens: Vec<UrlToken>,
    pub tracking_domain: String,
    pub tracking_method: String,
    pub payout_currency: String,
    pub postback_url: String,
    pub append_click_id: bool,
    pub accept_duplicate_postbacks: bool,
    pub whitelist_postback_ips: Vec<String>,
    pub referrer_handling: String,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferSource {
    pub id: String,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
    pub archived: bool,
    #[serde(flatten)]
    pub draft: OfferSourceDraft,
}

impl Deref for OfferSource {
    type Target = OfferSourceDraft;

    fn deref(&self) -> &Self::Target {
        &self.draft
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferDraft {
    pub offer_source_id: String,
    pub country: String,
    pub name: String,
    pub tags: Vec<String>,
    pub url: String,
    pub url_tokens: Vec<UrlToken>,
    pub payout_model: String,
    pub payout_value: f64,
    pub currency: String,
    pub language: String,
    pub vertical: String,
    pub weight: u32,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    pub id: String,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
    pub archived: bool,
    #[serde(flatten)]
    pub draft: OfferDraft,
}

impl Deref for Offer {
    type Target = OfferDraft;

    fn deref(&self) -> &Self::Target {
        &self.draft
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LandingPageDraft {
    pub country: String,
    pub name: String,
    pub tags: Vec<String>,
    pub url: String,
    pub url_tokens: Vec<UrlToken>,
    pub cta_count: u8,
    #[serde(default)]
    pub role: LandingPageRole,
    #[serde(default)]
    pub expected_conversion_event_type_ids: Vec<String>,
    pub language: String,
    pub vertical: String,
    pub weight: u32,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LandingPage {
    pub id: String,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
    pub archived: bool,
    #[serde(flatten)]
    pub draft: LandingPageDraft,
}

impl Deref for LandingPage {
    type Target = LandingPageDraft;

    fn deref(&self) -> &Self::Target {
        &self.draft
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandingPageRole {
    #[default]
    Standard,
    LeadCapture,
    Advertorial,
    AfterOptin,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrafficSourceDraft {
    pub name: String,
    pub external_id_parameter: String,
    pub cost_parameter: String,
    pub custom_parameters: Vec<UrlToken>,
    pub currency: String,
    pub postback_urls: Vec<String>,
    pub pixel_url: String,
    pub track_impressions: bool,
    pub direct_tracking: bool,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrafficSource {
    pub id: String,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
    pub archived: bool,
    #[serde(flatten)]
    pub draft: TrafficSourceDraft,
}

impl Deref for TrafficSource {
    type Target = TrafficSourceDraft;

    fn deref(&self) -> &Self::Target {
        &self.draft
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversionEventCategory {
    Lead,
    Sale,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionEventTypeDraft {
    pub name: String,
    pub event_key: String,
    pub aliases: Vec<String>,
    pub category: ConversionEventCategory,
    pub include_in_conversions: bool,
    pub include_in_revenue: bool,
    pub include_in_cost: bool,
    pub send_postback_to_traffic_source: bool,
    pub default_revenue_value: f64,
    pub currency: String,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionEventType {
    pub id: String,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
    pub archived: bool,
    #[serde(flatten)]
    pub draft: ConversionEventTypeDraft,
}

impl Deref for ConversionEventType {
    type Target = ConversionEventTypeDraft;

    fn deref(&self) -> &Self::Target {
        &self.draft
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunnelDraft {
    pub country: String,
    pub name: String,
    pub redirect_handling: String,
    pub referrer_handling: String,
    pub conditional_sequences: Vec<FunnelSequence>,
    pub default_sequences: Vec<FunnelSequence>,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Funnel {
    pub id: String,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
    pub archived: bool,
    #[serde(flatten)]
    pub draft: FunnelDraft,
}

impl Deref for Funnel {
    type Target = FunnelDraft;

    fn deref(&self) -> &Self::Target {
        &self.draft
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignDraft {
    pub traffic_source_id: String,
    pub destination_type: DestinationType,
    pub funnel_id: Option<String>,
    pub direct_sequence: Option<FunnelSequence>,
    pub cost_model: String,
    pub cost_value: f64,
    pub country: String,
    pub name: String,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Campaign {
    pub id: String,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
    pub archived: bool,
    pub tracking_url: String,
    pub traffic_source_query_template: String,
    pub last_clicked_at_millis: Option<i64>,
    #[serde(flatten)]
    pub draft: CampaignDraft,
}

impl Deref for Campaign {
    type Target = CampaignDraft;

    fn deref(&self) -> &Self::Target {
        &self.draft
    }
}
