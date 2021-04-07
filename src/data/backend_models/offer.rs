use crate::data::conversion::{
    ConversionCapConfig, ConversionTrackingMethod, ManualPayoutConfig, PayoutType,
};
use crate::data::elements::offer::Offer;
use crate::data::elements::offer_source::OfferSource;
use crate::data::lists::{Currency, DataURLToken, Language, Vertical};
use crate::data::work_space::Clearance;
#[cfg(feature = "backend")]
use crate::schema::*;
use crate::Country;
use chrono::{DateTime, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use url::Url;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "offers",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferModel {
    pub id: String,
    pub account_id: String,
    pub clearance: String,
    pub offer_source: String,
    pub country: String,
    pub name: String,
    pub tags: String,
    pub url: String,
    pub offer_tokens: String,
    pub conversion_tracking_method: String,
    pub payout_type: String,
    pub manual_payout_config: String,
    pub conversion_cap_config: String,
    pub payout_value: String,
    pub currency: String,
    pub language: String,
    pub vertical: String,
    pub notes: String,
    pub weight: String,
    pub archived: bool,
    pub last_updated: i64,
}

impl From<Offer> for OfferModel {
    fn from(offer: Offer) -> Self {
        Self {
            id: offer.offer_id.to_string(),
            account_id: offer.account_id.to_string(),
            clearance: serde_json::to_string(&offer.clearance).expect("H^tdfg"),
            offer_source: serde_json::to_string(&offer.offer_source).expect("H^gfds"),
            country: serde_json::to_string(&offer.country).expect("GTfsd"),
            name: offer.name,
            tags: serde_json::to_string(&offer.tags).expect("gtrsdf45"),
            url: serde_json::to_string(&offer.url).expect("Gt5fdst5"),
            offer_tokens: serde_json::to_string(&offer.offer_tokens).expect("H^drtf"),
            conversion_tracking_method: serde_json::to_string(&offer.conversion_tracking_method)
                .expect("G^tsfd"),
            payout_type: serde_json::to_string(&offer.payout_type).expect("YH6rdtf"),
            manual_payout_config: serde_json::to_string(&offer.manual_payout_config)
                .expect("Hgdfsgh"),
            conversion_cap_config: serde_json::to_string(&offer.conversion_cap_config)
                .expect("G5tsdf"),
            payout_value: serde_json::to_string(&offer.payout_value).expect("G6tsfdgf"),
            currency: serde_json::to_string(&offer.currency).expect("GH^tdsfg"),
            language: serde_json::to_string(&offer.language).expect("gfsg5dfsfg"),
            vertical: serde_json::to_string(&offer.vertical).expect("GH^tsrdfrg"),
            notes: offer.notes,
            weight: serde_json::to_string(&offer.weight).expect("G%sdfg"),
            archived: offer.archived,
            last_updated: offer.last_updated.timestamp(),
        }
    }
}

impl From<OfferModel> for Offer {
    fn from(offer_model: OfferModel) -> Self {
        Self {
            offer_id: Uuid::parse_str(&offer_model.id).expect("HG^tdfh"),
            account_id: Uuid::parse_str(&offer_model.account_id).expect("G%^tsdf"),
            clearance: serde_json::from_str(&offer_model.clearance).expect("Y%^gsdf"),
            offer_source: serde_json::from_str(&offer_model.offer_source).expect("G5t6sdfg"),
            country: serde_json::from_str(&offer_model.country).expect("G%Tsdgfg"),
            name: offer_model.name,
            tags: serde_json::from_str(&offer_model.tags).expect("HG^Tdsftg"),
            url: serde_json::from_str(&offer_model.url).expect("GT%sdfg"),
            offer_tokens: serde_json::from_str(&offer_model.offer_tokens).expect("GTsdfg"),
            conversion_tracking_method: serde_json::from_str(
                &offer_model.conversion_tracking_method,
            )
            .expect("G%Tsdfg"),
            payout_type: serde_json::from_str(&offer_model.payout_type).expect("GTrsdfg"),
            manual_payout_config: serde_json::from_str(&offer_model.manual_payout_config)
                .expect("Gt4sdfgfd"),
            conversion_cap_config: serde_json::from_str(&offer_model.conversion_cap_config)
                .expect("t5g4sfdg"),
            payout_value: serde_json::from_str(&offer_model.payout_value).expect("G%Tsdftg"),
            currency: serde_json::from_str(&offer_model.currency).expect("GTrsdfg4"),
            language: serde_json::from_str(&offer_model.language).expect("G^%sdfg"),
            vertical: serde_json::from_str(&offer_model.vertical).expect("Y^%hgwertfg"),
            notes: offer_model.notes,
            weight: serde_json::from_str(&offer_model.weight).expect("G%dsffg"),
            archived: offer_model.archived,
            last_updated: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(offer_model.last_updated, 0),
                Utc,
            ),
        }
    }
}
