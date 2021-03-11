use crate::data::elements::funnel::Funnel;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "funnel_table",
    primary_key("funnel_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunnelModel {
    pub funnel_id: String,
    pub account_id: String,
    pub funnel_data: String,
    pub last_updated: i64,
}

impl From<Funnel> for FunnelModel {
    fn from(funnel: Funnel) -> Self {
        Self {
            funnel_id: funnel.funnel_id.to_string(),
            account_id: funnel.account_id.to_string(),
            funnel_data: serde_json::to_string(&funnel).expect("G%$#sS"),
            last_updated: funnel.last_updated.timestamp(),
        }
    }
}

impl From<FunnelModel> for Funnel {
    fn from(funnel_model: FunnelModel) -> Self {
        serde_json::from_str(&funnel_model.funnel_data).expect("VG#4rzs")
    }
}
