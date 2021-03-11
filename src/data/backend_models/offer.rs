use crate::data::elements::offer::Offer;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "offer_table",
    primary_key("offer_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferModel {
    pub offer_id: String,
    pub account_id: String,
    pub offer_data: String,
    pub last_updated: i64,
}

impl From<Offer> for OfferModel {
    fn from(offer: Offer) -> Self {
        Self {
            offer_id: offer.offer_id.to_string(),
            account_id: offer.account_id.to_string(),
            offer_data: serde_json::to_string(&offer).expect("G%$#sS"),
            last_updated: offer.last_updated.timestamp(),
        }
    }
}

impl From<OfferModel> for Offer {
    fn from(offer_model: OfferModel) -> Self {
        serde_json::from_str(&offer_model.offer_data).expect("VG#4rzs")
    }
}
