use serde::{Deserialize, Serialize};

use crate::entities::{
    Campaign, CampaignDraft, ConversionEventType, ConversionEventTypeDraft, Funnel, FunnelDraft,
    LandingPage, LandingPageDraft, Offer, OfferDraft, OfferSource, OfferSourceDraft, TrafficSource,
    TrafficSourceDraft,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    Validation,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub code: ApiErrorCode,
    pub message: String,
    pub details: Vec<FieldError>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardSummaryResponse {
    pub generated_at_millis: i64,
    pub current_window: DashboardDateWindow,
    pub comparison_window: Option<DashboardDateWindow>,
    pub kpis: Vec<DashboardKpi>,
    pub performance: Vec<DashboardPerformancePoint>,
    pub decision_feed: Vec<DashboardDecision>,
    pub top_movers: Vec<DashboardTopMover>,
    pub conversion_path: Vec<DashboardConversionPathStep>,
    pub traffic_mix: Vec<DashboardTrafficMix>,
    pub recent_events: Vec<DashboardRecentEvent>,
    pub setup_health: Vec<DashboardSetupHealthItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardDateWindow {
    pub label: String,
    pub start_at_millis: Option<i64>,
    pub end_at_millis: Option<i64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardMetricUnit {
    Count,
    Currency,
    Percentage,
    Ratio,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardTone {
    Neutral,
    Positive,
    Warning,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardKpi {
    pub key: String,
    pub label: String,
    pub value: f64,
    pub previous_value: Option<f64>,
    pub delta_percent: Option<f64>,
    pub unit: DashboardMetricUnit,
    pub tone: DashboardTone,
    pub estimated: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardPerformancePoint {
    pub label: String,
    pub start_at_millis: i64,
    pub visits: i64,
    pub revenue: f64,
    pub cost: f64,
    pub profit: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardDecision {
    pub title: String,
    pub detail: String,
    pub tone: DashboardTone,
    pub action_label: String,
    pub route_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTopMover {
    pub category: String,
    pub name: String,
    pub detail: String,
    pub route_path: Option<String>,
    pub visits: i64,
    pub conversions: i64,
    pub revenue: f64,
    pub cost: f64,
    pub profit: f64,
    pub roi: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardConversionPathStep {
    pub label: String,
    pub count: i64,
    pub rate_from_previous: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTrafficMix {
    pub dimension: String,
    pub segments: Vec<DashboardTrafficSegment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTrafficSegment {
    pub label: String,
    pub visits: i64,
    pub share_percent: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardRecentEvent {
    pub label: String,
    pub detail: String,
    pub occurred_at_millis: i64,
    pub tone: DashboardTone,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardSetupHealthItem {
    pub label: String,
    pub detail: String,
    pub tone: DashboardTone,
    pub route_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityRow {
    pub id: String,
    pub name: String,
    pub detail: String,
    pub visits: i64,
    pub unique_visits: i64,
    pub updated_at_millis: i64,
    pub tracking_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionItem {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionsResponse {
    pub items: Vec<OptionItem>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportDimensionKey {
    Campaigns,
    TrafficSources,
    OfferSources,
    Offers,
    Landers,
    Conversions,
    Funnels,
    Browsers,
    BrowserVersions,
    OperatingSystems,
    OperatingSystemVersions,
    DeviceTypes,
    DeviceBrands,
    DeviceModels,
    Countries,
    Regions,
    Cities,
    Timezones,
    PostalCodes,
    MetroCodes,
    Asns,
    AsnOrganizations,
    ConnectionTypes,
    IspCarriers,
    MobileCarriers,
    Proxies,
    Dates,
    DayParting,
}

impl ReportDimensionKey {
    pub const ALL: &'static [Self] = &[
        Self::TrafficSources,
        Self::OfferSources,
        Self::Browsers,
        Self::BrowserVersions,
        Self::Campaigns,
        Self::ConnectionTypes,
        Self::Countries,
        Self::Regions,
        Self::Cities,
        Self::Dates,
        Self::DayParting,
        Self::DeviceTypes,
        Self::DeviceBrands,
        Self::DeviceModels,
        Self::Conversions,
        Self::Funnels,
        Self::IspCarriers,
        Self::Landers,
        Self::MobileCarriers,
        Self::OperatingSystems,
        Self::OperatingSystemVersions,
        Self::Offers,
        Self::Proxies,
        Self::Timezones,
        Self::PostalCodes,
        Self::MetroCodes,
        Self::Asns,
        Self::AsnOrganizations,
    ];

    pub fn from_slug(value: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|dimension| dimension.slug() == value)
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Campaigns => "campaigns",
            Self::TrafficSources => "traffic-sources",
            Self::OfferSources => "offer-sources",
            Self::Offers => "offers",
            Self::Landers => "landers",
            Self::Conversions => "conversions",
            Self::Funnels => "funnels",
            Self::Browsers => "browsers",
            Self::BrowserVersions => "browser-versions",
            Self::OperatingSystems => "os",
            Self::OperatingSystemVersions => "os-versions",
            Self::DeviceTypes => "device-types",
            Self::DeviceBrands => "device-brands",
            Self::DeviceModels => "device-models",
            Self::Countries => "countries",
            Self::Regions => "regions",
            Self::Cities => "cities",
            Self::Timezones => "timezones",
            Self::PostalCodes => "postal-codes",
            Self::MetroCodes => "metro-codes",
            Self::Asns => "asns",
            Self::AsnOrganizations => "asn-organizations",
            Self::ConnectionTypes => "connection-types",
            Self::IspCarriers => "isp-carriers",
            Self::MobileCarriers => "mobile-carriers",
            Self::Proxies => "proxies",
            Self::Dates => "date",
            Self::DayParting => "day-parting",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Campaigns => "Campaigns",
            Self::TrafficSources => "Traffic Sources",
            Self::OfferSources => "Offer Sources",
            Self::Offers => "Offers",
            Self::Landers => "Landers",
            Self::Conversions => "Conversions",
            Self::Funnels => "Funnels",
            Self::Browsers => "Browsers",
            Self::BrowserVersions => "Browser Versions",
            Self::OperatingSystems => "OS",
            Self::OperatingSystemVersions => "OS Versions",
            Self::DeviceTypes => "Device Types",
            Self::DeviceBrands => "Brands",
            Self::DeviceModels => "Models",
            Self::Countries => "Countries",
            Self::Regions => "Regions / States",
            Self::Cities => "Cities",
            Self::Timezones => "Timezones",
            Self::PostalCodes => "Postal Codes",
            Self::MetroCodes => "Metro Codes",
            Self::Asns => "ASNs",
            Self::AsnOrganizations => "ASN Organizations",
            Self::ConnectionTypes => "Connection Types",
            Self::IspCarriers => "ISP / Carriers",
            Self::MobileCarriers => "Mobile Carriers",
            Self::Proxies => "Proxies",
            Self::Dates => "Day",
            Self::DayParting => "Hour of Day",
        }
    }

    pub const fn route_path(self) -> Option<&'static str> {
        match self {
            Self::Campaigns => Some("/campaigns"),
            Self::TrafficSources => Some("/traffic-sources"),
            Self::OfferSources => Some("/offer-sources"),
            Self::Offers => Some("/offers"),
            Self::Landers => Some("/landers"),
            Self::Conversions => Some("/conversions"),
            Self::Funnels => Some("/funnels"),
            Self::Browsers => Some("/browsers"),
            Self::OperatingSystems => Some("/os"),
            Self::DeviceTypes => Some("/device"),
            Self::ConnectionTypes => Some("/connection"),
            Self::Dates => Some("/date"),
            Self::DayParting => Some("/day-parting"),
            _ => None,
        }
    }

    pub const fn supported(self) -> bool {
        !matches!(
            self,
            Self::ConnectionTypes | Self::IspCarriers | Self::MobileCarriers | Self::Proxies
        )
    }

    pub const fn detail(self) -> &'static str {
        match self {
            Self::Countries | Self::Regions | Self::Cities | Self::Timezones => {
                "GeoLite City database"
            }
            Self::PostalCodes | Self::MetroCodes => "GeoLite City database when available",
            Self::Asns | Self::AsnOrganizations => "GeoLite ASN database",
            Self::ConnectionTypes => "Requires paid MaxMind Connection Type database",
            Self::IspCarriers => "Requires paid MaxMind ISP or Carrier database",
            Self::MobileCarriers => "Requires paid MaxMind Carrier database",
            Self::Proxies => "Requires paid MaxMind Anonymous IP database",
            Self::BrowserVersions
            | Self::Browsers
            | Self::OperatingSystems
            | Self::OperatingSystemVersions
            | Self::DeviceTypes
            | Self::DeviceBrands
            | Self::DeviceModels => "User agent",
            Self::Dates | Self::DayParting => "Visit timestamp",
            Self::Conversions => "Tracked conversion event type",
            _ => "Tracked entity",
        }
    }

    pub fn metadata(self) -> ReportDimension {
        ReportDimension {
            key: self,
            label: self.label().to_string(),
            route_path: self.route_path().map(ToOwned::to_owned),
            supported: self.supported(),
            detail: self.detail().to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportDimension {
    pub key: ReportDimensionKey,
    pub label: String,
    pub route_path: Option<String>,
    pub supported: bool,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeolocationSettingsResponse {
    pub account_id: String,
    pub license_key_configured: bool,
    pub license_key_preview: Option<String>,
    pub city_database_path: String,
    pub country_database_path: String,
    pub asn_database_path: String,
    pub city_database: GeolocationDatabaseStatus,
    pub country_database: GeolocationDatabaseStatus,
    pub asn_database: GeolocationDatabaseStatus,
    pub updated_at_millis: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeolocationSettingsUpdate {
    pub account_id: String,
    pub license_key: Option<String>,
    pub city_database_path: String,
    pub country_database_path: String,
    pub asn_database_path: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeolocationDatabaseStatus {
    pub configured_path: String,
    pub exists: bool,
    pub database_type: Option<String>,
    pub build_epoch: Option<u64>,
    pub last_loaded_at_millis: Option<i64>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeolocationDownloadResponse {
    pub downloaded: Vec<GeolocationDownloadedDatabase>,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeolocationDownloadedDatabase {
    pub edition_id: String,
    pub path: String,
    pub database_type: String,
    pub build_epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainSetupStatus {
    NotConfigured,
    Configured,
}

impl DomainSetupStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotConfigured => "not_configured",
            Self::Configured => "configured",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainSettingsResponse {
    pub primary_tracking_domain: String,
    pub tracking_base_url: String,
    pub admin_dashboard_domain: String,
    pub admin_dashboard_base_url: String,
    pub domain_setup_status: DomainSetupStatus,
    pub updated_at_millis: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainSettingsUpdate {
    pub primary_tracking_domain: String,
    pub admin_dashboard_domain: String,
}

impl DomainSettingsUpdate {
    pub fn from_primary_domain(primary_domain: String) -> Self {
        Self {
            primary_tracking_domain: primary_domain.clone(),
            admin_dashboard_domain: primary_domain,
        }
    }

    pub fn validate(&self) -> Vec<FieldError> {
        [
            validate_domain("primary_tracking_domain", &self.primary_tracking_domain),
            validate_domain("admin_dashboard_domain", &self.admin_dashboard_domain),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn normalized_primary_tracking_domain(&self) -> String {
        normalized_domain(&self.primary_tracking_domain)
    }

    pub fn normalized_admin_dashboard_domain(&self) -> String {
        normalized_domain(&self.admin_dashboard_domain)
    }

    pub fn tracking_base_url(&self) -> String {
        base_url_from_domain(&self.primary_tracking_domain)
    }

    pub fn admin_dashboard_base_url(&self) -> String {
        base_url_from_domain(&self.admin_dashboard_domain)
    }
}

fn validate_domain(field: &'static str, value: &str) -> Option<FieldError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Some(field_error(field, "Domain is required"));
    }
    if trimmed.len() != value.len() || trimmed.chars().any(char::is_whitespace) {
        return Some(field_error(field, "Domain must not contain whitespace"));
    }
    if trimmed.contains("://")
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || trimmed.contains(':')
        || trimmed.contains('?')
        || trimmed.contains('#')
        || trimmed.contains('@')
    {
        return Some(field_error(
            field,
            "Enter a hostname without a scheme, path, or port",
        ));
    }
    if trimmed.len() > 253 {
        return Some(field_error(field, "Domain is too long"));
    }
    if trimmed.split('.').any(|label| !valid_hostname_label(label)) {
        return Some(field_error(field, "Domain is not a valid hostname"));
    }
    None
}

fn valid_hostname_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 63
        && !label.starts_with('-')
        && !label.ends_with('-')
        && label
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
}

fn normalized_domain(value: &str) -> String {
    value.trim().trim_end_matches('.').to_ascii_lowercase()
}

fn base_url_from_domain(value: &str) -> String {
    format!("https://{}", normalized_domain(value))
}

fn field_error(field: impl Into<String>, message: impl Into<String>) -> FieldError {
    FieldError {
        field: field.into(),
        message: message.into(),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum EntityDraft {
    OfferSource(OfferSourceDraft),
    Offer(OfferDraft),
    LandingPage(LandingPageDraft),
    ConversionEventType(ConversionEventTypeDraft),
    TrafficSource(TrafficSourceDraft),
    Funnel(FunnelDraft),
    Campaign(CampaignDraft),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum EntityRecord {
    OfferSource(OfferSource),
    Offer(Offer),
    LandingPage(LandingPage),
    ConversionEventType(ConversionEventType),
    TrafficSource(TrafficSource),
    Funnel(Funnel),
    Campaign(Campaign),
}

impl EntityRecord {
    pub fn id(&self) -> &str {
        match self {
            Self::OfferSource(record) => &record.id,
            Self::Offer(record) => &record.id,
            Self::LandingPage(record) => &record.id,
            Self::ConversionEventType(record) => &record.id,
            Self::TrafficSource(record) => &record.id,
            Self::Funnel(record) => &record.id,
            Self::Campaign(record) => &record.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::OfferSource(record) => &record.name,
            Self::Offer(record) => &record.name,
            Self::LandingPage(record) => &record.name,
            Self::ConversionEventType(record) => &record.name,
            Self::TrafficSource(record) => &record.name,
            Self::Funnel(record) => &record.name,
            Self::Campaign(record) => &record.name,
        }
    }
}
