use crate::data::elements::landing_page::LandingPage;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use uuid::Uuid;
use crate::data::work_space::Clearance;
use crate::Country;
use url::Url;
use crate::data::lists::{DataURLToken, Vertical, Language};

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "landing_page_table",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LandingPageModel {
    pub id: String,
    pub account_id: String,
    pub is_pre_landing_page: bool,
    pub clearance: String,
    pub country: String,
    pub name: String,
    pub tags: String,
    pub url: String,
    pub url_tokens: String,
    pub number_of_calls_to_action: String,
    pub vertical: String,
    pub language: String,
    pub notes: String,
    pub archived: bool,
    pub last_updated: i64,
}

impl From<LandingPage> for LandingPageModel {
    fn from(landing_page: LandingPage) -> Self {
        to_json_string!(
            id; landing_page.landing_page_id
            account_id; landing_page.account_id
            clearance; landing_page.clearance
            country; landing_page.clearance
            tags; landing_page.tags
            url; landing_page.url
            url_tokens; landing_page.url_tokens
            number_of_calls_to_action; landing_page.number_of_calls_to_action
            vertical; landing_page.vertical
            language; landing_page.language
        );
        
        Self {
            id,
            account_id,
            is_pre_landing_page: landing_page.is_pre_landing_page,
            clearance,
            country,
            name:landing_page.name,
            tags,
            url,
            url_tokens,
            number_of_calls_to_action,
            vertical,
            language,
            notes:landing_page.notes,
            archived: landing_page.archived,
            last_updated: landing_page.last_updated.timestamp(),
        }
    }
}

impl From<LandingPageModel> for LandingPage {
    fn from(landing_page_model: LandingPageModel) -> Self {
        from_json_string!(
            landing_page_id; landing_page_model.id => Uuid
            account_id; landing_page_model.account_id => Uuid
            clearance; landing_page_model.clearance => Clearance
            country; landing_page_model.country => Country
            tags; landing_page_model.tags => Vec<String>
            url; landing_page_model.url => Url
            url_tokens; landing_page_model.url_tokens => Vec<DataURLToken>
            number_of_calls_to_action; landing_page_model.number_of_calls_to_action => u8
            vertical; landing_page_model.vertical => Vertical
            language; landing_page_model.language => Language
        );
        
        Self {
            landing_page_id,
            account_id,
            is_pre_landing_page:landing_page_model.is_pre_landing_page,
            clearance,
            country,
            name:landing_page_model.name,
            tags,
            url,
            url_tokens,
            number_of_calls_to_action,
            vertical,
            language,
            notes:landing_page_model.notes,
            archived:landing_page_model.archived,
            last_updated: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(landing_page_model.last_updated, 0), Utc),
        }
    }
}
