use crate::data::visit::Visit;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use uuid::Uuid;
use crate::data::visit::click_event::ClickEvent;
use url::Url;
use std::collections::HashMap;
use std::time::Duration;
use crate::data::visit::click_map::ClickMap;
use crate::data::visit::user_agent::UserAgentData;
use crate::data::visit::geo_ip::GeoIPData;
use crate::data::visit::conversion::Conversion;
use crate::data::custom_events::CustomConversionEvent;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "visits",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VisitModel {
    pub id: i64,
    pub account_id: String,
    pub campaign_id: String,
    pub traffic_source_id: String,
    pub funnel_id: String,
    pub pre_sell_landing_page_id: String,
    pub landing_page_ids: String,
    pub offer_ids: String,
    pub impressions_from_traffic_source: String,
    pub tracking_link_clicks: String,
    pub pre_landing_page_clicks: String,
    pub landing_page_clicks: String,
    pub offer_clicks: String,
    pub referrer: String,
    pub traffic_source_parameters: String,
    pub redirection_time: String,
    pub click_map: String,
    pub user_agent_data: String,
    pub geo_ip_data: String,
    pub conversions: String,
    pub custom_conversions: String,
    pub click_is_suspicious: bool,
    pub last_updated: i64,
}

impl From<Visit> for VisitModel {
    fn from(visit: Visit) -> Self {
        
        Self {
            id:visit.id,
            account_id:visit.account_id.to_string(),
            campaign_id:visit.campaign_id.to_string(),
            traffic_source_id:visit.traffic_source_id.to_string(),
            funnel_id: serde_json::to_string(&visit.funnel_id).expect("G%sdfg"),
            pre_sell_landing_page_id: serde_json::to_string(&visit.pre_sell_landing_page_id).expect("HGTsdfg"),
            landing_page_ids: serde_json::to_string(&visit.landing_page_ids).expect("GH%Tsfd"),
            offer_ids: serde_json::to_string(&visit.offer_ids).expect("GHTsdf"),
            impressions_from_traffic_source: serde_json::to_string(&visit.impressions_from_traffic_source).expect("Gfsdffg"),
            tracking_link_clicks: serde_json::to_string(&visit.tracking_link_clicks).expect("a^sdf"),
            pre_landing_page_clicks: serde_json::to_string(&visit.pre_landing_page_clicks).expect("gtsfd"),
            landing_page_clicks: serde_json::to_string(&visit.landing_page_clicks).expect("YHtdcfgh"),
            offer_clicks: serde_json::to_string(&visit.offer_clicks).expect("fdasdf4"),
            referrer: serde_json::to_string(&visit.referrer).expect("hgfsffd"),
            traffic_source_parameters: serde_json::to_string(&visit.traffic_source_parameters).expect("Gfsdg5r"),
            redirection_time: serde_json::to_string(&visit.redirection_time).expect("h65dfg"),
            click_map: serde_json::to_string(&visit.click_map).expect("G%"),
            user_agent_data: serde_json::to_string(&visit.user_agent_data).expect("h765gh"),
            geo_ip_data:serde_json::to_string(&visit.geo_ip_data).expect("GH^%dsf"),
            conversions:serde_json::to_string(&visit.conversions).expect("t5sdfd"),
            custom_conversions:serde_json::to_string(&visit.custom_conversions).expect("G%^gsdf"),
            click_is_suspicious: visit.click_is_suspicious,
            last_updated: visit.last_updated.timestamp(),
        }
    }
}

impl From<VisitModel> for Visit {
    fn from(visit_model: VisitModel) -> Self {
        Self {
            id:visit_model.id,
            account_id:Uuid::parse_str(&visit_model.account_id).expect("G%sdgf"),
            campaign_id:Uuid::parse_str(&visit_model.campaign_id).expect("G%sdgff"),
            traffic_source_id:Uuid::parse_str(&visit_model.traffic_source_id).expect("G%45sdf"),
            funnel_id: serde_json::from_str(&visit_model.funnel_id).expect("F43sdaf"),
            pre_sell_landing_page_id:serde_json::from_str(&visit_model.pre_sell_landing_page_id).expect("G%$sdf"),
            landing_page_ids: serde_json::from_str(&visit_model.landing_page_ids).expect("GT%fd"),
            offer_ids: serde_json::from_str(&visit_model.offer_ids).expect("H^gdsf"),
            impressions_from_traffic_source: serde_json::from_str(&visit_model.impressions_from_traffic_source).expect("Gf45sf"),
            tracking_link_clicks: serde_json::from_str(&visit_model.tracking_link_clicks).expect("G%sf"),
            pre_landing_page_clicks: serde_json::from_str(&visit_model.pre_landing_page_clicks).expect("GH%tsf"),
            landing_page_clicks: serde_json::from_str(&visit_model.landing_page_clicks).expect("g54sdf"),
            offer_clicks: serde_json::from_str(&visit_model.offer_clicks).expect("G%sdf"),
            referrer: serde_json::from_str(&visit_model.referrer).expect("G%sdf"),
            traffic_source_parameters: serde_json::from_str(&visit_model.traffic_source_parameters).expect("GT%sf"),
            redirection_time: serde_json::from_str(&visit_model.redirection_time).expect("GTsf"),
            click_map: serde_json::from_str(&visit_model.click_map).expect("GTfx"),
            user_agent_data: serde_json::from_str(&visit_model.user_agent_data).expect("h76gfe"),
            geo_ip_data: serde_json::from_str(&visit_model.geo_ip_data).expect("uyhgfd"),
            conversions: serde_json::from_str(&visit_model.conversions).expect("GH%^sf"),
            custom_conversions: serde_json::from_str(&visit_model.custom_conversions).expect("gfdssf"),
            click_is_suspicious:visit_model.click_is_suspicious,
            last_updated:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(visit_model.last_updated, 0), Utc),
        }
    }
}
