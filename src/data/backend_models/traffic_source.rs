use crate::data::elements::traffic_source::TrafficSource;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "traffic_source_table",
    primary_key("traffic_source_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficSourceModel {
    pub traffic_source_id: String,
    pub account_id: String,
    pub traffic_source_data: String,
    pub last_updated: i64,
}

impl From<TrafficSource> for TrafficSourceModel {
    fn from(traffic_source: TrafficSource) -> Self {
        Self {
            traffic_source_id: traffic_source.traffic_source_id.to_string(),
            account_id: traffic_source.account_id.to_string(),
            traffic_source_data: serde_json::to_string(&traffic_source).expect("G%$#sS"),
            last_updated: traffic_source.last_updated.timestamp(),
        }
    }
}

impl From<TrafficSourceModel> for TrafficSource {
    fn from(traffic_source_model: TrafficSourceModel) -> Self {
        serde_json::from_str(&traffic_source_model.traffic_source_data).expect("VG#4rzs")
    }
}
