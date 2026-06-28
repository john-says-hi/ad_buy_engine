use serde::{Deserialize, Serialize};

use crate::entities::{
    Campaign, CampaignDraft, Funnel, FunnelDraft, LandingPage, LandingPageDraft, Offer, OfferDraft,
    OfferSource, OfferSourceDraft, TrafficSource, TrafficSourceDraft,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    Validation,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub code: ApiErrorCode,
    pub message: String,
    pub details: Vec<FieldError>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityRow {
    pub id: String,
    pub name: String,
    pub detail: String,
    pub visits: i64,
    pub unique_visits: i64,
    pub updated_at_millis: i64,
    pub tracking_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionItem {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionsResponse {
    pub items: Vec<OptionItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum EntityDraft {
    OfferSource(OfferSourceDraft),
    Offer(OfferDraft),
    LandingPage(LandingPageDraft),
    TrafficSource(TrafficSourceDraft),
    Funnel(FunnelDraft),
    Campaign(CampaignDraft),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum EntityRecord {
    OfferSource(OfferSource),
    Offer(Offer),
    LandingPage(LandingPage),
    TrafficSource(TrafficSource),
    Funnel(Funnel),
    Campaign(Campaign),
}

impl EntityRecord {
    pub fn id(&self) -> &str {
        match self {
            Self::OfferSource(record) => &record.id,
            Self::Offer(record) => &record.id,
            Self::LandingPage(record) => &record.id,
            Self::TrafficSource(record) => &record.id,
            Self::Funnel(record) => &record.id,
            Self::Campaign(record) => &record.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::OfferSource(record) => &record.name,
            Self::Offer(record) => &record.name,
            Self::LandingPage(record) => &record.name,
            Self::TrafficSource(record) => &record.name,
            Self::Funnel(record) => &record.name,
            Self::Campaign(record) => &record.name,
        }
    }
}
