use crate::data::custom_events::traffic_source_postback_url::TrafficSourcePostbackURLForEvent;
use crate::data::custom_events::CustomConversionEvent;
use crate::data::elements::funnel::{Funnel, Sequence};
use crate::data::elements::offer_source::{LiveOfferSource, OfferSource};
use crate::data::elements::traffic_source::{LiveTrafficSource, TrafficSource};
use crate::data::lists::click_transition_method::RedirectOption;
use crate::data::lists::{CostModel, DataURLToken, Language, Vertical};
use crate::data::live_campaign::LiveCampaign;
use crate::data::work_space::Clearance;
use crate::{AError, Country};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use either::Either;
use rust_decimal::Decimal;
use std::str::FromStr;
use strum::IntoEnumIterator;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LiveCampaignDestination {
    Funnel(Funnel),
    Sequence(Sequence),
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, EnumString, ToString, EnumIter)]
pub enum CampaignDestinationType {
    #[strum(serialize = "Funnel")]
    Funnel,
    #[strum(serialize = "Sequence")]
    Sequence,
}

impl FromStr for Campaign {
    type Err = AError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

impl ToString for Campaign {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Campaign {
    pub campaign_id: Uuid,
    pub account_id: Uuid,
    pub clearance: Clearance,
    pub traffic_source: TrafficSource,
    pub country: Country,
    pub name: String,
    pub cost_model: CostModel,
    pub cost_value: Decimal,
    pub redirect_option: RedirectOption,
    pub campaign_destination: CampaignDestinationType,
    pub campaign_core: Either<Funnel, Sequence>,
    pub notes: String,
    pub archived: bool,
    pub last_updated: DateTime<Utc>,
    pub last_clicked: DateTime<Utc>,
    pub hosts: Vec<Url>,
}

// impl From<Campaign> for LiveCampaign {
//     fn from(campaign: Campaign) -> Self {
//         Self {
//             campaign_id: campaign.campaign_id,
//             account_id: campaign.account_id,
//             live_traffic_source: campaign.traffic_source.into(),
//             live_offer_sources: campaign
//                 .linked_offer_sources
//                 .iter()
//                 .map(|s| s.clone().into())
//                 .collect::<Vec<LiveOfferSource>>(),
//             linked_custom_conversions: campaign.linked_custom_conversions,
//             cost_model: campaign.cost_model,
//             cost_value: campaign.cost_value,
//             redirect_option: campaign.redirect_option,
//             campaign_destination: match campaign.campaign_core {
//                 Either::Left(funnel) => LiveCampaignDestination::Funnel(funnel),
//                 Either::Right(sequence) => LiveCampaignDestination::Sequence(sequence),
//             },
//             unique_traffic_source_postback_url: campaign.unique_traffic_source_postback_url,
//             unique_traffic_source_postback_url_on_custom_event: campaign
//                 .unique_traffic_source_postback_url_on_custom_event,
//             unique_pixel_redirect_url: campaign.unique_pixel_redirect_url,
//             last_visit_on: Local::now().naive_local(),
//         }
//     }
// }
