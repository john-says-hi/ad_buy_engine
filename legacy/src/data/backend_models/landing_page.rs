use crate::data::elements::landing_page::LandingPage;
use crate::data::lists::{DataURLToken, Language, Vertical};
use crate::data::work_space::Clearance;
#[cfg(feature = "backend")]
use crate::schema::*;
use crate::Country;
use chrono::{DateTime, NaiveDateTime, Utc};
use url::Url;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "landing_pages",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LandingPageModel {
    pub id: String,
    pub account_id: String,
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
    pub weight: String,
    pub archived: bool,
    pub last_updated: i64,
}

impl From<LandingPage> for LandingPageModel {
    fn from(landing_page: LandingPage) -> Self {
        Self {
            id: landing_page.landing_page_id.to_string(),
            account_id: landing_page.account_id.to_string(),
            clearance: serde_json::to_string(&landing_page.clearance).expect("GFsdfg"),
            country: serde_json::to_string(&landing_page.country).expect("Gtrfxg"),
            name: landing_page.name,
            tags: serde_json::to_string(&landing_page.tags).expect("fsdgsdfg4"),
            url: serde_json::to_string(&landing_page.url).expect("GHsdfg4"),
            url_tokens: serde_json::to_string(&landing_page.url_tokens).expect("Gt54f"),
            number_of_calls_to_action: serde_json::to_string(
                &landing_page.number_of_calls_to_action,
            )
            .expect("g54gfdfd"),
            vertical: serde_json::to_string(&landing_page.vertical).expect("GHT%sdf"),
            language: serde_json::to_string(&landing_page.language).expect("G%Tdf"),
            notes: landing_page.notes,
            weight: serde_json::to_string(&landing_page.weight).expect("%Ggfdss"),
            archived: landing_page.archived,
            last_updated: landing_page.last_updated.timestamp(),
        }
    }
}

impl From<LandingPageModel> for LandingPage {
    fn from(landing_page_model: LandingPageModel) -> Self {
        Self {
            landing_page_id: Uuid::parse_str(&landing_page_model.id).expect("YT%gdsf"),
            account_id: Uuid::parse_str(&landing_page_model.account_id).expect("G%^srdfg"),
            clearance: serde_json::from_str(&landing_page_model.clearance).expect("YH^%gdfg"),
            country: serde_json::from_str(&landing_page_model.country).expect("G%xfgg"),
            name: landing_page_model.name,
            tags: serde_json::from_str(&landing_page_model.tags).expect("^HYdfsgfd"),
            url: serde_json::from_str(&landing_page_model.url).expect("H^Tdfg"),
            url_tokens: serde_json::from_str(&landing_page_model.url_tokens).expect("H^Ydfgh"),
            number_of_calls_to_action: serde_json::from_str(
                &landing_page_model.number_of_calls_to_action,
            )
            .expect("HG%^sdf"),
            vertical: serde_json::from_str(&landing_page_model.vertical).expect("GHTsdf"),
            language: serde_json::from_str(&landing_page_model.language).expect("Hgfdshgf"),
            notes: landing_page_model.notes,
            weight: serde_json::from_str(&landing_page_model.weight).expect("G%sdfg"),
            archived: landing_page_model.archived,
            last_updated: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(landing_page_model.last_updated, 0),
                Utc,
            ),
        }
    }
}
