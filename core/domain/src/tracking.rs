use serde::{Deserialize, Serialize};

use crate::funnel::FunnelSequence;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignLinkInfo {
    pub campaign_id: String,
    pub campaign_name: String,
    pub tracking_url: String,
    pub traffic_source_query_template: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisitEventType {
    CampaignClick,
    LanderClick,
    OfferClick,
    Conversion,
    CustomConversion,
    Error,
    ConditionDataMissing,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisitRecord {
    pub id: String,
    pub campaign_id: String,
    pub traffic_source_id: String,
    pub selected_funnel_id: Option<String>,
    pub selected_sequence: Option<FunnelSequence>,
    pub selected_landing_page_id: Option<String>,
    pub selected_offer_id: Option<String>,
    pub referrer: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub query_params: Vec<(String, String)>,
    pub click_map: Vec<ClickMapEntry>,
    pub redirect_target: String,
    pub suspicious: bool,
    pub created_at_millis: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClickMapEntry {
    pub slot: u8,
    pub offer_id: String,
    pub target_url: String,
}
