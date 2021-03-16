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
    table_name = "visit_table",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VisitModel {
    pub id: String,
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
    pub created_at: i64,
    pub last_updated: i64,
}

impl From<Visit> for VisitModel {
    fn from(visit: Visit) -> Self {
        to_json_string!(
            id; visit.id
            account_id; visit.account_id
            campaign_id; visit.campaign_id
            traffic_source_id; visit.traffic_source_id
            funnel_id; visit.funnel_id
            pre_sell_landing_page_id; visit.pre_sell_landing_page_id
            landing_page_ids; visit.landing_page_ids
            offer_ids; visit.offer_ids
            impressions_from_traffic_source; visit.impressions_from_traffic_source
            tracking_link_clicks; visit.tracking_link_clicks
            pre_landing_page_clicks; visit.pre_landing_page_clicks
            landing_page_clicks; visit.landing_page_clicks
            offer_clicks; visit.offer_clicks
            referrer; visit.referrer
            traffic_source_parameters; visit.traffic_source_parameters
            redirection_time; visit.redirection_time
            click_map; visit.click_map
            user_agent_data; visit.user_agent_data
            geo_ip_data; visit.geo_ip_data
            conversions; visit.conversions
            custom_conversions; visit.custom_conversions
        );
        
        Self {
            id,
            account_id,
            campaign_id,
            traffic_source_id,
            funnel_id,
            pre_sell_landing_page_id,
            landing_page_ids,
            offer_ids,
            impressions_from_traffic_source,
            tracking_link_clicks,
            pre_landing_page_clicks,
            landing_page_clicks,
            offer_clicks,
            referrer,
            traffic_source_parameters,
            redirection_time,
            click_map,
            user_agent_data,
            geo_ip_data,
            conversions,
            custom_conversions,
            click_is_suspicious: visit.click_is_suspicious,
            created_at: visit.created_at.timestamp(),
            last_updated: visit.last_updated.timestamp(),
        }
    }
}

impl From<VisitModel> for Visit {
    fn from(visit_model: VisitModel) -> Self {
        from_json_string!(
            id; visit_model.id => Uuid
            account_id; visit_model.account_id => Uuid
            campaign_id; visit_model.campaign_id => Uuid
            traffic_source_id; visit_model.traffic_source_id => Uuid
            funnel_id; visit_model.funnel_id => Option<Uuid>
            pre_sell_landing_page_id; visit_model.pre_sell_landing_page_id => Option<Uuid>
            landing_page_ids; visit_model.landing_page_ids => Vec<Uuid>
            offer_ids; visit_model.offer_ids => Vec<Uuid>
            impressions_from_traffic_source; visit_model.impressions_from_traffic_source => u64
            tracking_link_clicks; visit_model.tracking_link_clicks => u32
            pre_landing_page_clicks; visit_model.pre_landing_page_clicks =>Vec<ClickEvent>
            landing_page_clicks; visit_model.landing_page_clicks => Vec<ClickEvent>
            offer_clicks; visit_model.offer_clicks => Vec<ClickEvent>
            referrer; visit_model.referrer => Url
            traffic_source_parameters; visit_model.traffic_source_parameters => HashMap<String, String>
            redirection_time; visit_model.redirection_time => Duration
            click_map; visit_model.click_map => ClickMap
            user_agent_data; visit_model.user_agent_data => UserAgentData
            geo_ip_data; visit_model.geo_ip_data => GeoIPData
            conversions; visit_model.conversions => Vec<Conversion>
            custom_conversions; visit_model.custom_conversions => Vec<CustomConversionEvent>
        );
    
        Self {
            id,
            account_id,
            campaign_id,
            traffic_source_id,
            funnel_id,
            pre_sell_landing_page_id,
            landing_page_ids,
            offer_ids,
            impressions_from_traffic_source,
            tracking_link_clicks,
            pre_landing_page_clicks,
            landing_page_clicks,
            offer_clicks,
            referrer,
            traffic_source_parameters,
            redirection_time,
            click_map,
            user_agent_data,
            geo_ip_data,
            conversions,
            custom_conversions,
            click_is_suspicious:visit_model.click_is_suspicious,
            created_at:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(visit_model.last_updated, 0), Utc),
            last_updated:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(visit_model.last_updated, 0), Utc),
        }
    }
}
