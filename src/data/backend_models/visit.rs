use crate::data::visit::Visit;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "visit_table",
    primary_key("click_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VisitModel {
    pub click_id: String,
    pub account_id: String,
    pub visit_data: String,
    pub last_updated: i64,
}

impl From<Visit> for VisitModel {
    fn from(visit: Visit) -> Self {
        Self {
            click_id: visit.click_id.to_string(),
            account_id: visit.account_id.to_string(),
            visit_data: serde_json::to_string(&visit).expect("G%$#sS"),
            last_updated: visit.last_updated.timestamp(),
        }
    }
}

impl From<VisitModel> for Visit {
    fn from(visit_model: VisitModel) -> Self {
        serde_json::from_str(&visit_model.visit_data).expect("VG#4rzs")
    }
}
