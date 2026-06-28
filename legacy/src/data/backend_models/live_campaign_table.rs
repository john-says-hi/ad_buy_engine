// use crate::data::elements::landing_page::LandingPage;
// #[cfg(feature = "backend")]
// use crate::schema::*;
// use chrono::NaiveDateTime;
// use uuid::Uuid;
//
// #[cfg_attr(feature = "backend", derive(Queryable, Insertable, AsChangeset))]
// #[cfg_attr(feature = "backend", table_name = "live_campaign_table")]
// #[derive(Serialize, Deserialize, Clone,Debug)]
// pub struct LandingPageModel {
//     pub landing_page_id: String,
//     pub account_id: String,
//     pub landing_page_data: String,
// }
//
// impl From<LandingPage> for LandingPageModel {
//     fn from(landing_page: LandingPage) -> Self {
//         Self {
//             landing_page_id: landing_page.landing_page_id.to_string(),
//             account_id: landing_page.account_id.to_string(),
//             landing_page_data: serde_json::to_string(&landing_page).expect("G%$#sS"),
//         }
//     }
// }
//
// impl From<LandingPageModel> for LandingPage {
//     fn from(landing_page_model: LandingPageModel) -> Self {
//         serde_json::from_str(&landing_page_model.landing_page_data).expect("VG#4rzs")
//     }
// }
