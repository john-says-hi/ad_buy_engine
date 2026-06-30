use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use ad_buy_engine_domain::{FakeAffiliateOffer, FakeAffiliateOfferKind};
use chrono::Utc;
use thiserror::Error;

use crate::config::{ConfigError, MAX_THRESHOLD, RunConfig, validate_postback_template};
use crate::conversions::{
    APPROVED_STATUS, payout_string, threshold_for_offer, transaction_id_for_threshold,
};
use crate::macros::{PostbackMacros, render_postback_url};
use crate::postback::{DeliveryStatus, PostbackDelivery};

const TRACKING_IDENTIFIER_ALIASES: &[&str] = &["subid", "cid", "clickid", "click_id", "visit_id"];
const KNOWN_CLICK_PARAMETERS: &[&str] = &[
    "subid",
    "cid",
    "clickid",
    "click_id",
    "visit_id",
    "source",
    "utm_source",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeSettings {
    pub postback_template: String,
    pub lead_threshold: u32,
    pub sale_threshold: u32,
}

impl RuntimeSettings {
    pub fn from_config(config: &RunConfig) -> Self {
        Self {
            postback_template: config.postback_template.clone(),
            lead_threshold: config.lead_threshold,
            sale_threshold: config.sale_threshold,
        }
    }

    pub fn try_new(
        postback_template: String,
        lead_threshold: u32,
        sale_threshold: u32,
    ) -> Result<Self, ConfigError> {
        let template_url = validate_postback_template(&postback_template)?;
        if template_url.host_str().is_none() {
            return Err(ConfigError::InvalidPostbackTemplate(
                "template must include a host".to_string(),
            ));
        }
        if lead_threshold == 0 || lead_threshold > MAX_THRESHOLD {
            return Err(ConfigError::NumericRange {
                field: "lead-threshold",
                min: "1".to_string(),
                max: MAX_THRESHOLD.to_string(),
                actual: lead_threshold.to_string(),
            });
        }
        if sale_threshold == 0 || sale_threshold > MAX_THRESHOLD {
            return Err(ConfigError::NumericRange {
                field: "sale-threshold",
                min: "1".to_string(),
                max: MAX_THRESHOLD.to_string(),
                actual: sale_threshold.to_string(),
            });
        }
        Ok(Self {
            postback_template,
            lead_threshold,
            sale_threshold,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClickRecord {
    pub id: u64,
    pub offer_id: String,
    pub tracking_identifier: Option<String>,
    pub source_parameter: Option<String>,
    pub known_parameters: Vec<(String, String)>,
    pub created_at_millis: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClickSummary {
    pub offer_id: String,
    pub tracking_identifier: Option<String>,
    pub clicks: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversionRecord {
    pub id: u64,
    pub origin: ConversionOrigin,
    pub offer_id: String,
    pub event_type: String,
    pub payout: String,
    pub currency: String,
    pub status: String,
    pub transaction_id: String,
    pub tracking_identifier: String,
    pub callback_url: String,
    pub delivery: PostbackDelivery,
    pub created_at_millis: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConversionOrigin {
    Threshold,
    Sample,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClickOutcome {
    pub click: ClickRecord,
    pub generated_conversion: Option<ConversionRecord>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkSnapshot {
    pub settings: RuntimeSettings,
    pub clicks: Vec<ClickRecord>,
    pub click_summaries: Vec<ClickSummary>,
    pub conversions: Vec<ConversionRecord>,
}

#[derive(Clone, Debug)]
pub struct NetworkState {
    inner: Arc<Mutex<StateInner>>,
}

#[derive(Debug)]
struct StateInner {
    settings: RuntimeSettings,
    next_click_id: u64,
    next_conversion_id: u64,
    clicks: Vec<ClickRecord>,
    conversions: Vec<ConversionRecord>,
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("fake network state lock is poisoned")]
    LockPoisoned,
    #[error("failed to render postback URL: {0}")]
    Macro(String),
    #[error("conversion record {0} was not found")]
    ConversionNotFound(u64),
}

impl NetworkState {
    pub fn new(settings: RuntimeSettings) -> Self {
        Self {
            inner: Arc::new(Mutex::new(StateInner {
                settings,
                next_click_id: 1,
                next_conversion_id: 1,
                clicks: Vec::new(),
                conversions: Vec::new(),
            })),
        }
    }

    pub fn snapshot(&self) -> Result<NetworkSnapshot, StateError> {
        let inner = self.lock()?;
        Ok(NetworkSnapshot {
            settings: inner.settings.clone(),
            clicks: inner.clicks.clone(),
            click_summaries: click_summaries(&inner.clicks),
            conversions: inner.conversions.clone(),
        })
    }

    pub fn update_settings(&self, settings: RuntimeSettings) -> Result<(), StateError> {
        let mut inner = self.lock()?;
        inner.settings = settings;
        Ok(())
    }

    pub fn record_click(
        &self,
        offer: FakeAffiliateOffer,
        params: &HashMap<String, String>,
    ) -> Result<ClickOutcome, StateError> {
        let mut inner = self.lock()?;
        let tracking_identifier = tracking_identifier(params);
        let source_parameter = tracking_source_parameter(params);
        let known_parameters = known_click_parameters(params);
        let click = ClickRecord {
            id: inner.next_click_id,
            offer_id: offer.id.to_string(),
            tracking_identifier: tracking_identifier.clone(),
            source_parameter,
            known_parameters,
            created_at_millis: now_millis(),
        };
        inner.next_click_id += 1;
        inner.clicks.push(click.clone());

        let generated_conversion = if tracking_identifier.is_some() {
            threshold_conversion_for_click(&mut inner, offer, tracking_identifier.as_deref())?
        } else {
            None
        };

        Ok(ClickOutcome {
            click,
            generated_conversion,
        })
    }

    pub fn record_sample_conversion(
        &self,
        kind: FakeAffiliateOfferKind,
        tracking_identifier: &str,
    ) -> Result<ConversionRecord, StateError> {
        let mut inner = self.lock()?;
        let offer = sample_offer(kind);
        let transaction_id = format!(
            "fan-sample-{}-{}",
            offer.event_type().to_ascii_lowercase(),
            inner.next_conversion_id
        );
        let conversion = build_conversion_record(
            &mut inner,
            offer,
            tracking_identifier,
            ConversionOrigin::Sample,
            Some(transaction_id),
        )?;
        inner.conversions.push(conversion.clone());
        Ok(conversion)
    }

    pub fn update_conversion_delivery(
        &self,
        conversion_id: u64,
        delivery: PostbackDelivery,
    ) -> Result<(), StateError> {
        let mut inner = self.lock()?;
        let Some(conversion) = inner
            .conversions
            .iter_mut()
            .find(|conversion| conversion.id == conversion_id)
        else {
            return Err(StateError::ConversionNotFound(conversion_id));
        };
        conversion.delivery = delivery;
        Ok(())
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, StateInner>, StateError> {
        self.inner.lock().map_err(|_| StateError::LockPoisoned)
    }
}

fn threshold_conversion_for_click(
    inner: &mut StateInner,
    offer: FakeAffiliateOffer,
    tracking_identifier: Option<&str>,
) -> Result<Option<ConversionRecord>, StateError> {
    let qualifying_click_count = inner
        .clicks
        .iter()
        .filter(|click| click.offer_id == offer.id && click.tracking_identifier.is_some())
        .count() as u64;
    let threshold = u64::from(threshold_for_offer(
        offer,
        inner.settings.lead_threshold,
        inner.settings.sale_threshold,
    ));
    let Some(tracking_identifier) = tracking_identifier else {
        return Ok(None);
    };
    if qualifying_click_count == 0 || !qualifying_click_count.is_multiple_of(threshold) {
        return Ok(None);
    }

    let conversion = build_conversion_record(
        inner,
        offer,
        tracking_identifier,
        ConversionOrigin::Threshold,
        Some(transaction_id_for_threshold(offer, qualifying_click_count)),
    )?;
    inner.conversions.push(conversion.clone());
    Ok(Some(conversion))
}

fn build_conversion_record(
    inner: &mut StateInner,
    offer: FakeAffiliateOffer,
    tracking_identifier: &str,
    origin: ConversionOrigin,
    transaction_id: Option<String>,
) -> Result<ConversionRecord, StateError> {
    let id = inner.next_conversion_id;
    inner.next_conversion_id += 1;
    let payout = payout_string(offer);
    let transaction_id = transaction_id.unwrap_or_else(|| transaction_id_for_threshold(offer, id));
    let macros = PostbackMacros {
        click_id: tracking_identifier,
        event_type: offer.event_type(),
        payout: &payout,
        currency: offer.currency,
        status: APPROVED_STATUS,
        transaction_id: &transaction_id,
    };
    let callback_url = render_postback_url(&inner.settings.postback_template, &macros)
        .map_err(|error| StateError::Macro(error.to_string()))?;

    Ok(ConversionRecord {
        id,
        origin,
        offer_id: offer.id.to_string(),
        event_type: offer.event_type().to_string(),
        payout,
        currency: offer.currency.to_string(),
        status: APPROVED_STATUS.to_string(),
        transaction_id,
        tracking_identifier: tracking_identifier.to_string(),
        callback_url: callback_url.to_string(),
        delivery: PostbackDelivery::pending(),
        created_at_millis: now_millis(),
    })
}

fn sample_offer(kind: FakeAffiliateOfferKind) -> FakeAffiliateOffer {
    for offer in ad_buy_engine_domain::fake_affiliate_catalog() {
        if offer.kind == kind {
            return *offer;
        }
    }
    if let Some(offer) = ad_buy_engine_domain::fake_affiliate_catalog().first() {
        return *offer;
    }
    FakeAffiliateOffer {
        id: "fake-sample-fallback",
        name: "Fake Sample Fallback",
        kind,
        payout_value: 1.0,
        currency: "USD",
        vertical: "fake-fallback",
        default_threshold: 1,
        display_copy: "Fallback fake offer used only if the shared catalog is empty.",
    }
}

fn tracking_identifier(params: &HashMap<String, String>) -> Option<String> {
    TRACKING_IDENTIFIER_ALIASES
        .iter()
        .find_map(|alias| cleaned_param(params, alias))
}

fn tracking_source_parameter(params: &HashMap<String, String>) -> Option<String> {
    TRACKING_IDENTIFIER_ALIASES
        .iter()
        .find(|alias| cleaned_param(params, alias).is_some())
        .map(|alias| (*alias).to_string())
}

fn known_click_parameters(params: &HashMap<String, String>) -> Vec<(String, String)> {
    KNOWN_CLICK_PARAMETERS
        .iter()
        .filter_map(|name| cleaned_param(params, name).map(|value| ((*name).to_string(), value)))
        .collect()
}

fn cleaned_param(params: &HashMap<String, String>, name: &str) -> Option<String> {
    params.get(name).and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn click_summaries(clicks: &[ClickRecord]) -> Vec<ClickSummary> {
    let mut grouped = BTreeMap::<(String, Option<String>), usize>::new();
    for click in clicks {
        *grouped
            .entry((click.offer_id.clone(), click.tracking_identifier.clone()))
            .or_insert(0) += 1;
    }
    grouped
        .into_iter()
        .map(|((offer_id, tracking_identifier), clicks)| ClickSummary {
            offer_id,
            tracking_identifier,
            clicks,
        })
        .collect()
}

fn now_millis() -> i64 {
    Utc::now().timestamp_millis()
}

pub fn visible_delivery_label(status: &DeliveryStatus) -> &'static str {
    status.label()
}
