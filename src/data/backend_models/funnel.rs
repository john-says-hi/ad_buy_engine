use crate::data::elements::funnel::{Funnel, ConditionalSequence, Sequence};
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use uuid::Uuid;
use crate::Country;
use crate::data::work_space::Clearance;
use crate::data::lists::click_transition_method::RedirectOption;
use crate::data::lists::referrer_handling::ReferrerHandling;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "funnels",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunnelModel {
    pub id: String,
    pub account_id: String,
    pub country: String,
    pub name: String,
    pub clearance: String,
    pub redirect_option: String,
    pub referrer_handling: String,
    pub notes: String,
    pub conditional_sequences: String,
    pub default_sequences: String,
    pub archived: bool,
    pub last_updated: i64,
}

impl From<Funnel> for FunnelModel {
    fn from(funnel: Funnel) -> Self {
        
        Self {
            id: funnel.funnel_id.to_string(),
            account_id: funnel.account_id.to_string(),
            country: serde_json::to_string(&funnel.country).expect("G%$rsdf"),
            name:funnel.name,
            clearance: serde_json::to_string(&funnel.clearance).expect("G%Tsdf"),
            redirect_option: serde_json::to_string(&funnel.redirect_option).expect("G%$#sdf"),
            referrer_handling: serde_json::to_string(&funnel.referrer_handling).expect("G54sdff"),
            notes:funnel.notes,
            conditional_sequences: serde_json::to_string(&funnel.conditional_sequences).expect("G%sdf"),
            default_sequences: serde_json::to_string(&funnel.default_sequences).expect("gyt5643f"),
            archived: funnel.archived,
            last_updated: funnel.last_updated.timestamp(),
        }
    }
}

impl From<FunnelModel> for Funnel {
    fn from(funnel_model: FunnelModel) -> Self {
        
        Self {
            funnel_id:Uuid::parse_str(&funnel_model.id).expect("g534d"),
            account_id:Uuid::parse_str(&funnel_model.account_id).expect("G%sxdsf"),
            country: serde_json::from_str(&funnel_model.country).expect("f4g"),
            name:funnel_model.name,
            clearance:serde_json::from_str(&funnel_model.clearance).expect("G%Rsdf"),
            redirect_option:serde_json::from_str(&funnel_model.redirect_option).expect("G54sdf"),
            referrer_handling:serde_json::from_str(&funnel_model.referrer_handling).expect("GH56sd"),
            notes:funnel_model.notes,
            conditional_sequences:serde_json::from_str(&funnel_model.conditional_sequences).expect("%G$s"),
            default_sequences:serde_json::from_str(&funnel_model.default_sequences).expect("G%sdf"),
            archived:funnel_model.archived,
            last_updated:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(funnel_model.last_updated, 0), Utc),
        }
    }
}
