use std::collections::{BTreeMap, BTreeSet};

use ad_buy_engine_domain::EntityRow;
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
    list_derived_rows(pool, date_filter, |visit| {
        let name = visit
            .user_agent
            .as_deref()
            .map(detect_browser)
            .unwrap_or_else(|| "Unknown".to_string());
        (name, "Derived from user agent".to_string())
    })
    .await
}

pub async fn list_device_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_derived_rows(pool, date_filter, |visit| {
        let name = visit
            .user_agent
            .as_deref()
            .map(detect_device_type)
            .unwrap_or_else(|| "Unknown".to_string());
        (name, "Derived from user agent".to_string())
    })
    .await
}

pub async fn list_os_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_derived_rows(pool, date_filter, |visit| {
        let name = visit
            .user_agent
            .as_deref()
            .map(detect_operating_system)
            .unwrap_or_else(|| "Unknown".to_string());
        (name, "Derived from user agent".to_string())
    })
    .await
}

pub async fn list_connection_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    list_derived_rows(pool, date_filter, |_| {
        (
            "Unknown".to_string(),
            "Connection provider not configured".to_string(),
        )
    })
    .await
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
        "SELECT id, ip_address, user_agent, created_at_millis
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
                created_at_millis: row.try_get("created_at_millis")?,
            })
        })
        .collect()
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
