use crate::data::elements::offer::Offer;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use uuid::Uuid;
use crate::data::work_space::Clearance;
use crate::data::elements::offer_source::OfferSource;
use crate::Country;
use url::Url;
use crate::data::lists::{DataURLToken, Currency, Language, Vertical};
use crate::data::conversion::{ConversionTrackingMethod, PayoutType, ManualPayoutConfig, ConversionCapConfig};
use rust_decimal::Decimal;

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
    pub conversion_cap_config:String,
    pub payout_value: String,
    pub currency: String,
    pub language: String,
    pub vertical: String,
    pub notes: String,
    pub archived: bool,
    pub last_updated: i64,
}

impl From<Offer> for OfferModel {
    fn from(offer: Offer) -> Self {
        to_json_string!(
            id; offer.offer_id
            account_id; offer.account_id
            clearance; offer.clearance
            offer_source; offer.offer_source
            country; offer.country
            tags; offer.tags
            url; offer.url
            offer_tokens; offer.offer_tokens
            conversion_tracking_method; offer.conversion_tracking_method
            payout_type; offer.payout_type
            manual_payout_config; offer.manual_payout_config
            conversion_cap_config; offer.conversion_cap_config
            payout_value; offer.payout_type
            currency; offer.currency
            language; offer.language
            vertical; offer.vertical
        );
        
        Self {
            id,
            account_id,
            clearance,
            offer_source,
            country,
            name:offer.name,
            tags,
            url,
            offer_tokens,
            conversion_tracking_method,
            payout_type,
            manual_payout_config,
            conversion_cap_config,
            payout_value,
            currency,
            language,
            vertical,
            notes:offer.notes,
            archived: offer.archived,
            last_updated: offer.last_updated.timestamp(),
        }
    }
}

impl From<OfferModel> for Offer {
    fn from(offer_model: OfferModel) -> Self {
        from_json_string!(
            offer_id; offer_model.id => Uuid
            account_id; offer_model.account_id => Uuid
            clearance; offer_model.clearance => Clearance
            offer_source; offer_model.offer_source => OfferSource
            country; offer_model.country => Country
            tags; offer_model.tags => Vec<String>
            url; offer_model.url => Url
            offer_tokens; offer_model.offer_tokens => Vec<DataURLToken>
            conversion_tracking_method; offer_model.conversion_tracking_method => ConversionTrackingMethod
            payout_type; offer_model.payout_type => PayoutType
            manual_payout_config; offer_model.manual_payout_config => Option<ManualPayoutConfig>
            conversion_cap_config; offer_model.conversion_cap_config => Option<ConversionCapConfig>
            payout_value; offer_model.payout_value => Decimal
            currency; offer_model.currency => Currency
            language; offer_model.language => Language
            vertical; offer_model.vertical => Vertical
        );
        
        Self {
            offer_id,
            account_id,
            clearance,
            offer_source,
            country,
            name:offer_model.name,
            tags,
            url,
            offer_tokens,
            conversion_tracking_method,
            payout_type,
            manual_payout_config,
            conversion_cap_config,
            payout_value,
            currency,
            language,
            vertical,
            notes:offer_model.notes,
            archived:offer_model.archived,
            last_updated: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(offer_model.last_updated, 0), Utc),
        }
    }
}
