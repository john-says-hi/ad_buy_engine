use crate::data::custom_events::CustomConversionEvent;
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficSourcePostbackURLForEvent {
    pub event: CustomConversionEvent,
    pub traffic_source_postback_url: Url,
}
