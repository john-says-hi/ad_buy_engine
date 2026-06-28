use std::collections::BTreeMap;

use ad_buy_engine_domain::{
    CampaignDraft, ConditionRule, ConversionEventCategory, ConversionEventTypeDraft,
    DestinationType, EntityRecord, FunnelDraft, FunnelPath, FunnelSequence, LandingPageDraft,
    LandingPageRole, OfferDraft, OfferSourceDraft, OptionItem, SequenceType, TrafficSourceDraft,
    UrlToken, WeightedReference,
};

use crate::route::Route;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntityKind {
    OfferSource,
    Offer,
    LandingPage,
    ConversionEventType,
    TrafficSource,
    Funnel,
    Campaign,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FormValues {
    pub text: BTreeMap<String, String>,
    pub toggles: BTreeMap<String, bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FormOptionLists {
    pub offer_sources: Vec<OptionItem>,
    pub offers: Vec<OptionItem>,
    pub landing_pages: Vec<OptionItem>,
    pub conversion_events: Vec<OptionItem>,
    pub traffic_sources: Vec<OptionItem>,
    pub funnels: Vec<OptionItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormFieldSpec {
    pub key: &'static str,
    pub label: &'static str,
    pub field_type: FieldType,
    pub wide: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldType {
    Text,
    Number,
    Decimal,
    TextArea,
    Toggle,
    Select(SelectSource),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectSource {
    Static(&'static [&'static str]),
    OfferSources,
    Offers,
    LandingPages,
    ConversionEvents,
    TrafficSources,
    Funnels,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SaveDraft {
    OfferSource(OfferSourceDraft),
    Offer(OfferDraft),
    LandingPage(LandingPageDraft),
    ConversionEventType(ConversionEventTypeDraft),
    TrafficSource(TrafficSourceDraft),
    Funnel(FunnelDraft),
    Campaign(CampaignDraft),
}

impl EntityKind {
    pub const fn from_route(route: Route) -> Option<Self> {
        match route.render_route() {
            Route::OfferSources => Some(Self::OfferSource),
            Route::Offers => Some(Self::Offer),
            Route::Landers => Some(Self::LandingPage),
            Route::Conversions => Some(Self::ConversionEventType),
            Route::TrafficSources => Some(Self::TrafficSource),
            Route::Funnels => Some(Self::Funnel),
            Route::Campaigns => Some(Self::Campaign),
            _ => None,
        }
    }

    pub const fn endpoint(self) -> &'static str {
        match self {
            Self::OfferSource => "/api/offer-sources",
            Self::Offer => "/api/offers",
            Self::LandingPage => "/api/landers",
            Self::ConversionEventType => "/api/conversions",
            Self::TrafficSource => "/api/traffic-sources",
            Self::Funnel => "/api/funnels",
            Self::Campaign => "/api/campaigns",
        }
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::OfferSource => "New Offer Source",
            Self::Offer => "New Offer",
            Self::LandingPage => "New Lander",
            Self::ConversionEventType => "New Conversion Event",
            Self::TrafficSource => "New Traffic Source",
            Self::Funnel => "Create Funnel",
            Self::Campaign => "Create Campaign",
        }
    }
}

impl FormValues {
    pub fn with_text(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.text.insert(key.into(), value.into());
        self
    }

    pub fn with_toggle(mut self, key: impl Into<String>, value: bool) -> Self {
        self.toggles.insert(key.into(), value);
        self
    }

    pub fn set_text(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.text.insert(key.into(), value.into());
    }

    pub fn set_toggle(&mut self, key: impl Into<String>, value: bool) {
        self.toggles.insert(key.into(), value);
    }

    pub fn text(&self, key: &str) -> String {
        self.text.get(key).cloned().unwrap_or_default()
    }

    pub fn toggle(&self, key: &str) -> bool {
        self.toggles.get(key).copied().unwrap_or(false)
    }
}

impl FormOptionLists {
    pub fn for_source(&self, source: SelectSource) -> Vec<OptionItem> {
        match source {
            SelectSource::Static(values) => values
                .iter()
                .map(|value| OptionItem {
                    value: (*value).to_string(),
                    label: (*value).to_string(),
                })
                .collect(),
            SelectSource::OfferSources => self.offer_sources.clone(),
            SelectSource::Offers => self.offers.clone(),
            SelectSource::LandingPages => self.landing_pages.clone(),
            SelectSource::ConversionEvents => self.conversion_events.clone(),
            SelectSource::TrafficSources => self.traffic_sources.clone(),
            SelectSource::Funnels => self.funnels.clone(),
        }
    }
}

pub fn form_fields(kind: EntityKind) -> Vec<FormFieldSpec> {
    match kind {
        EntityKind::OfferSource => vec![
            text("name", "Name of Source:", false),
            text("tracking_domain", "Tracking Domain", false),
            select(
                "tracking_method",
                "Tracking Method",
                SelectSource::Static(TRACKING_METHODS),
                false,
            ),
            select(
                "payout_currency",
                "Payout Currency",
                SelectSource::Static(CURRENCIES),
                false,
            ),
            text_area("postback_url", "Postback URL", true),
            toggle("append_click_id", "Append Click ID"),
            toggle("accept_duplicate_postbacks", "Accept Duplicate Postbacks"),
            text("whitelist_postback_ips", "Whitelisted IPs", true),
            select(
                "referrer_handling",
                "Referrer Handling",
                SelectSource::Static(REFERRER_HANDLING),
                false,
            ),
            text_area("notes", "Notes", true),
        ],
        EntityKind::Offer => vec![
            select(
                "offer_source_id",
                "Link Offer Source:",
                SelectSource::OfferSources,
                false,
            ),
            select("country", "Country", SelectSource::Static(COUNTRIES), false),
            text("name", "Offer Name:", false),
            text("tags", "Tags", false),
            text("url", "Offer URL", true),
            select(
                "payout_model",
                "Payout Type",
                SelectSource::Static(PAYOUT_MODELS),
                false,
            ),
            decimal("payout_value", "Payout Value", false),
            select(
                "currency",
                "Payout Currency",
                SelectSource::Static(CURRENCIES),
                false,
            ),
            text("language", "Language", false),
            text("vertical", "Vertical", false),
            number("weight", "Weight", false),
            text_area("notes", "Notes", true),
        ],
        EntityKind::LandingPage => vec![
            select("country", "Country", SelectSource::Static(COUNTRIES), false),
            text("name", "Lander Name:", false),
            text("tags", "Tags", false),
            text("url", "Lander URL", true),
            number("cta_count", "Number of CTAs:", false),
            select(
                "role",
                "Lander Role",
                SelectSource::Static(LANDING_PAGE_ROLES),
                false,
            ),
            text(
                "expected_conversion_event_type_ids",
                "Expected Conversion Event IDs",
                true,
            ),
            text("language", "Language", false),
            text("vertical", "Vertical", false),
            number("weight", "Weight", false),
            text_area("notes", "Notes", true),
        ],
        EntityKind::ConversionEventType => vec![
            text("name", "Event Name", false),
            text("event_key", "Event Key", false),
            text("aliases", "Accepted Aliases", true),
            select(
                "category",
                "Event Category",
                SelectSource::Static(CONVERSION_EVENT_CATEGORIES),
                false,
            ),
            toggle("include_in_conversions", "Include in Conversions"),
            toggle("include_in_revenue", "Include in Revenue"),
            toggle("include_in_cost", "Include in Cost"),
            toggle(
                "send_postback_to_traffic_source",
                "Send Postback to Traffic Source",
            ),
            decimal("default_revenue_value", "Default Revenue", false),
            select(
                "currency",
                "Currency",
                SelectSource::Static(CURRENCIES),
                false,
            ),
            text_area("notes", "Notes", true),
        ],
        EntityKind::TrafficSource => vec![
            text("name", "Traffic Source Name:", false),
            select(
                "currency",
                "Cost Currency",
                SelectSource::Static(CURRENCIES),
                false,
            ),
            text("external_id_parameter", "External ID Parameter", false),
            text("cost_parameter", "Cost Parameter", false),
            text("custom_parameters", "Custom Parameters", true),
            text_area("postback_urls", "Postback URLs", true),
            text("pixel_url", "Pixel URL", true),
            toggle("track_impressions", "Track Impressions"),
            toggle("direct_tracking", "Direct Tracking"),
            text_area("notes", "Notes", true),
        ],
        EntityKind::Funnel => vec![
            text("name", "Funnel Name", false),
            select("country", "Country", SelectSource::Static(COUNTRIES), false),
            select(
                "referrer_handling",
                "Referrer Handling",
                SelectSource::Static(REFERRER_HANDLING),
                false,
            ),
            select(
                "funnel_template",
                "Funnel Template",
                SelectSource::Static(FUNNEL_TEMPLATES),
                false,
            ),
            select(
                "sequence_type",
                "Default Sequence Type",
                SelectSource::Static(SEQUENCE_TYPES),
                false,
            ),
            select(
                "landing_page_id",
                "Default Lander",
                SelectSource::LandingPages,
                false,
            ),
            select("offer_id", "Default Offer", SelectSource::Offers, false),
            select(
                "lead_capture_landing_page_id",
                "Lead Capture Lander",
                SelectSource::LandingPages,
                false,
            ),
            select(
                "advertorial_landing_page_id",
                "Advertorial Lander",
                SelectSource::LandingPages,
                false,
            ),
            select("sales_offer_id", "Sales Offer", SelectSource::Offers, false),
            number("direct_sales_weight", "Direct Sales Weight", false),
            number("advertorial_weight", "Advertorial Weight", false),
            text("condition_query_key", "Conditional Query Key", false),
            text("condition_query_value", "Conditional Query Value", false),
            text_area("notes", "Notes", true),
        ],
        EntityKind::Campaign => vec![
            select(
                "traffic_source_id",
                "Traffic Source",
                SelectSource::TrafficSources,
                false,
            ),
            select("country", "Country", SelectSource::Static(COUNTRIES), false),
            text("name", "Name", false),
            select(
                "cost_model",
                "Cost Model",
                SelectSource::Static(COST_MODELS),
                false,
            ),
            decimal("cost_value", "Cost Value", false),
            select(
                "destination_type",
                "Destination Type",
                SelectSource::Static(DESTINATION_TYPES),
                false,
            ),
            select("funnel_id", "Select Funnel", SelectSource::Funnels, false),
            select(
                "direct_offer_id",
                "Direct Offer",
                SelectSource::Offers,
                false,
            ),
            text_area("notes", "Notes", true),
        ],
    }
}

pub fn default_values(kind: EntityKind) -> FormValues {
    match kind {
        EntityKind::OfferSource => FormValues::default()
            .with_text("tracking_domain", "main")
            .with_text("tracking_method", "postback")
            .with_text("payout_currency", "USD")
            .with_text("referrer_handling", "do_nothing"),
        EntityKind::Offer => FormValues::default()
            .with_text("country", "Global")
            .with_text("payout_model", "fixed")
            .with_text("payout_value", "0")
            .with_text("currency", "USD")
            .with_text("language", "en")
            .with_text("weight", "100"),
        EntityKind::LandingPage => FormValues::default()
            .with_text("country", "Global")
            .with_text("cta_count", "1")
            .with_text("role", "standard")
            .with_text("language", "en")
            .with_text("weight", "100"),
        EntityKind::ConversionEventType => FormValues::default()
            .with_text("category", "custom")
            .with_text("currency", "USD")
            .with_text("default_revenue_value", "0")
            .with_toggle("include_in_conversions", true)
            .with_toggle("send_postback_to_traffic_source", true),
        EntityKind::TrafficSource => FormValues::default()
            .with_text("currency", "USD")
            .with_text("external_id_parameter", "external_id")
            .with_text("cost_parameter", "cost")
            .with_toggle("direct_tracking", true),
        EntityKind::Funnel => FormValues::default()
            .with_text("country", "Global")
            .with_text("referrer_handling", "do_nothing")
            .with_text("funnel_template", "simple")
            .with_text("sequence_type", "offers_only")
            .with_text("direct_sales_weight", "50")
            .with_text("advertorial_weight", "50"),
        EntityKind::Campaign => FormValues::default()
            .with_text("country", "Global")
            .with_text("cost_model", "CPC")
            .with_text("cost_value", "0")
            .with_text("destination_type", "funnel"),
    }
}

pub fn draft_from_values(kind: EntityKind, values: &FormValues) -> Result<SaveDraft, String> {
    match kind {
        EntityKind::OfferSource => Ok(SaveDraft::OfferSource(OfferSourceDraft {
            name: values.text("name"),
            tokens: default_offer_source_tokens(),
            tracking_domain: values.text("tracking_domain"),
            tracking_method: values.text("tracking_method"),
            payout_currency: values.text("payout_currency"),
            postback_url: values.text("postback_url"),
            append_click_id: values.toggle("append_click_id"),
            accept_duplicate_postbacks: values.toggle("accept_duplicate_postbacks"),
            whitelist_postback_ips: split_csv(&values.text("whitelist_postback_ips")),
            referrer_handling: values.text("referrer_handling"),
            notes: values.text("notes"),
        })),
        EntityKind::Offer => Ok(SaveDraft::Offer(OfferDraft {
            offer_source_id: values.text("offer_source_id"),
            country: values.text("country"),
            name: values.text("name"),
            tags: split_csv(&values.text("tags")),
            url: values.text("url"),
            url_tokens: default_url_tokens(),
            payout_model: values.text("payout_model"),
            payout_value: parse_f64(&values.text("payout_value"), "Payout value")?,
            currency: values.text("currency"),
            language: values.text("language"),
            vertical: values.text("vertical"),
            weight: parse_u32(&values.text("weight"), "Weight")?,
            notes: values.text("notes"),
        })),
        EntityKind::LandingPage => Ok(SaveDraft::LandingPage(LandingPageDraft {
            country: values.text("country"),
            name: values.text("name"),
            tags: split_csv(&values.text("tags")),
            url: values.text("url"),
            url_tokens: default_url_tokens(),
            cta_count: parse_u8(&values.text("cta_count"), "CTA count")?,
            role: landing_page_role_from_value(&values.text("role")),
            expected_conversion_event_type_ids: split_csv(
                &values.text("expected_conversion_event_type_ids"),
            ),
            language: values.text("language"),
            vertical: values.text("vertical"),
            weight: parse_u32(&values.text("weight"), "Weight")?,
            notes: values.text("notes"),
        })),
        EntityKind::ConversionEventType => {
            Ok(SaveDraft::ConversionEventType(ConversionEventTypeDraft {
                name: values.text("name"),
                event_key: values.text("event_key"),
                aliases: split_csv(&values.text("aliases")),
                category: conversion_event_category_from_value(&values.text("category")),
                include_in_conversions: values.toggle("include_in_conversions"),
                include_in_revenue: values.toggle("include_in_revenue"),
                include_in_cost: values.toggle("include_in_cost"),
                send_postback_to_traffic_source: values.toggle("send_postback_to_traffic_source"),
                default_revenue_value: parse_f64(
                    &values.text("default_revenue_value"),
                    "Default revenue",
                )?,
                currency: values.text("currency"),
                notes: values.text("notes"),
            }))
        }
        EntityKind::TrafficSource => Ok(SaveDraft::TrafficSource(TrafficSourceDraft {
            name: values.text("name"),
            external_id_parameter: values.text("external_id_parameter"),
            cost_parameter: values.text("cost_parameter"),
            custom_parameters: custom_parameters(&values.text("custom_parameters")),
            currency: values.text("currency"),
            postback_urls: split_lines(&values.text("postback_urls")),
            pixel_url: values.text("pixel_url"),
            track_impressions: values.toggle("track_impressions"),
            direct_tracking: values.toggle("direct_tracking"),
            notes: values.text("notes"),
        })),
        EntityKind::Funnel => Ok(SaveDraft::Funnel(FunnelDraft {
            country: values.text("country"),
            name: values.text("name"),
            redirect_handling: "default".to_string(),
            referrer_handling: values.text("referrer_handling"),
            conditional_sequences: conditional_sequences(values),
            default_sequences: vec![default_funnel_sequence(values)?],
            notes: values.text("notes"),
        })),
        EntityKind::Campaign => {
            let destination_type = if values.text("destination_type") == "direct_sequence" {
                DestinationType::DirectSequence
            } else {
                DestinationType::Funnel
            };
            let direct_sequence = matches!(destination_type, DestinationType::DirectSequence)
                .then(|| campaign_direct_sequence(values));
            Ok(SaveDraft::Campaign(CampaignDraft {
                traffic_source_id: values.text("traffic_source_id"),
                destination_type,
                funnel_id: empty_to_none(values.text("funnel_id")),
                direct_sequence,
                cost_model: values.text("cost_model"),
                cost_value: parse_f64(&values.text("cost_value"), "Cost value")?,
                country: values.text("country"),
                name: values.text("name"),
                notes: values.text("notes"),
            }))
        }
    }
}

pub fn values_from_record(record: EntityRecord) -> FormValues {
    match record {
        EntityRecord::OfferSource(record) => {
            let draft = record.draft;
            FormValues::default()
                .with_text("name", draft.name)
                .with_text("tracking_domain", draft.tracking_domain)
                .with_text("tracking_method", draft.tracking_method)
                .with_text("payout_currency", draft.payout_currency)
                .with_text("postback_url", draft.postback_url)
                .with_text(
                    "whitelist_postback_ips",
                    draft.whitelist_postback_ips.join(", "),
                )
                .with_text("referrer_handling", draft.referrer_handling)
                .with_text("notes", draft.notes)
                .with_toggle("append_click_id", draft.append_click_id)
                .with_toggle(
                    "accept_duplicate_postbacks",
                    draft.accept_duplicate_postbacks,
                )
        }
        EntityRecord::Offer(record) => {
            let draft = record.draft;
            FormValues::default()
                .with_text("offer_source_id", draft.offer_source_id)
                .with_text("country", draft.country)
                .with_text("name", draft.name)
                .with_text("tags", draft.tags.join(", "))
                .with_text("url", draft.url)
                .with_text("payout_model", draft.payout_model)
                .with_text("payout_value", draft.payout_value.to_string())
                .with_text("currency", draft.currency)
                .with_text("language", draft.language)
                .with_text("vertical", draft.vertical)
                .with_text("weight", draft.weight.to_string())
                .with_text("notes", draft.notes)
        }
        EntityRecord::LandingPage(record) => {
            let draft = record.draft;
            FormValues::default()
                .with_text("country", draft.country)
                .with_text("name", draft.name)
                .with_text("tags", draft.tags.join(", "))
                .with_text("url", draft.url)
                .with_text("cta_count", draft.cta_count.to_string())
                .with_text("role", landing_page_role_value(&draft.role))
                .with_text(
                    "expected_conversion_event_type_ids",
                    draft.expected_conversion_event_type_ids.join(", "),
                )
                .with_text("language", draft.language)
                .with_text("vertical", draft.vertical)
                .with_text("weight", draft.weight.to_string())
                .with_text("notes", draft.notes)
        }
        EntityRecord::ConversionEventType(record) => {
            let draft = record.draft;
            FormValues::default()
                .with_text("name", draft.name)
                .with_text("event_key", draft.event_key)
                .with_text("aliases", draft.aliases.join(", "))
                .with_text("category", conversion_event_category_value(&draft.category))
                .with_text(
                    "default_revenue_value",
                    draft.default_revenue_value.to_string(),
                )
                .with_text("currency", draft.currency)
                .with_text("notes", draft.notes)
                .with_toggle("include_in_conversions", draft.include_in_conversions)
                .with_toggle("include_in_revenue", draft.include_in_revenue)
                .with_toggle("include_in_cost", draft.include_in_cost)
                .with_toggle(
                    "send_postback_to_traffic_source",
                    draft.send_postback_to_traffic_source,
                )
        }
        EntityRecord::TrafficSource(record) => {
            let draft = record.draft;
            FormValues::default()
                .with_text("name", draft.name)
                .with_text("external_id_parameter", draft.external_id_parameter)
                .with_text("cost_parameter", draft.cost_parameter)
                .with_text(
                    "custom_parameters",
                    tokens_to_text(&draft.custom_parameters),
                )
                .with_text("currency", draft.currency)
                .with_text("postback_urls", draft.postback_urls.join("\n"))
                .with_text("pixel_url", draft.pixel_url)
                .with_text("notes", draft.notes)
                .with_toggle("track_impressions", draft.track_impressions)
                .with_toggle("direct_tracking", draft.direct_tracking)
        }
        EntityRecord::Funnel(record) => {
            let draft = record.draft;
            let sequence = draft.default_sequences.first();
            let path = sequence.and_then(|sequence| sequence.paths.first());
            FormValues::default()
                .with_text("country", draft.country)
                .with_text("name", draft.name)
                .with_text("referrer_handling", draft.referrer_handling)
                .with_text("funnel_template", "simple")
                .with_text(
                    "sequence_type",
                    sequence
                        .map(|sequence| sequence_type_value(&sequence.sequence_type))
                        .unwrap_or("offers_only"),
                )
                .with_text(
                    "landing_page_id",
                    path.and_then(|path| path.landing_page_id.clone())
                        .unwrap_or_default(),
                )
                .with_text(
                    "offer_id",
                    path.and_then(|path| path.offers.first())
                        .map(|offer| offer.id.clone())
                        .unwrap_or_default(),
                )
                .with_text("notes", draft.notes)
        }
        EntityRecord::Campaign(record) => {
            let draft = record.draft;
            FormValues::default()
                .with_text("traffic_source_id", draft.traffic_source_id)
                .with_text("country", draft.country)
                .with_text("name", draft.name)
                .with_text("cost_model", draft.cost_model)
                .with_text("cost_value", draft.cost_value.to_string())
                .with_text(
                    "destination_type",
                    destination_type_value(&draft.destination_type),
                )
                .with_text("funnel_id", draft.funnel_id.unwrap_or_default())
                .with_text(
                    "direct_offer_id",
                    draft
                        .direct_sequence
                        .as_ref()
                        .and_then(|sequence| sequence.paths.first())
                        .and_then(|path| path.offers.first())
                        .map(|offer| offer.id.clone())
                        .unwrap_or_default(),
                )
                .with_text("notes", draft.notes)
        }
    }
}

const COUNTRIES: &[&str] = &["Global", "US", "CA", "GB"];
const CURRENCIES: &[&str] = &["USD", "CAD", "EUR", "GBP"];
const TRACKING_METHODS: &[&str] = &["postback", "pixel"];
const REFERRER_HANDLING: &[&str] = &["do_nothing", "hide_referrer", "pass_referrer"];
const LANDING_PAGE_ROLES: &[&str] = &["standard", "lead_capture", "advertorial", "after_optin"];
const CONVERSION_EVENT_CATEGORIES: &[&str] = &["lead", "sale", "custom"];
const FUNNEL_TEMPLATES: &[&str] = &["simple", "lead_capture_split"];
const PAYOUT_MODELS: &[&str] = &["fixed", "percentage", "auto", "none"];
const COST_MODELS: &[&str] = &["CPC", "CPA", "CPM", "RevShare", "Not Tracked"];
const SEQUENCE_TYPES: &[&str] = &["offers_only", "landing_page_and_offers", "matrix"];
const DESTINATION_TYPES: &[&str] = &["funnel", "direct_sequence"];

fn text(key: &'static str, label: &'static str, wide: bool) -> FormFieldSpec {
    field(key, label, FieldType::Text, wide)
}

fn number(key: &'static str, label: &'static str, wide: bool) -> FormFieldSpec {
    field(key, label, FieldType::Number, wide)
}

fn decimal(key: &'static str, label: &'static str, wide: bool) -> FormFieldSpec {
    field(key, label, FieldType::Decimal, wide)
}

fn text_area(key: &'static str, label: &'static str, wide: bool) -> FormFieldSpec {
    field(key, label, FieldType::TextArea, wide)
}

fn toggle(key: &'static str, label: &'static str) -> FormFieldSpec {
    field(key, label, FieldType::Toggle, false)
}

fn select(
    key: &'static str,
    label: &'static str,
    source: SelectSource,
    wide: bool,
) -> FormFieldSpec {
    field(key, label, FieldType::Select(source), wide)
}

fn field(
    key: &'static str,
    label: &'static str,
    field_type: FieldType,
    wide: bool,
) -> FormFieldSpec {
    FormFieldSpec {
        key,
        label,
        field_type,
        wide,
    }
}

fn default_funnel_sequence(values: &FormValues) -> Result<FunnelSequence, String> {
    if values.text("funnel_template") == "lead_capture_split" {
        lead_capture_split_sequence(values)
    } else {
        Ok(sequence_from_values("default", "Default", values))
    }
}

fn lead_capture_split_sequence(values: &FormValues) -> Result<FunnelSequence, String> {
    let lead_capture_landing_page_id = required_value(
        values,
        "lead_capture_landing_page_id",
        "Lead capture lander",
    )?;
    let advertorial_landing_page_id =
        required_value(values, "advertorial_landing_page_id", "Advertorial lander")?;
    let sales_offer_id = required_value(values, "sales_offer_id", "Sales offer")?;
    let direct_sales_weight =
        parse_u32(&values.text("direct_sales_weight"), "Direct sales weight")?;
    let advertorial_weight = parse_u32(&values.text("advertorial_weight"), "Advertorial weight")?;

    Ok(FunnelSequence {
        id: "lead-capture-split".to_string(),
        name: "Lead Capture Split".to_string(),
        active: true,
        weight: 100,
        sequence_type: SequenceType::Matrix,
        conditions: Vec::new(),
        paths: vec![FunnelPath {
            id: "lead-capture".to_string(),
            weight: 100,
            landing_page_id: Some(lead_capture_landing_page_id),
            offers: Vec::new(),
            children: vec![
                FunnelPath {
                    id: "direct-sales".to_string(),
                    weight: direct_sales_weight,
                    landing_page_id: None,
                    offers: vec![WeightedReference {
                        id: sales_offer_id.clone(),
                        weight: 100,
                    }],
                    children: Vec::new(),
                },
                FunnelPath {
                    id: "advertorial".to_string(),
                    weight: advertorial_weight,
                    landing_page_id: Some(advertorial_landing_page_id),
                    offers: Vec::new(),
                    children: vec![FunnelPath {
                        id: "advertorial-sales".to_string(),
                        weight: 100,
                        landing_page_id: None,
                        offers: vec![WeightedReference {
                            id: sales_offer_id,
                            weight: 100,
                        }],
                        children: Vec::new(),
                    }],
                },
            ],
        }],
    })
}

fn sequence_from_values(id: &str, name: &str, values: &FormValues) -> FunnelSequence {
    let sequence_type = sequence_type_from_value(&values.text("sequence_type"));
    FunnelSequence {
        id: id.to_string(),
        name: name.to_string(),
        active: true,
        weight: 100,
        sequence_type: sequence_type.clone(),
        conditions: Vec::new(),
        paths: vec![FunnelPath {
            id: format!("{id}-path"),
            weight: 100,
            landing_page_id: matches!(
                sequence_type,
                SequenceType::LandingPageAndOffers | SequenceType::Matrix
            )
            .then(|| values.text("landing_page_id"))
            .and_then(empty_to_none),
            offers: empty_to_none(values.text("offer_id"))
                .map(|offer_id| {
                    vec![WeightedReference {
                        id: offer_id,
                        weight: 100,
                    }]
                })
                .unwrap_or_default(),
            children: Vec::new(),
        }],
    }
}

fn campaign_direct_sequence(values: &FormValues) -> FunnelSequence {
    let mut direct_values = values.clone();
    direct_values.set_text("offer_id", values.text("direct_offer_id"));
    direct_values.set_text("sequence_type", "offers_only");
    sequence_from_values("direct", "Direct Sequence", &direct_values)
}

fn conditional_sequences(values: &FormValues) -> Vec<FunnelSequence> {
    let key = values.text("condition_query_key");
    let value = values.text("condition_query_value");
    if key.trim().is_empty() || value.trim().is_empty() {
        return Vec::new();
    }

    let mut sequence = sequence_from_values("conditional", "Conditional Query", values);
    sequence.conditions = vec![ConditionRule::query_parameter("condition-1", key, value)];
    vec![sequence]
}

fn sequence_type_from_value(value: &str) -> SequenceType {
    match value {
        "landing_page_and_offers" => SequenceType::LandingPageAndOffers,
        "matrix" => SequenceType::Matrix,
        _ => SequenceType::OffersOnly,
    }
}

fn sequence_type_value(sequence_type: &SequenceType) -> &'static str {
    match sequence_type {
        SequenceType::OffersOnly => "offers_only",
        SequenceType::LandingPageAndOffers => "landing_page_and_offers",
        SequenceType::Matrix => "matrix",
    }
}

fn destination_type_value(destination_type: &DestinationType) -> &'static str {
    match destination_type {
        DestinationType::Funnel => "funnel",
        DestinationType::DirectSequence => "direct_sequence",
    }
}

fn landing_page_role_from_value(value: &str) -> LandingPageRole {
    match value {
        "lead_capture" => LandingPageRole::LeadCapture,
        "advertorial" => LandingPageRole::Advertorial,
        "after_optin" => LandingPageRole::AfterOptin,
        _ => LandingPageRole::Standard,
    }
}

fn landing_page_role_value(role: &LandingPageRole) -> &'static str {
    match role {
        LandingPageRole::Standard => "standard",
        LandingPageRole::LeadCapture => "lead_capture",
        LandingPageRole::Advertorial => "advertorial",
        LandingPageRole::AfterOptin => "after_optin",
    }
}

fn conversion_event_category_from_value(value: &str) -> ConversionEventCategory {
    match value {
        "lead" => ConversionEventCategory::Lead,
        "sale" => ConversionEventCategory::Sale,
        _ => ConversionEventCategory::Custom,
    }
}

fn conversion_event_category_value(category: &ConversionEventCategory) -> &'static str {
    match category {
        ConversionEventCategory::Lead => "lead",
        ConversionEventCategory::Sale => "sale",
        ConversionEventCategory::Custom => "custom",
    }
}

fn split_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn split_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn custom_parameters(value: &str) -> Vec<UrlToken> {
    split_csv(value)
        .into_iter()
        .map(|name| UrlToken {
            token: format!("{{{name}}}"),
            name,
        })
        .collect()
}

fn tokens_to_text(tokens: &[UrlToken]) -> String {
    tokens
        .iter()
        .map(|token| token.name.clone())
        .collect::<Vec<_>>()
        .join(", ")
}

fn default_url_tokens() -> Vec<UrlToken> {
    vec![UrlToken {
        name: "clickid".to_string(),
        token: "{clickid}".to_string(),
    }]
}

fn default_offer_source_tokens() -> Vec<UrlToken> {
    vec![
        UrlToken {
            name: "click_id".to_string(),
            token: "{click_id}".to_string(),
        },
        UrlToken {
            name: "payout".to_string(),
            token: "{payout}".to_string(),
        },
        UrlToken {
            name: "conversion_id".to_string(),
            token: "{conversion_id}".to_string(),
        },
    ]
}

fn parse_f64(value: &str, label: &str) -> Result<f64, String> {
    value
        .trim()
        .parse()
        .map_err(|_| format!("{label} must be a number"))
}

fn parse_u32(value: &str, label: &str) -> Result<u32, String> {
    value
        .trim()
        .parse()
        .map_err(|_| format!("{label} must be a whole number"))
}

fn parse_u8(value: &str, label: &str) -> Result<u8, String> {
    value
        .trim()
        .parse()
        .map_err(|_| format!("{label} must be a whole number"))
}

fn empty_to_none(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn required_value(values: &FormValues, key: &str, label: &str) -> Result<String, String> {
    empty_to_none(values.text(key)).ok_or_else(|| format!("{label} is required"))
}
