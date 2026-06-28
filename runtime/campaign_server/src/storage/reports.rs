use std::collections::{BTreeMap, BTreeSet};

use ad_buy_engine_domain::{EntityRow, ReportDimensionKey, VisitEnrichment};
use chrono::{Datelike, TimeZone, Timelike, Utc};
use sqlx::{Row, SqlitePool};

use crate::error::ServerResult;
use crate::services::user_agent::{detect_browser, detect_device_type, detect_operating_system};
use crate::storage::date_filter::VisitDateFilter;

#[derive(Clone, Debug)]
struct VisitFact {
    id: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    enrichment: VisitEnrichment,
    created_at_millis: i64,
}

#[derive(Clone, Debug)]
struct ReportBucket {
    name: String,
    detail: String,
    visits: i64,
    unique_keys: BTreeSet<String>,
    updated_at_millis: i64,
}

pub async fn list_browser_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_dimension_rows(pool, date_filter, ReportDimensionKey::Browsers).await
}

pub async fn list_device_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_dimension_rows(pool, date_filter, ReportDimensionKey::DeviceTypes).await
}

pub async fn list_os_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_dimension_rows(pool, date_filter, ReportDimensionKey::OperatingSystems).await
}

pub async fn list_connection_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_dimension_rows(pool, date_filter, ReportDimensionKey::ConnectionTypes).await
}

pub async fn list_date_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_derived_rows(pool, date_filter, |visit| {
        match visit_datetime(visit.created_at_millis) {
            Some(datetime) => (
                format!(
                    "{:04}-{:02}-{:02}",
                    datetime.year(),
                    datetime.month(),
                    datetime.day()
                ),
                "UTC date".to_string(),
            ),
            None => (
                "Unknown date".to_string(),
                "Invalid visit timestamp".to_string(),
            ),
        }
    })
    .await
}

pub async fn list_dimension_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
    dimension: ReportDimensionKey,
) -> ServerResult<Vec<EntityRow>> {
    match dimension {
        ReportDimensionKey::Dates => list_date_rows(pool, date_filter).await,
        ReportDimensionKey::DayParting => list_day_parting_rows(pool, date_filter).await,
        _ => list_derived_rows(pool, date_filter, |visit| label_dimension(visit, dimension)).await,
    }
}

pub async fn list_day_parting_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_derived_rows(pool, date_filter, |visit| {
        match visit_datetime(visit.created_at_millis) {
            Some(datetime) => {
                let hour = datetime.hour();
                (
                    format!("{hour:02}:00 UTC"),
                    format!("{hour:02}:00-{hour:02}:59 UTC"),
                )
            }
            None => (
                "Unknown hour".to_string(),
                "Invalid visit timestamp".to_string(),
            ),
        }
    })
    .await
}

async fn list_derived_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
    labeler: impl Fn(&VisitFact) -> (String, String),
) -> ServerResult<Vec<EntityRow>> {
    let visits = visit_facts(pool, date_filter).await?;
    Ok(rows_by_label(&visits, labeler))
}

async fn visit_facts(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<VisitFact>> {
    let rows = sqlx::query(
        "SELECT id, ip_address, user_agent, country, region, city, timezone, postal_code,
                metro_code, asn, asn_organization, isp, connection_type, proxy_type, carrier,
                browser, browser_version, operating_system, operating_system_version,
                device_type, device_brand, device_model, created_at_millis
         FROM visits
         WHERE (? IS NULL OR created_at_millis >= ?)
            AND (? IS NULL OR created_at_millis < ?)
         ORDER BY created_at_millis DESC",
    )
    .bind(date_filter.start_at_millis)
    .bind(date_filter.start_at_millis)
    .bind(date_filter.end_at_millis)
    .bind(date_filter.end_at_millis)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(VisitFact {
                id: row.try_get("id")?,
                ip_address: row.try_get("ip_address")?,
                user_agent: row.try_get("user_agent")?,
                enrichment: VisitEnrichment {
                    country: row.try_get("country")?,
                    region: row.try_get("region")?,
                    city: row.try_get("city")?,
                    timezone: row.try_get("timezone")?,
                    postal_code: row.try_get("postal_code")?,
                    metro_code: row.try_get("metro_code")?,
                    asn: row.try_get("asn")?,
                    asn_organization: row.try_get("asn_organization")?,
                    isp: row.try_get("isp")?,
                    connection_type: row.try_get("connection_type")?,
                    proxy_type: row.try_get("proxy_type")?,
                    carrier: row.try_get("carrier")?,
                    browser: row.try_get("browser")?,
                    browser_version: row.try_get("browser_version")?,
                    operating_system: row.try_get("operating_system")?,
                    operating_system_version: row.try_get("operating_system_version")?,
                    device_type: row.try_get("device_type")?,
                    device_brand: row.try_get("device_brand")?,
                    device_model: row.try_get("device_model")?,
                },
                created_at_millis: row.try_get("created_at_millis")?,
            })
        })
        .collect()
}

fn label_dimension(visit: &VisitFact, dimension: ReportDimensionKey) -> (String, String) {
    match dimension {
        ReportDimensionKey::Browsers => label_with_user_agent_fallback(
            visit.enrichment.browser.as_deref(),
            visit.user_agent.as_deref(),
            detect_browser,
            "Browser",
        ),
        ReportDimensionKey::BrowserVersions => label_optional(
            visit.enrichment.browser_version.as_deref(),
            "Browser version",
        ),
        ReportDimensionKey::OperatingSystems => label_with_user_agent_fallback(
            visit.enrichment.operating_system.as_deref(),
            visit.user_agent.as_deref(),
            detect_operating_system,
            "Operating system",
        ),
        ReportDimensionKey::OperatingSystemVersions => label_optional(
            visit.enrichment.operating_system_version.as_deref(),
            "Operating system version",
        ),
        ReportDimensionKey::DeviceTypes => label_with_user_agent_fallback(
            visit.enrichment.device_type.as_deref(),
            visit.user_agent.as_deref(),
            detect_device_type,
            "Device type",
        ),
        ReportDimensionKey::DeviceBrands => {
            label_optional(visit.enrichment.device_brand.as_deref(), "Device brand")
        }
        ReportDimensionKey::DeviceModels => {
            label_optional(visit.enrichment.device_model.as_deref(), "Device model")
        }
        ReportDimensionKey::Countries => {
            label_optional(visit.enrichment.country.as_deref(), "Country")
        }
        ReportDimensionKey::Regions => {
            label_optional(visit.enrichment.region.as_deref(), "Region / state")
        }
        ReportDimensionKey::Cities => label_optional(visit.enrichment.city.as_deref(), "City"),
        ReportDimensionKey::Timezones => {
            label_optional(visit.enrichment.timezone.as_deref(), "Timezone")
        }
        ReportDimensionKey::PostalCodes => {
            label_optional(visit.enrichment.postal_code.as_deref(), "Postal code")
        }
        ReportDimensionKey::MetroCodes => {
            label_optional(visit.enrichment.metro_code.as_deref(), "Metro code")
        }
        ReportDimensionKey::Asns => label_optional(visit.enrichment.asn.as_deref(), "ASN"),
        ReportDimensionKey::AsnOrganizations => label_optional(
            visit.enrichment.asn_organization.as_deref(),
            "ASN organization",
        ),
        ReportDimensionKey::ConnectionTypes => label_optional(
            visit.enrichment.connection_type.as_deref(),
            "Connection type provider not configured",
        ),
        ReportDimensionKey::IspCarriers => label_optional(
            visit
                .enrichment
                .isp
                .as_deref()
                .or(visit.enrichment.carrier.as_deref()),
            "ISP / carrier provider not configured",
        ),
        ReportDimensionKey::MobileCarriers => label_optional(
            visit.enrichment.carrier.as_deref(),
            "Mobile carrier provider not configured",
        ),
        ReportDimensionKey::Proxies => label_optional(
            visit.enrichment.proxy_type.as_deref(),
            "Proxy provider not configured",
        ),
        _ => (
            "Unknown".to_string(),
            "Dimension not visit-backed".to_string(),
        ),
    }
}

fn label_with_user_agent_fallback(
    persisted_value: Option<&str>,
    user_agent: Option<&str>,
    fallback: impl Fn(&str) -> String,
    detail: &str,
) -> (String, String) {
    if let Some(value) = normalized_label(persisted_value) {
        return (value, detail.to_string());
    }
    let value = user_agent
        .map(fallback)
        .unwrap_or_else(|| "Unknown".to_string());
    (value, "Derived from user agent".to_string())
}

fn label_optional(value: Option<&str>, detail: &str) -> (String, String) {
    (
        normalized_label(value).unwrap_or_else(|| "Unknown".to_string()),
        detail.to_string(),
    )
}

fn normalized_label(value: Option<&str>) -> Option<String> {
    let value = value?.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn rows_by_label(
    visits: &[VisitFact],
    labeler: impl Fn(&VisitFact) -> (String, String),
) -> Vec<EntityRow> {
    let mut buckets = BTreeMap::<String, ReportBucket>::new();
    for visit in visits {
        let (name, detail) = labeler(visit);
        let bucket = buckets.entry(name.clone()).or_insert_with(|| ReportBucket {
            name,
            detail,
            visits: 0,
            unique_keys: BTreeSet::new(),
            updated_at_millis: visit.created_at_millis,
        });
        bucket.visits += 1;
        bucket.unique_keys.insert(unique_key(visit));
        bucket.updated_at_millis = bucket.updated_at_millis.max(visit.created_at_millis);
    }

    let mut rows = buckets
        .into_values()
        .map(|bucket| EntityRow {
            id: format!("report-{}", slug(&bucket.name)),
            name: bucket.name,
            detail: bucket.detail,
            visits: bucket.visits,
            unique_visits: i64::try_from(bucket.unique_keys.len()).unwrap_or(i64::MAX),
            updated_at_millis: bucket.updated_at_millis,
            tracking_url: None,
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        right
            .visits
            .cmp(&left.visits)
            .then_with(|| left.name.cmp(&right.name))
    });
    rows
}

fn unique_key(visit: &VisitFact) -> String {
    let ip_address = visit.ip_address.as_deref().unwrap_or_default();
    let user_agent = visit.user_agent.as_deref().unwrap_or_default();
    if ip_address.is_empty() && user_agent.is_empty() {
        return visit.id.clone();
    }
    format!(
        "{}|{}",
        visit.ip_address.as_deref().unwrap_or_default(),
        visit.user_agent.as_deref().unwrap_or_default()
    )
}

fn visit_datetime(created_at_millis: i64) -> Option<chrono::DateTime<Utc>> {
    Utc.timestamp_millis_opt(created_at_millis).single()
}

fn slug(value: &str) -> String {
    let slug = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if slug.is_empty() {
        "unknown".to_string()
    } else {
        slug
    }
}
