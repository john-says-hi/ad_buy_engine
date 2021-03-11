use crate::data::elements::landing_page::LandingPage;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "landing_page_table",
    primary_key("landing_page_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LandingPageModel {
    pub landing_page_id: String,
    pub account_id: String,
    pub landing_page_data: String,
    pub last_updated: i64,
}

impl From<LandingPage> for LandingPageModel {
    fn from(landing_page: LandingPage) -> Self {
        Self {
            landing_page_id: landing_page.landing_page_id.to_string(),
            account_id: landing_page.account_id.to_string(),
            landing_page_data: serde_json::to_string(&landing_page).expect("G%$#sS"),
            last_updated: landing_page.last_updated.timestamp(),
        }
    }
}

impl From<LandingPageModel> for LandingPage {
    fn from(landing_page_model: LandingPageModel) -> Self {
        serde_json::from_str(&landing_page_model.landing_page_data).expect("VG#4rzs")
    }
}
