use crate::data::elements::campaign::Campaign;
use crate::data::elements::funnel::Funnel;
use crate::data::elements::landing_page::LandingPage;
use crate::data::elements::offer::Offer;
use crate::data::elements::offer_source::OfferSource;
use crate::data::elements::traffic_source::TrafficSource;
use crate::data::visit::Visit;
use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct SyncVisitsRequest {
    pub date_of_newest_visit: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SyncVisitsResponse {
    pub visits: Vec<Visit>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SyncElementsRequest {
    pub offer_sources: Vec<SyncElementData>,
    pub offers: Vec<SyncElementData>,
    pub landing_pages: Vec<SyncElementData>,
    pub funnels: Vec<SyncElementData>,
    pub traffic_sources: Vec<SyncElementData>,
    pub campaigns: Vec<SyncElementData>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SyncElementsResponse {
    pub offer_sources: Vec<OfferSource>,
    pub offers: Vec<Offer>,
    pub landing_pages: Vec<LandingPage>,
    pub funnels: Vec<Funnel>,
    pub traffic_sources: Vec<TrafficSource>,
    pub campaigns: Vec<Campaign>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SyncElementData {
    pub element_id: Uuid,
    pub last_updated: DateTime<Utc>,
}
