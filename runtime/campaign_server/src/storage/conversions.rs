use std::collections::BTreeMap;

use ad_buy_engine_domain::{
    ConversionEventCategory, ConversionEventType, ConversionEventTypeDraft,
    ConversionTrackingResponse, EntityRow, VisitEventType,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::error::{ServerError, ServerResult};
use crate::storage::date_filter::VisitDateFilter;
use crate::storage::visits::{get_visit, insert_event};
use crate::time::now_millis;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingConversion {
    pub source: &'static str,
    pub params: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
struct ParsedConversion {
    visit_id: Option<String>,
    campaign_id: String,
    event_key: String,
    raw_status: Option<String>,
    status: &'static str,
    revenue_value: f64,
    currency: String,
    transaction_id: Option<String>,
    external_event_id: Option<String>,
    identity_hash: Option<String>,
    dedupe_key: String,
    sanitized_params: Vec<(String, String)>,
}

pub async fn list_conversion_event_type_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    let rows = sqlx::query(
        "SELECT conversion_event_types.id, conversion_event_types.name,
                conversion_event_types.event_category || ' / ' || conversion_event_types.event_key AS detail,
                conversion_event_types.updated_at_millis,
                COUNT(CASE WHEN conversion_events.duplicate = 0 THEN conversion_events.id END) AS visits,
                COUNT(DISTINCT CASE WHEN conversion_events.duplicate = 0 THEN conversion_events.visit_id END) AS unique_visits
         FROM conversion_event_types
         LEFT JOIN conversion_events ON conversion_events.event_type_id = conversion_event_types.id
            AND (? IS NULL OR conversion_events.created_at_millis >= ?)
            AND (? IS NULL OR conversion_events.created_at_millis < ?)
         WHERE conversion_event_types.archived = 0
         GROUP BY conversion_event_types.id
         ORDER BY conversion_event_types.updated_at_millis DESC, conversion_event_types.name ASC",
    )
    .bind(date_filter.start_at_millis)
    .bind(date_filter.start_at_millis)
    .bind(date_filter.end_at_millis)
    .bind(date_filter.end_at_millis)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| {
            Ok(EntityRow {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                detail: row.try_get("detail")?,
                visits: row.try_get("visits")?,
                unique_visits: row.try_get("unique_visits")?,
                updated_at_millis: row.try_get("updated_at_millis")?,
                tracking_url: None,
            })
        })
        .collect()
}

pub async fn create_conversion_event_type(
    pool: &SqlitePool,
    draft: ConversionEventTypeDraft,
) -> ServerResult<ConversionEventType> {
    ensure_valid_event_type(&draft)?;
    ensure_unique_event_key(pool, None, &draft.event_key).await?;
    let id = new_id();
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO conversion_event_types
         (id, name, event_key, aliases_json, event_category, include_in_conversions,
          include_in_revenue, include_in_cost, send_postback_to_traffic_source,
          default_revenue_value, currency, notes, archived, created_at_millis, updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&draft.name)
    .bind(&draft.event_key)
    .bind(json_string(&draft.aliases)?)
    .bind(category_to_str(&draft.category))
    .bind(draft.include_in_conversions)
    .bind(draft.include_in_revenue)
    .bind(draft.include_in_cost)
    .bind(draft.send_postback_to_traffic_source)
    .bind(draft.default_revenue_value)
    .bind(&draft.currency)
    .bind(&draft.notes)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    get_conversion_event_type(pool, &id).await
}

pub async fn update_conversion_event_type(
    pool: &SqlitePool,
    id: &str,
    draft: ConversionEventTypeDraft,
) -> ServerResult<ConversionEventType> {
    ensure_valid_event_type(&draft)?;
    ensure_unique_event_key(pool, Some(id), &draft.event_key).await?;
    let now = now_millis()?;
    let result = sqlx::query(
        "UPDATE conversion_event_types SET
            name = ?, event_key = ?, aliases_json = ?, event_category = ?,
            include_in_conversions = ?, include_in_revenue = ?, include_in_cost = ?,
            send_postback_to_traffic_source = ?, default_revenue_value = ?, currency = ?,
            notes = ?, updated_at_millis = ?
         WHERE id = ? AND archived = 0",
    )
    .bind(&draft.name)
    .bind(&draft.event_key)
    .bind(json_string(&draft.aliases)?)
    .bind(category_to_str(&draft.category))
    .bind(draft.include_in_conversions)
    .bind(draft.include_in_revenue)
    .bind(draft.include_in_cost)
    .bind(draft.send_postback_to_traffic_source)
    .bind(draft.default_revenue_value)
    .bind(&draft.currency)
    .bind(&draft.notes)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    ensure_changed(result.rows_affected(), "Conversion event type not found")?;
    get_conversion_event_type(pool, id).await
}

pub async fn get_conversion_event_type(
    pool: &SqlitePool,
    id: &str,
) -> ServerResult<ConversionEventType> {
    let row = sqlx::query("SELECT * FROM conversion_event_types WHERE id = ? AND archived = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServerError::not_found("Conversion event type not found"))?;
    row_to_conversion_event_type(row)
}

pub async fn archive_conversion_event_type(pool: &SqlitePool, id: &str) -> ServerResult<()> {
    let result = sqlx::query(
        "UPDATE conversion_event_types
         SET archived = 1, updated_at_millis = ?
         WHERE id = ? AND archived = 0",
    )
    .bind(now_millis()?)
    .bind(id)
    .execute(pool)
    .await?;
    ensure_changed(result.rows_affected(), "Conversion event type not found")
}

pub async fn track_conversion(
    pool: &SqlitePool,
    incoming: IncomingConversion,
) -> ServerResult<ConversionTrackingResponse> {
    let requested_event_key = first_value(
        &incoming.params,
        &["type", "et", "event", "event_type", "conversion_type"],
    )
    .unwrap_or_else(|| "Sale".to_string());
    let event_type = resolve_event_type(pool, &requested_event_key)
        .await?
        .ok_or_else(|| ServerError::bad_request("Unknown conversion event type"))?;
    let parsed = parse_conversion(pool, &incoming, &event_type).await?;
    let duplicate = has_existing_dedupe_key(pool, &parsed.dedupe_key).await?;
    let conversion_id = new_id();
    let now = now_millis()?;

    sqlx::query(
        "INSERT INTO conversion_events
         (id, visit_id, campaign_id, event_type_id, event_key, event_name, event_category,
          status, raw_status, revenue_value, currency, transaction_id, external_event_id,
          identity_hash, dedupe_key, source, duplicate, raw_payload_json, created_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&conversion_id)
    .bind(parsed.visit_id.as_deref())
    .bind(&parsed.campaign_id)
    .bind(&event_type.id)
    .bind(&parsed.event_key)
    .bind(&event_type.name)
    .bind(category_to_str(&event_type.category))
    .bind(parsed.status)
    .bind(parsed.raw_status.as_deref())
    .bind(parsed.revenue_value)
    .bind(&parsed.currency)
    .bind(parsed.transaction_id.as_deref())
    .bind(parsed.external_event_id.as_deref())
    .bind(parsed.identity_hash.as_deref())
    .bind(&parsed.dedupe_key)
    .bind(incoming.source)
    .bind(duplicate)
    .bind(json_string(&parsed.sanitized_params)?)
    .bind(now)
    .execute(pool)
    .await?;

    insert_event(
        pool,
        parsed.visit_id.as_deref(),
        Some(&parsed.campaign_id),
        visit_event_type(&event_type.category),
        serde_json::json!({
            "conversion_id": conversion_id,
            "event_type_id": event_type.id,
            "event_key": parsed.event_key,
            "status": parsed.status,
            "duplicate": duplicate,
            "source": incoming.source,
        }),
    )
    .await?;

    Ok(ConversionTrackingResponse {
        conversion_id,
        duplicate,
        campaign_id: parsed.campaign_id,
        visit_id: parsed.visit_id,
        event_type_id: event_type.id,
        event_key: parsed.event_key,
    })
}

async fn parse_conversion(
    pool: &SqlitePool,
    incoming: &IncomingConversion,
    event_type: &ConversionEventType,
) -> ServerResult<ParsedConversion> {
    let visit_id = first_value(
        &incoming.params,
        &["cid", "clickid", "click_id", "visit_id", "subid"],
    );
    let campaign_id = if let Some(visit_id) = visit_id.as_deref() {
        get_visit(pool, visit_id).await?.campaign_id
    } else {
        let campaign_id = first_value(&incoming.params, &["campaign_id", "campaign"])
            .ok_or_else(|| ServerError::bad_request("Missing click id or campaign id"))?;
        ensure_campaign_exists(pool, &campaign_id).await?;
        campaign_id
    };
    let raw_status = first_value(&incoming.params, &["status"]);
    let status = normalized_status(raw_status.as_deref());
    let revenue_value = counted_revenue_value(incoming, event_type, status);
    let currency = first_value(&incoming.params, &["currency", "cur"])
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| event_type.currency.clone());
    let transaction_id = first_value(
        &incoming.params,
        &["txid", "transaction_id", "conversion_id", "order_id"],
    );
    let external_event_id = first_value(
        &incoming.params,
        &["external_event_id", "eventid", "rdtk_event_id"],
    )
    .or_else(|| transaction_id.clone());
    let identity_hash = identity_hash(&incoming.params);
    let sanitized_params = sanitized_params(&incoming.params);
    let dedupe_key = dedupe_key(DedupeParts {
        campaign_id: &campaign_id,
        event_type_id: &event_type.id,
        visit_id: visit_id.as_deref(),
        external_event_id: external_event_id.as_deref(),
        identity_hash: identity_hash.as_deref(),
        sanitized_params: &sanitized_params,
    });

    Ok(ParsedConversion {
        visit_id,
        campaign_id,
        event_key: event_type.event_key.clone(),
        raw_status,
        status,
        revenue_value,
        currency,
        transaction_id,
        external_event_id,
        identity_hash,
        dedupe_key,
        sanitized_params,
    })
}

fn counted_revenue_value(
    incoming: &IncomingConversion,
    event_type: &ConversionEventType,
    status: &str,
) -> f64 {
    if matches!(status, "pending" | "rejected") || !event_type.include_in_revenue {
        return 0.0;
    }
    first_value(&incoming.params, &["payout", "sum", "revenue"])
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(event_type.default_revenue_value)
}

async fn resolve_event_type(
    pool: &SqlitePool,
    requested_key: &str,
) -> ServerResult<Option<ConversionEventType>> {
    let rows = sqlx::query(
        "SELECT * FROM conversion_event_types
         WHERE archived = 0
         ORDER BY updated_at_millis DESC",
    )
    .fetch_all(pool)
    .await?;
    for row in rows {
        let event_type = row_to_conversion_event_type(row)?;
        if event_type.event_key == requested_key
            || event_type
                .aliases
                .iter()
                .any(|alias| alias == requested_key)
        {
            return Ok(Some(event_type));
        }
    }
    Ok(None)
}

async fn has_existing_dedupe_key(pool: &SqlitePool, dedupe_key: &str) -> ServerResult<bool> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM conversion_events
         WHERE dedupe_key = ? AND duplicate = 0",
    )
    .bind(dedupe_key)
    .fetch_one(pool)
    .await?;
    Ok(count > 0)
}

async fn ensure_campaign_exists(pool: &SqlitePool, campaign_id: &str) -> ServerResult<()> {
    let found: Option<String> =
        sqlx::query_scalar("SELECT id FROM campaigns WHERE id = ? AND archived = 0")
            .bind(campaign_id)
            .fetch_optional(pool)
            .await?;
    found
        .map(|_| ())
        .ok_or_else(|| ServerError::not_found("Campaign not found"))
}

async fn ensure_unique_event_key(
    pool: &SqlitePool,
    current_id: Option<&str>,
    event_key: &str,
) -> ServerResult<()> {
    let found: Option<String> =
        sqlx::query_scalar("SELECT id FROM conversion_event_types WHERE event_key = ?")
            .bind(event_key)
            .fetch_optional(pool)
            .await?;
    match (found.as_deref(), current_id) {
        (Some(found_id), Some(current_id)) if found_id == current_id => Ok(()),
        (Some(_), _) => Err(ServerError::conflict(
            "A conversion event type already uses this event key",
        )),
        (None, _) => Ok(()),
    }
}

fn ensure_valid_event_type(draft: &ConversionEventTypeDraft) -> ServerResult<()> {
    let errors = ad_buy_engine_domain::ValidateDraft::validate(draft);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(ServerError::validation("Validation failed", errors))
    }
}

fn row_to_conversion_event_type(row: SqliteRow) -> ServerResult<ConversionEventType> {
    Ok(ConversionEventType {
        id: row.try_get("id")?,
        created_at_millis: row.try_get("created_at_millis")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
        archived: row.try_get("archived")?,
        draft: ConversionEventTypeDraft {
            name: row.try_get("name")?,
            event_key: row.try_get("event_key")?,
            aliases: json_value(row.try_get::<String, _>("aliases_json")?)?,
            category: category_from_str(row.try_get::<String, _>("event_category")?)?,
            include_in_conversions: row.try_get("include_in_conversions")?,
            include_in_revenue: row.try_get("include_in_revenue")?,
            include_in_cost: row.try_get("include_in_cost")?,
            send_postback_to_traffic_source: row.try_get("send_postback_to_traffic_source")?,
            default_revenue_value: row.try_get("default_revenue_value")?,
            currency: row.try_get("currency")?,
            notes: row.try_get("notes")?,
        },
    })
}

fn category_to_str(category: &ConversionEventCategory) -> &'static str {
    match category {
        ConversionEventCategory::Lead => "lead",
        ConversionEventCategory::Sale => "sale",
        ConversionEventCategory::Custom => "custom",
    }
}

fn category_from_str(value: String) -> ServerResult<ConversionEventCategory> {
    match value.as_str() {
        "lead" => Ok(ConversionEventCategory::Lead),
        "sale" => Ok(ConversionEventCategory::Sale),
        "custom" => Ok(ConversionEventCategory::Custom),
        _ => Err(ServerError::internal(format!(
            "invalid conversion event category stored: {value}"
        ))),
    }
}

fn visit_event_type(category: &ConversionEventCategory) -> VisitEventType {
    match category {
        ConversionEventCategory::Sale => VisitEventType::Conversion,
        ConversionEventCategory::Lead | ConversionEventCategory::Custom => {
            VisitEventType::CustomConversion
        }
    }
}

fn normalized_status(raw_status: Option<&str>) -> &'static str {
    match raw_status.map(|value| value.trim().to_ascii_lowercase()) {
        Some(value)
            if matches!(
                value.as_str(),
                "approved" | "approve" | "accepted" | "confirmed" | "paid"
            ) =>
        {
            "approved"
        }
        Some(value) if matches!(value.as_str(), "pending" | "hold" | "review") => "pending",
        Some(value)
            if matches!(
                value.as_str(),
                "rejected" | "reject" | "declined" | "denied" | "failed"
            ) =>
        {
            "rejected"
        }
        _ => "unknown",
    }
}

fn first_value(params: &[(String, String)], names: &[&str]) -> Option<String> {
    for name in names {
        if let Some((_, value)) = params.iter().find(|(key, _)| key == name) {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn identity_hash(params: &[(String, String)]) -> Option<String> {
    first_value(params, &["email", "phone", "phone_number"]).map(|value| {
        let normalized = value.trim().to_ascii_lowercase();
        sha256_hex(&normalized)
    })
}

fn sanitized_params(params: &[(String, String)]) -> Vec<(String, String)> {
    params
        .iter()
        .filter(|(key, _)| !is_pii_key(key))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

fn is_pii_key(key: &str) -> bool {
    matches!(
        key.to_ascii_lowercase().as_str(),
        "email" | "phone" | "phone_number" | "first_name" | "last_name" | "name"
    )
}

struct DedupeParts<'a> {
    campaign_id: &'a str,
    event_type_id: &'a str,
    visit_id: Option<&'a str>,
    external_event_id: Option<&'a str>,
    identity_hash: Option<&'a str>,
    sanitized_params: &'a [(String, String)],
}

fn dedupe_key(parts: DedupeParts<'_>) -> String {
    if let Some(external_event_id) = parts.external_event_id {
        return format!(
            "campaign:{}|event:{}|external:{}",
            parts.campaign_id, parts.event_type_id, external_event_id
        );
    }
    if let Some(identity_hash) = parts.identity_hash {
        return format!(
            "campaign:{}|event:{}|identity:{}",
            parts.campaign_id, parts.event_type_id, identity_hash
        );
    }
    if let Some(visit_id) = parts.visit_id {
        return format!(
            "campaign:{}|event:{}|visit:{}",
            parts.campaign_id, parts.event_type_id, visit_id
        );
    }
    let payload = stable_payload(parts.sanitized_params);
    format!(
        "campaign:{}|event:{}|payload:{}",
        parts.campaign_id,
        parts.event_type_id,
        sha256_hex(&payload)
    )
}

fn stable_payload(params: &[(String, String)]) -> String {
    let mut sorted = BTreeMap::<&str, &str>::new();
    for (key, value) in params {
        sorted.insert(key.as_str(), value.as_str());
    }
    sorted
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&")
}

fn sha256_hex(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

fn ensure_changed(rows_affected: u64, message: &str) -> ServerResult<()> {
    if rows_affected == 0 {
        Err(ServerError::not_found(message))
    } else {
        Ok(())
    }
}

fn json_string<T: Serialize + ?Sized>(value: &T) -> ServerResult<String> {
    serde_json::to_string(value).map_err(ServerError::from)
}

fn json_value<T: serde::de::DeserializeOwned>(value: String) -> ServerResult<T> {
    serde_json::from_str(&value).map_err(ServerError::from)
}

fn new_id() -> String {
    Uuid::new_v4().to_string()
}
