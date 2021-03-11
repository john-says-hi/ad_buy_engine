use crate::data::elements::offer_source::OfferSource;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "offer_source_table",
    primary_key("offer_source_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferSourceModel {
    pub offer_source_id: String,
    pub account_id: String,
    pub offer_source_data: String,
    pub last_updated: i64,
}

impl From<OfferSource> for OfferSourceModel {
    fn from(offer_source: OfferSource) -> Self {
        Self {
            offer_source_id: offer_source.offer_source_id.to_string(),
            account_id: offer_source.account_id.to_string(),
            offer_source_data: serde_json::to_string(&offer_source).expect("35g^%6"),
            last_updated: offer_source.last_updated.timestamp(),
        }
    }
}

impl From<OfferSourceModel> for OfferSource {
    fn from(offer_source_model: OfferSourceModel) -> Self {
        serde_json::from_str(&offer_source_model.offer_source_data).expect("$G%sH^7")
    }
}
