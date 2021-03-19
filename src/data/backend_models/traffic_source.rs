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
        
        Self {
            id: traffic_source.traffic_source_id.to_string(),
            account_id:traffic_source.account_id.to_string(),
            name:traffic_source.name,
            clearance: serde_json::to_string(&traffic_source.clearance).expect("G%Tsf"),
            external_id_token_data: serde_json::to_string(&traffic_source.external_id_token_data).expect("G%f8"),
            cost_token_data: serde_json::to_string(&traffic_source.cost_token_data).expect("G654trdseg"),
            custom_token_data: serde_json::to_string(&traffic_source.custom_token_data).expect("G%Rtsdfg"),
            currency: serde_json::to_string(&traffic_source.currency).expect("H^%gsdf"),
            traffic_source_postback_url: serde_json::to_string(&traffic_source.traffic_source_postback_url).expect("GH^T%sddfg"),
            traffic_source_postback_url_on_custom_event: serde_json::to_string(&traffic_source.traffic_source_postback_url_on_custom_event).expect("fdgfsdgfsd"),
            pixel_redirect_url: serde_json::to_string(&traffic_source.pixel_redirect_url).expect("pofdsa"),
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

      Self {
          traffic_source_id:Uuid::parse_str(&traffic_source_model.id).expect("GFsdfg"),
          account_id:Uuid::parse_str(&traffic_source_model.account_id).expect("Gfsdfg5"),
          name:traffic_source_model.name,
          clearance:serde_json::from_str(&traffic_source_model.clearance).expect("Gfsdfg54"),
          external_id_token_data:serde_json::from_str(&traffic_source_model.external_id_token_data).expect("gh65tdfsg"),
          cost_token_data:serde_json::from_str(&traffic_source_model.cost_token_data).expect("G5sdrfg"),
          custom_token_data:serde_json::from_str(&traffic_source_model.custom_token_data).expect("yt564srf"),
          currency:serde_json::from_str(&traffic_source_model.currency).expect("HG^gfsdh"),
          traffic_source_postback_url:serde_json::from_str(&traffic_source_model.traffic_source_postback_url).expect("Gh6dfsg"),
          traffic_source_postback_url_on_custom_event:serde_json::from_str(&traffic_source_model.traffic_source_postback_url_on_custom_event).expect("FG54sdf"),
          pixel_redirect_url:serde_json::from_str(&traffic_source_model.pixel_redirect_url).expect("g56rfst"),
          track_impressions:traffic_source_model.track_impressions,
          direct_tracking:traffic_source_model.direct_tracking,
          notes:traffic_source_model.notes,
          archived:traffic_source_model.archived,
          last_updated:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(traffic_source_model.last_updated, 0), Utc),
      }  
    }
}
