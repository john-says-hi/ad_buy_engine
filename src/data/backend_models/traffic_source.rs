use crate::data::elements::traffic_source::TrafficSource;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use uuid::Uuid;
use crate::data::work_space::Clearance;
use crate::data::elements::traffic_source::traffic_source_params::{ExternalIDParameter, CostParameter, CustomParameter};
use crate::data::lists::Currency;
use url::Url;
use crate::data::custom_events::traffic_source_postback_url::TrafficSourcePostbackURLForEvent;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "traffic_sources",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficSourceModel {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub clearance: String,
    pub external_id_token_data: String,
    pub cost_token_data: String,
    pub custom_token_data: String,
    pub currency: String,
    pub traffic_source_postback_url: String,
    pub traffic_source_postback_url_on_custom_event: String,
    pub pixel_redirect_url: String,
    pub track_impressions: bool,
    pub direct_tracking: bool,
    pub notes: String,
    pub archived: bool,
    pub last_updated: i64,
}

impl From<TrafficSource> for TrafficSourceModel {
    fn from(traffic_source: TrafficSource) -> Self {
        to_json_string!(
            id; traffic_source.traffic_source_id
            account_id; traffic_source.account_id
            clearance; traffic_source.clearance
            external_id_token_data; traffic_source.external_id_token_data
            cost_token_data; traffic_source.cost_token_data
            custom_token_data; traffic_source.custom_token_data
            currency; traffic_source.currency
            traffic_source_postback_url; traffic_source.traffic_source_postback_url
            traffic_source_postback_url_on_custom_event; traffic_source.traffic_source_postback_url_on_custom_event
            pixel_redirect_url; traffic_source.pixel_redirect_url
        );
        
        Self {
            id,
            account_id,
            name:traffic_source.name,
            clearance,
            external_id_token_data,
            cost_token_data,
            custom_token_data,
            currency,
            traffic_source_postback_url,
            traffic_source_postback_url_on_custom_event,
            pixel_redirect_url,
            track_impressions: traffic_source.track_impressions,
            direct_tracking: traffic_source.direct_tracking,
            notes:traffic_source.notes,
            archived: traffic_source.archived,
            last_updated: traffic_source.last_updated.timestamp(),
        }
    }
}

impl From<TrafficSourceModel> for TrafficSource {
    fn from(traffic_source_model: TrafficSourceModel) -> Self {
        from_json_string!(
            traffic_source_id; traffic_source_model.id => Uuid
            account_id; traffic_source_model.account_id => Uuid
            clearance; traffic_source_model.clearance => Clearance
            external_id_token_data; traffic_source_model.external_id_token_data => ExternalIDParameter
            cost_token_data; traffic_source_model.cost_token_data => CostParameter
            custom_token_data; traffic_source_model.custom_token_data => Vec<CustomParameter>
            currency; traffic_source_model.currency => Currency
            traffic_source_postback_url; traffic_source_model.traffic_source_postback_url => Option<Url>
            traffic_source_postback_url_on_custom_event; traffic_source_model.traffic_source_postback_url_on_custom_event => Vec<TrafficSourcePostbackURLForEvent>
            pixel_redirect_url; traffic_source_model.pixel_redirect_url => Option<Url>
        );
        
      Self {
          traffic_source_id,
          account_id,
          name:traffic_source_model.name,
          clearance,
          external_id_token_data,
          cost_token_data,
          custom_token_data,
          currency,
          traffic_source_postback_url,
          traffic_source_postback_url_on_custom_event,
          pixel_redirect_url,
          track_impressions:traffic_source_model.track_impressions,
          direct_tracking:traffic_source_model.direct_tracking,
          notes:traffic_source_model.notes,
          archived:traffic_source_model.archived,
          last_updated:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(traffic_source_model.last_updated, 0), Utc),
      }  
    }
}
