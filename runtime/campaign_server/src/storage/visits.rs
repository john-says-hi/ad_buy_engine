use ad_buy_engine_domain::{
    ClickMapEntry, FunnelSequence, VisitEnrichment, VisitEventType, VisitRecord,
};
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

use crate::error::{ServerError, ServerResult};
use crate::time::now_millis;

pub struct NewVisit<'a> {
    pub id: &'a str,
    pub campaign_id: &'a str,
    pub traffic_source_id: &'a str,
    pub selected_funnel_id: Option<&'a str>,
    pub selected_sequence: Option<&'a FunnelSequence>,
    pub selected_landing_page_id: Option<&'a str>,
    pub selected_offer_id: Option<&'a str>,
    pub referrer: Option<&'a str>,
    pub ip_address: Option<&'a str>,
    pub user_agent: Option<&'a str>,
    pub enrichment: &'a VisitEnrichment,
    pub query_params: &'a [(String, String)],
    pub click_map: &'a [ClickMapEntry],
    pub redirect_target: &'a str,
    pub suspicious: bool,
}

pub async fn insert_visit_with_event(
    transaction: &mut Transaction<'_, Sqlite>,
    new_visit: NewVisit<'_>,
) -> ServerResult<VisitRecord> {
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO visits
         (id, campaign_id, traffic_source_id, selected_funnel_id, selected_sequence_json,
          selected_landing_page_id, selected_offer_id, referrer, ip_address, user_agent,
          country, region, city, timezone, postal_code, metro_code, asn, asn_organization,
          isp, connection_type, proxy_type, carrier, browser, browser_version, operating_system,
          operating_system_version, device_type, device_brand, device_model, query_params_json,
          click_map_json, redirect_target, suspicious, created_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(new_visit.id)
    .bind(new_visit.campaign_id)
    .bind(new_visit.traffic_source_id)
    .bind(new_visit.selected_funnel_id)
    .bind(new_visit.selected_sequence.map(json_string).transpose()?)
    .bind(new_visit.selected_landing_page_id)
    .bind(new_visit.selected_offer_id)
    .bind(new_visit.referrer)
    .bind(new_visit.ip_address)
    .bind(new_visit.user_agent)
    .bind(new_visit.enrichment.country.as_deref())
    .bind(new_visit.enrichment.region.as_deref())
    .bind(new_visit.enrichment.city.as_deref())
    .bind(new_visit.enrichment.timezone.as_deref())
    .bind(new_visit.enrichment.postal_code.as_deref())
    .bind(new_visit.enrichment.metro_code.as_deref())
    .bind(new_visit.enrichment.asn.as_deref())
    .bind(new_visit.enrichment.asn_organization.as_deref())
    .bind(new_visit.enrichment.isp.as_deref())
    .bind(new_visit.enrichment.connection_type.as_deref())
    .bind(new_visit.enrichment.proxy_type.as_deref())
    .bind(new_visit.enrichment.carrier.as_deref())
    .bind(new_visit.enrichment.browser.as_deref())
    .bind(new_visit.enrichment.browser_version.as_deref())
    .bind(new_visit.enrichment.operating_system.as_deref())
    .bind(new_visit.enrichment.operating_system_version.as_deref())
    .bind(new_visit.enrichment.device_type.as_deref())
    .bind(new_visit.enrichment.device_brand.as_deref())
    .bind(new_visit.enrichment.device_model.as_deref())
    .bind(json_string(new_visit.query_params)?)
    .bind(json_string(new_visit.click_map)?)
    .bind(new_visit.redirect_target)
    .bind(new_visit.suspicious)
    .bind(now)
    .execute(&mut **transaction)
    .await?;

    insert_event_in_transaction(
        transaction,
        Some(new_visit.id),
        Some(new_visit.campaign_id),
        VisitEventType::CampaignClick,
        serde_json::json!({
            "redirect_target": new_visit.redirect_target,
            "selected_landing_page_id": new_visit.selected_landing_page_id,
            "selected_offer_id": new_visit.selected_offer_id,
        }),
    )
    .await?;

    Ok(VisitRecord {
        id: new_visit.id.to_string(),
        campaign_id: new_visit.campaign_id.to_string(),
        traffic_source_id: new_visit.traffic_source_id.to_string(),
        selected_funnel_id: new_visit.selected_funnel_id.map(ToOwned::to_owned),
        selected_sequence: new_visit.selected_sequence.cloned(),
        selected_landing_page_id: new_visit.selected_landing_page_id.map(ToOwned::to_owned),
        selected_offer_id: new_visit.selected_offer_id.map(ToOwned::to_owned),
        referrer: new_visit.referrer.map(ToOwned::to_owned),
        ip_address: new_visit.ip_address.map(ToOwned::to_owned),
        user_agent: new_visit.user_agent.map(ToOwned::to_owned),
        enrichment: new_visit.enrichment.clone(),
        query_params: new_visit.query_params.to_vec(),
        click_map: new_visit.click_map.to_vec(),
        redirect_target: new_visit.redirect_target.to_string(),
        suspicious: new_visit.suspicious,
        created_at_millis: now,
    })
}

pub async fn get_visit(pool: &SqlitePool, id: &str) -> ServerResult<VisitRecord> {
    let row = sqlx::query("SELECT * FROM visits WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServerError::not_found("Visit not found"))?;
    row_to_visit(row)
}

pub async fn insert_event(
    pool: &SqlitePool,
    visit_id: Option<&str>,
    campaign_id: Option<&str>,
    event_type: VisitEventType,
    event_data: serde_json::Value,
) -> ServerResult<()> {
    sqlx::query(
        "INSERT INTO visit_events
         (id, visit_id, campaign_id, event_type, event_data_json, created_at_millis)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(visit_id)
    .bind(campaign_id)
    .bind(event_type_to_str(event_type))
    .bind(json_string(&event_data)?)
    .bind(now_millis()?)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn is_unique_visit(
    pool: &SqlitePool,
    campaign_id: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> ServerResult<bool> {
    let Some(ip_address) = ip_address else {
        return Ok(false);
    };
    let Some(user_agent) = user_agent else {
        return Ok(false);
    };
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM visits
         WHERE campaign_id = ? AND ip_address = ? AND user_agent = ?",
    )
    .bind(campaign_id)
    .bind(ip_address)
    .bind(user_agent)
    .fetch_one(pool)
    .await?;
    Ok(count == 0)
}

async fn insert_event_in_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
    visit_id: Option<&str>,
    campaign_id: Option<&str>,
    event_type: VisitEventType,
    event_data: serde_json::Value,
) -> ServerResult<()> {
    sqlx::query(
        "INSERT INTO visit_events
         (id, visit_id, campaign_id, event_type, event_data_json, created_at_millis)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(visit_id)
    .bind(campaign_id)
    .bind(event_type_to_str(event_type))
    .bind(json_string(&event_data)?)
    .bind(now_millis()?)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

fn row_to_visit(row: SqliteRow) -> ServerResult<VisitRecord> {
    Ok(VisitRecord {
        id: row.try_get("id")?,
        campaign_id: row.try_get("campaign_id")?,
        traffic_source_id: row.try_get("traffic_source_id")?,
        selected_funnel_id: row.try_get("selected_funnel_id")?,
        selected_sequence: row
            .try_get::<Option<String>, _>("selected_sequence_json")?
            .map(json_value)
            .transpose()?,
        selected_landing_page_id: row.try_get("selected_landing_page_id")?,
        selected_offer_id: row.try_get("selected_offer_id")?,
        referrer: row.try_get("referrer")?,
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
        query_params: json_value(row.try_get::<String, _>("query_params_json")?)?,
        click_map: json_value(row.try_get::<String, _>("click_map_json")?)?,
        redirect_target: row.try_get("redirect_target")?,
        suspicious: row.try_get("suspicious")?,
        created_at_millis: row.try_get("created_at_millis")?,
    })
}

fn event_type_to_str(event_type: VisitEventType) -> &'static str {
    match event_type {
        VisitEventType::CampaignClick => "campaign_click",
        VisitEventType::LanderClick => "lander_click",
        VisitEventType::OfferClick => "offer_click",
        VisitEventType::Conversion => "conversion",
        VisitEventType::CustomConversion => "custom_conversion",
        VisitEventType::Error => "error",
        VisitEventType::ConditionDataMissing => "condition_data_missing",
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

pub fn new_visit_id() -> String {
    new_id()
}
