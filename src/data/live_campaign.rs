use crate::data::custom_events::traffic_source_postback_url::TrafficSourcePostbackURLForEvent;
use crate::data::custom_events::CustomConversionEvent;
use crate::data::elements::campaign::LiveCampaignDestination;
use crate::data::elements::offer_source::LiveOfferSource;
use crate::data::elements::traffic_source::LiveTrafficSource;
use crate::data::lists::click_transition_method::RedirectOption;
use crate::data::lists::CostModel;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: String,
    pub account_id: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlimUser {
    pub user_id: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiveCampaign {
    pub campaign_id: Uuid,
    pub account_id: Uuid,
    pub live_traffic_source: LiveTrafficSource,
    pub live_offer_sources: Vec<LiveOfferSource>,
    pub linked_custom_conversions: Vec<CustomConversionEvent>,
    pub cost_model: CostModel,
    pub cost_value: Decimal,
    pub redirect_option: RedirectOption,
    pub campaign_destination: LiveCampaignDestination,
    pub unique_traffic_source_postback_url: Option<Url>,
    pub unique_traffic_source_postback_url_on_custom_event: Vec<TrafficSourcePostbackURLForEvent>,
    pub unique_pixel_redirect_url: Option<Url>,
    pub last_visit_on: NaiveDateTime,
}
