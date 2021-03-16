use crate::data::elements::offer_source::OfferSource;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use uuid::Uuid;
use crate::data::work_space::Clearance;
use crate::data::custom_events::CustomConversionEventToken;
use url::Url;
use crate::data::conversion::{ConversionTrackingMethod, WhiteListedPostbackIPs};
use crate::data::lists::Currency;
use crate::data::lists::referrer_handling::ReferrerHandling;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "offer_source_table",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferSourceModel {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub clearance: String,
    pub click_id_token: String,
    pub payout_token: String,
    pub conversion_id_token: String,
    pub custom_events: String,
    pub tracking_domain: String,
    pub conversion_tracking_method: String,
    pub include_additional_parameters_in_postback_url: bool,
    pub payout_currency: String,
    pub append_click_id: bool,
    pub accept_duplicate_post_backs: bool,
    pub whitelisted_postback_ips: String,
    pub referrer_handling: String,
    pub notes: String,
    pub archived: bool,
    pub last_updated: i64,
}

impl From<OfferSource> for OfferSourceModel {
    fn from(offer_source: OfferSource) -> Self {
        to_json_string!(
            id; offer_source.offer_source_id
            account_id; offer_source.account_id
            clearance; offer_source.clearance
            custom_events; offer_source.custom_events
            tracking_domain; offer_source.tracking_domain
            conversion_tracking_method; offer_source.conversion_tracking_method
            payout_currency; offer_source.payout_currency
            whitelisted_postback_ips; offer_source.whitelisted_postback_ips
            referrer_handling; offer_source.referrer_handling
        );
        
        Self {
            id,
            account_id,
            name:offer_source.name,
            clearance,
            click_id_token:offer_source.click_id_token,
            payout_token:offer_source.payout_token,
            conversion_id_token:offer_source.conversion_id_token,
            custom_events,
            tracking_domain,
            conversion_tracking_method,
            include_additional_parameters_in_postback_url:offer_source.include_additional_parameters_in_postback_url,
            payout_currency,
            append_click_id:offer_source.append_click_id,
            accept_duplicate_post_backs:offer_source.accept_duplicate_post_backs,
            whitelisted_postback_ips,
            referrer_handling,
            notes:offer_source.notes,
            archived:offer_source.archived,
            last_updated: offer_source.last_updated.timestamp(),
        }
    }
}

impl From<OfferSourceModel> for OfferSource {
    fn from(offer_source_model: OfferSourceModel) -> Self {
    from_json_string!(
        offer_source_id; offer_source_model.id => Uuid
        account_id; offer_source_model.account_id => Uuid
        clearance; offer_source_model.clearance => Clearance
        custom_events; offer_source_model.custom_events => Vec<CustomConversionEventToken>
        tracking_domain; offer_source_model.tracking_domain => Url
        conversion_tracking_method; offer_source_model.conversion_tracking_method =>ConversionTrackingMethod
        payout_currency; offer_source_model.payout_currency => Currency
        whitelisted_postback_ips; offer_source_model.whitelisted_postback_ips => WhiteListedPostbackIPs
        referrer_handling; offer_source_model.referrer_handling => ReferrerHandling
    );
    
    Self {
        offer_source_id,
        account_id,
        name:offer_source_model.name,
        clearance,
        click_id_token:offer_source_model.click_id_token,
        payout_token:offer_source_model.payout_token,
        conversion_id_token:offer_source_model.conversion_id_token,
        custom_events,
        tracking_domain,
        conversion_tracking_method,
        include_additional_parameters_in_postback_url:offer_source_model.include_additional_parameters_in_postback_url,
        payout_currency,
        append_click_id:offer_source_model.append_click_id,
        accept_duplicate_post_backs:offer_source_model.accept_duplicate_post_backs,
        whitelisted_postback_ips,
        referrer_handling,
        notes:offer_source_model.notes,
        archived:offer_source_model.archived,
        last_updated:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(offer_source_model.last_updated, 0), Utc),
    }
    }
}
