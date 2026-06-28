use url::Url;
use uuid::Uuid;

use crate::data::custom_events::traffic_source_postback_url::TrafficSourcePostbackURLForEvent;
use crate::data::elements::traffic_source::traffic_source_params::{
    CostParameter, CustomParameter, ExternalIDParameter,
};
use crate::data::lists::{Currency, DataURLToken, Language, TrafficSourceToken, Vertical};
use crate::data::work_space::Clearance;
use crate::{AError, Country};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::str::FromStr;
pub mod traffic_source_params;

impl FromStr for TrafficSource {
    type Err = AError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

impl ToString for TrafficSource {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficSource {
    pub traffic_source_id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub clearance: Clearance,
    pub external_id_token_data: ExternalIDParameter,
    pub cost_token_data: CostParameter,
    pub custom_token_data: Vec<CustomParameter>,
    pub currency: Currency,
    pub traffic_source_postback_url: Option<Url>,
    pub traffic_source_postback_url_on_custom_event: Vec<TrafficSourcePostbackURLForEvent>,
    pub pixel_redirect_url: Option<Url>,
    pub track_impressions: bool,
    pub direct_tracking: bool,
    pub notes: String,
    pub archived: bool,
    pub last_updated: DateTime<Utc>,
}

impl TrafficSource {
    pub fn generate_query_external_id(&self) -> Option<String> {
        if !self.external_id_token_data.parameter.is_empty()
            && !self.external_id_token_data.placeholder.is_empty()
        {
            Some(format!(
                "{}={}",
                self.external_id_token_data.parameter, self.external_id_token_data.placeholder
            ))
        } else {
            None
        }
    }

    pub fn generate_query_cost(&self) -> Option<String> {
        if !self.cost_token_data.parameter.is_empty()
            && !self.cost_token_data.placeholder.is_empty()
        {
            Some(format!(
                "{}={}",
                self.cost_token_data.parameter, self.cost_token_data.placeholder
            ))
        } else {
            None
        }
    }

    pub fn generate_query_custom(&self) -> Option<Vec<String>> {
        let mut tokens = vec![];

        if !self.custom_token_data.is_empty() {
            for token in self.custom_token_data.iter() {
                if token.is_tracked
                    && !token.name.is_empty()
                    && !token.parameter.is_empty()
                    && !token.placeholder.is_empty()
                {
                    tokens.push(format!("{}={}", token.parameter, token.placeholder))
                }
            }
            Some(tokens)
        } else {
            None
        }
    }

    pub fn generate_query(&self) -> String {
        let mut query = "".to_string();

        if let Some(external_id_token) = self.generate_query_external_id() {
            query.push_str(&external_id_token);
            if let Some(cost_token) = self.generate_query_cost() {
                query.push('&');
                query.push_str(&cost_token);
                if let Some(custom_tokens) = self.generate_query_custom() {
                    query.push('&');
                    let last_record = custom_tokens.len() - 1;
                    for (idx, token) in custom_tokens.iter().enumerate() {
                        if idx == last_record {
                            query.push_str(&token)
                        } else {
                            query.push_str(&token);
                            query.push('&');
                        }
                    }
                }
            } else if let Some(custom_tokens) = self.generate_query_custom() {
                query.push('&');
                let last_record = custom_tokens.len() - 1;
                for (idx, token) in custom_tokens.iter().enumerate() {
                    if idx == last_record {
                        query.push_str(&token)
                    } else {
                        query.push_str(&token);
                        query.push('&');
                    }
                }
            }
        } else if let Some(cost_token) = self.generate_query_cost() {
            query.push_str(&cost_token);
            if let Some(custom_tokens) = self.generate_query_custom() {
                query.push('&');
                let last_record = custom_tokens.len() - 1;
                for (idx, token) in custom_tokens.iter().enumerate() {
                    if idx == last_record {
                        query.push_str(&token)
                    } else {
                        query.push_str(&token);
                        query.push('&');
                    }
                }
            }
        } else if let Some(custom_tokens) = self.generate_query_custom() {
            let last_record = custom_tokens.len() - 1;
            for (idx, token) in custom_tokens.iter().enumerate() {
                if idx == last_record {
                    query.push_str(&token)
                } else {
                    query.push_str(&token);
                    query.push('&');
                }
            }
        }
        query
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiveTrafficSource {
    pub traffic_source_id: Uuid,
    pub external_id_token_data: ExternalIDParameter,
    pub cost_token_data: CostParameter,
    pub custom_token_data: Vec<CustomParameter>,
    pub traffic_source_postback_url: Option<Url>,
    pub traffic_source_postback_url_on_custom_event: Vec<TrafficSourcePostbackURLForEvent>,
    pub pixel_redirect_url: Option<Url>,
    pub postback_url: Option<Url>,
    pub track_impressions: bool,
}

// impl From<TrafficSource> for LiveTrafficSource {
//     fn from(traffic_source: TrafficSource) -> Self {
//         Self {
//             traffic_source_id: traffic_source.traffic_source_id,
//             external_id_token_data: traffic_source.external_id_token_data,
//             cost_token_data: traffic_source.cost_token_data,
//             custom_token_data: traffic_source.custom_token_data,
//             traffic_source_postback_url: traffic_source.traffic_source_postback_url,
//             traffic_source_postback_url_on_custom_event: traffic_source
//                 .traffic_source_postback_url_on_custom_event,
//             pixel_redirect_url: traffic_source.pixel_redirect_url,
//             postback_url: traffic_source.postback_url,
//             track_impressions: traffic_source.track_impressions,
//         }
//     }
// }
