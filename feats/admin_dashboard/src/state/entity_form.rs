use std::collections::BTreeMap;

use ad_buy_engine_domain::{
    CampaignDraft, ConditionRule, DestinationType, EntityRecord, FunnelDraft, FunnelPath,
    FunnelSequence, LandingPageDraft, OfferDraft, OfferSourceDraft, OptionItem, SequenceType,
    TrafficSourceDraft, UrlToken, WeightedReference,
};

use crate::route::Route;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntityKind {
    OfferSource,
    Offer,
    LandingPage,
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
    TrafficSources,
    Funnels,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SaveDraft {
    OfferSource(OfferSourceDraft),
    Offer(OfferDraft),
    LandingPage(LandingPageDraft),
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
            text("language", "Language", false),
            text("vertical", "Vertical", false),
            number("weight", "Weight", false),
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
            .with_text("language", "en")
            .with_text("weight", "100"),
        EntityKind::TrafficSource => FormValues::default()
            .with_text("currency", "USD")
            .with_text("external_id_parameter", "external_id")
            .with_text("cost_parameter", "cost")
            .with_toggle("direct_tracking", true),
        EntityKind::Funnel => FormValues::default()
            .with_text("country", "Global")
            .with_text("referrer_handling", "do_nothing")
            .with_text("sequence_type", "offers_only"),
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
            language: values.text("language"),
            vertical: values.text("vertical"),
            weight: parse_u32(&values.text("weight"), "Weight")?,
            notes: values.text("notes"),
        })),
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
            default_sequences: vec![sequence_from_values("default", "Default", values)],
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
                .with_text("language", draft.language)
                .with_text("vertical", draft.vertical)
                .with_text("weight", draft.weight.to_string())
                .with_text("notes", draft.notes)
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
