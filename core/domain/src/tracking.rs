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
    pub enrichment: VisitEnrichment,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisitEnrichment {
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub timezone: Option<String>,
    pub postal_code: Option<String>,
    pub metro_code: Option<String>,
    pub asn: Option<String>,
    pub asn_organization: Option<String>,
    pub isp: Option<String>,
    pub connection_type: Option<String>,
    pub proxy_type: Option<String>,
    pub carrier: Option<String>,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    pub operating_system: Option<String>,
    pub operating_system_version: Option<String>,
    pub device_type: Option<String>,
    pub device_brand: Option<String>,
    pub device_model: Option<String>,
}
