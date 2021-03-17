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
        to_json_string!(
            id; funnel.funnel_id
            account_id; funnel.account_id
            country; funnel.country
            clearance; funnel.clearance
            redirect_option; funnel.redirect_option
            referrer_handling; funnel.referrer_handling
            conditional_sequences; funnel.conditional_sequences
            default_sequences; funnel.default_sequences
        );
        
        Self {
            id,
            account_id,
            country,
            name:funnel.name,
            clearance,
            redirect_option,
            referrer_handling,
            notes:funnel.notes,
            conditional_sequences,
            default_sequences,
            archived: funnel.archived,
            last_updated: funnel.last_updated.timestamp(),
        }
    }
}

impl From<FunnelModel> for Funnel {
    fn from(funnel_model: FunnelModel) -> Self {
        from_json_string!(
            funnel_id; funnel_model.id => Uuid
            account_id; funnel_model.account_id => Uuid
            country; funnel_model.country => Country
            clearance; funnel_model.clearance => Clearance
            redirect_option; funnel_model.redirect_option => RedirectOption
            referrer_handling; funnel_model.referrer_handling => ReferrerHandling
            conditional_sequences; funnel_model.conditional_sequences => Vec<ConditionalSequence>
            default_sequences; funnel_model.default_sequences => Vec<Sequence>
        );
        
        Self {
            funnel_id,
            account_id,
            country,
            name:funnel_model.name,
            clearance,
            redirect_option,
            referrer_handling,
            notes:funnel_model.notes,
            conditional_sequences,
            default_sequences,
            archived:funnel_model.archived,
            last_updated:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(funnel_model.last_updated, 0), Utc),
        }
    }
}
