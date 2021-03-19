use crate::data::visit::Visit;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "visit_ledger_table",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VisitUpdateLedger {
    pub id: String,
    pub visit_ids: String,
}
