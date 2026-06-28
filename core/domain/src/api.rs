use serde::{Deserialize, Serialize};

use crate::entities::{
    Campaign, CampaignDraft, Funnel, FunnelDraft, LandingPage, LandingPageDraft, Offer, OfferDraft,
    OfferSource, OfferSourceDraft, TrafficSource, TrafficSourceDraft,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum EntityDraft {
    OfferSource(OfferSourceDraft),
    Offer(OfferDraft),
    LandingPage(LandingPageDraft),
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
            Self::TrafficSource(record) => &record.name,
            Self::Funnel(record) => &record.name,
            Self::Campaign(record) => &record.name,
        }
    }
}
