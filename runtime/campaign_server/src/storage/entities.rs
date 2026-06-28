use ad_buy_engine_domain::{
    Campaign, CampaignDraft, CampaignLinkInfo, DestinationType, EntityRow, FieldError, Funnel,
    FunnelDraft, FunnelSequence, LandingPage, LandingPageDraft, Offer, OfferDraft, OfferSource,
    OfferSourceDraft, OptionItem, TrafficSource, TrafficSourceDraft, ValidateDraft,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use sqlx::query::Query;
use sqlx::sqlite::{SqliteArguments, SqliteRow};
use sqlx::{Row, Sqlite, SqlitePool};
use uuid::Uuid;

use crate::error::{ServerError, ServerResult};
use crate::storage::date_filter::VisitDateFilter;
use crate::time::now_millis;

pub async fn list_offer_source_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
        "SELECT offer_sources.id, offer_sources.name, offer_sources.tracking_method AS detail,
                offer_sources.updated_at_millis,
                COUNT(visits.id) AS visits,
                COUNT(DISTINCT CASE WHEN visits.id IS NULL THEN NULL ELSE COALESCE(visits.ip_address, '') || '|' || COALESCE(visits.user_agent, '') END) AS unique_visits
         FROM offer_sources
         LEFT JOIN offers ON offers.offer_source_id = offer_sources.id
         LEFT JOIN visits ON visits.selected_offer_id = offers.id
            AND (? IS NULL OR visits.created_at_millis >= ?)
            AND (? IS NULL OR visits.created_at_millis < ?)
         WHERE offer_sources.archived = 0
         GROUP BY offer_sources.id
         ORDER BY offer_sources.updated_at_millis DESC, offer_sources.name ASC",
        ),
        date_filter,
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(counted_entity_row).collect()
}

pub async fn list_offer_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
        "SELECT offers.id, offers.name, offer_sources.name AS detail, offers.updated_at_millis,
                COUNT(visits.id) AS visits,
                COUNT(DISTINCT CASE WHEN visits.id IS NULL THEN NULL ELSE COALESCE(visits.ip_address, '') || '|' || COALESCE(visits.user_agent, '') END) AS unique_visits
         FROM offers
         JOIN offer_sources ON offer_sources.id = offers.offer_source_id
         LEFT JOIN visits ON visits.selected_offer_id = offers.id
            AND (? IS NULL OR visits.created_at_millis >= ?)
            AND (? IS NULL OR visits.created_at_millis < ?)
         WHERE offers.archived = 0
         GROUP BY offers.id
         ORDER BY offers.updated_at_millis DESC, offers.name ASC",
        ),
        date_filter,
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(counted_entity_row).collect()
}

pub async fn list_landing_page_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
        "SELECT landing_pages.id, landing_pages.name,
                landing_pages.country || ' / ' || landing_pages.cta_count || ' CTA' AS detail,
                landing_pages.updated_at_millis,
                COUNT(visits.id) AS visits,
                COUNT(DISTINCT CASE WHEN visits.id IS NULL THEN NULL ELSE COALESCE(visits.ip_address, '') || '|' || COALESCE(visits.user_agent, '') END) AS unique_visits
         FROM landing_pages
         LEFT JOIN visits ON visits.selected_landing_page_id = landing_pages.id
            AND (? IS NULL OR visits.created_at_millis >= ?)
            AND (? IS NULL OR visits.created_at_millis < ?)
         WHERE landing_pages.archived = 0
         GROUP BY landing_pages.id
         ORDER BY landing_pages.updated_at_millis DESC, landing_pages.name ASC",
        ),
        date_filter,
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(counted_entity_row).collect()
}

pub async fn list_traffic_source_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
        "SELECT traffic_sources.id, traffic_sources.name, traffic_sources.currency AS detail,
                traffic_sources.updated_at_millis,
                COUNT(visits.id) AS visits,
                COUNT(DISTINCT CASE WHEN visits.id IS NULL THEN NULL ELSE COALESCE(visits.ip_address, '') || '|' || COALESCE(visits.user_agent, '') END) AS unique_visits
         FROM traffic_sources
         LEFT JOIN visits ON visits.traffic_source_id = traffic_sources.id
            AND (? IS NULL OR visits.created_at_millis >= ?)
            AND (? IS NULL OR visits.created_at_millis < ?)
         WHERE traffic_sources.archived = 0
         GROUP BY traffic_sources.id
         ORDER BY traffic_sources.updated_at_millis DESC, traffic_sources.name ASC",
        ),
        date_filter,
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(counted_entity_row).collect()
}

pub async fn list_funnel_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
        "SELECT funnels.id, funnels.name, funnels.country AS detail, funnels.updated_at_millis,
                COUNT(visits.id) AS visits,
                COUNT(DISTINCT CASE WHEN visits.id IS NULL THEN NULL ELSE COALESCE(visits.ip_address, '') || '|' || COALESCE(visits.user_agent, '') END) AS unique_visits
         FROM funnels
         LEFT JOIN visits ON visits.selected_funnel_id = funnels.id
            AND (? IS NULL OR visits.created_at_millis >= ?)
            AND (? IS NULL OR visits.created_at_millis < ?)
         WHERE funnels.archived = 0
         GROUP BY funnels.id
         ORDER BY funnels.updated_at_millis DESC, funnels.name ASC",
        ),
        date_filter,
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(counted_entity_row).collect()
}

pub async fn list_campaign_rows(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<EntityRow>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
        "SELECT campaigns.id, campaigns.name, traffic_sources.name AS detail,
                campaigns.updated_at_millis, campaigns.tracking_url,
                COUNT(visits.id) AS visits,
                COUNT(DISTINCT CASE WHEN visits.id IS NULL THEN NULL ELSE COALESCE(visits.ip_address, '') || '|' || COALESCE(visits.user_agent, '') END) AS unique_visits
         FROM campaigns
         JOIN traffic_sources ON traffic_sources.id = campaigns.traffic_source_id
         LEFT JOIN visits ON visits.campaign_id = campaigns.id
            AND (? IS NULL OR visits.created_at_millis >= ?)
            AND (? IS NULL OR visits.created_at_millis < ?)
         WHERE campaigns.archived = 0
         GROUP BY campaigns.id
         ORDER BY campaigns.updated_at_millis DESC, campaigns.name ASC",
        ),
        date_filter,
    )
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
                tracking_url: row.try_get("tracking_url")?,
            })
        })
        .collect()
}

pub async fn option_items(pool: &SqlitePool, table: &str) -> ServerResult<Vec<OptionItem>> {
    let sql = match table {
        "offer-sources" => {
            "SELECT id, name FROM offer_sources WHERE archived = 0 ORDER BY name ASC"
        }
        "offers" => "SELECT id, name FROM offers WHERE archived = 0 ORDER BY name ASC",
        "landers" => "SELECT id, name FROM landing_pages WHERE archived = 0 ORDER BY name ASC",
        "traffic-sources" => {
            "SELECT id, name FROM traffic_sources WHERE archived = 0 ORDER BY name ASC"
        }
        "funnels" => "SELECT id, name FROM funnels WHERE archived = 0 ORDER BY name ASC",
        _ => return Err(ServerError::not_found("Unknown options list")),
    };
    let rows = sqlx::query(sql).fetch_all(pool).await?;
    rows.into_iter()
        .map(|row| {
            Ok(OptionItem {
                value: row.try_get("id")?,
                label: row.try_get("name")?,
            })
        })
        .collect()
}

pub async fn create_offer_source(
    pool: &SqlitePool,
    draft: OfferSourceDraft,
) -> ServerResult<OfferSource> {
    ensure_valid(&draft)?;
    let id = new_id();
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO offer_sources
         (id, name, tokens_json, tracking_domain, tracking_method, payout_currency, postback_url,
          append_click_id, accept_duplicate_postbacks, whitelist_postback_ips_json,
          referrer_handling, notes, archived, created_at_millis, updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&draft.name)
    .bind(json_string(&draft.tokens)?)
    .bind(&draft.tracking_domain)
    .bind(&draft.tracking_method)
    .bind(&draft.payout_currency)
    .bind(&draft.postback_url)
    .bind(draft.append_click_id)
    .bind(draft.accept_duplicate_postbacks)
    .bind(json_string(&draft.whitelist_postback_ips)?)
    .bind(&draft.referrer_handling)
    .bind(&draft.notes)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    get_offer_source(pool, &id).await
}

pub async fn update_offer_source(
    pool: &SqlitePool,
    id: &str,
    draft: OfferSourceDraft,
) -> ServerResult<OfferSource> {
    ensure_valid(&draft)?;
    let now = now_millis()?;
    let result = sqlx::query(
        "UPDATE offer_sources SET
            name = ?, tokens_json = ?, tracking_domain = ?, tracking_method = ?,
            payout_currency = ?, postback_url = ?, append_click_id = ?,
            accept_duplicate_postbacks = ?, whitelist_postback_ips_json = ?,
            referrer_handling = ?, notes = ?, updated_at_millis = ?
         WHERE id = ? AND archived = 0",
    )
    .bind(&draft.name)
    .bind(json_string(&draft.tokens)?)
    .bind(&draft.tracking_domain)
    .bind(&draft.tracking_method)
    .bind(&draft.payout_currency)
    .bind(&draft.postback_url)
    .bind(draft.append_click_id)
    .bind(draft.accept_duplicate_postbacks)
    .bind(json_string(&draft.whitelist_postback_ips)?)
    .bind(&draft.referrer_handling)
    .bind(&draft.notes)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    ensure_changed(result.rows_affected(), "Offer source not found")?;
    get_offer_source(pool, id).await
}

pub async fn get_offer_source(pool: &SqlitePool, id: &str) -> ServerResult<OfferSource> {
    let row = sqlx::query("SELECT * FROM offer_sources WHERE id = ? AND archived = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServerError::not_found("Offer source not found"))?;
    row_to_offer_source(row)
}

pub async fn archive_offer_source(pool: &SqlitePool, id: &str) -> ServerResult<()> {
    archive_entity(pool, "offer_sources", id, "Offer source not found").await
}

pub async fn create_offer(pool: &SqlitePool, draft: OfferDraft) -> ServerResult<Offer> {
    ensure_valid(&draft)?;
    ensure_offer_source_exists(pool, &draft.offer_source_id).await?;
    let id = new_id();
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO offers
         (id, offer_source_id, country, name, tags_json, url, url_tokens_json, payout_model,
          payout_value, currency, language, vertical, weight, notes, archived, created_at_millis,
          updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&draft.offer_source_id)
    .bind(&draft.country)
    .bind(&draft.name)
    .bind(json_string(&draft.tags)?)
    .bind(&draft.url)
    .bind(json_string(&draft.url_tokens)?)
    .bind(&draft.payout_model)
    .bind(draft.payout_value)
    .bind(&draft.currency)
    .bind(&draft.language)
    .bind(&draft.vertical)
    .bind(i64::from(draft.weight))
    .bind(&draft.notes)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    get_offer(pool, &id).await
}

pub async fn update_offer(pool: &SqlitePool, id: &str, draft: OfferDraft) -> ServerResult<Offer> {
    ensure_valid(&draft)?;
    ensure_offer_source_exists(pool, &draft.offer_source_id).await?;
    let now = now_millis()?;
    let result = sqlx::query(
        "UPDATE offers SET
            offer_source_id = ?, country = ?, name = ?, tags_json = ?, url = ?,
            url_tokens_json = ?, payout_model = ?, payout_value = ?, currency = ?,
            language = ?, vertical = ?, weight = ?, notes = ?, updated_at_millis = ?
         WHERE id = ? AND archived = 0",
    )
    .bind(&draft.offer_source_id)
    .bind(&draft.country)
    .bind(&draft.name)
    .bind(json_string(&draft.tags)?)
    .bind(&draft.url)
    .bind(json_string(&draft.url_tokens)?)
    .bind(&draft.payout_model)
    .bind(draft.payout_value)
    .bind(&draft.currency)
    .bind(&draft.language)
    .bind(&draft.vertical)
    .bind(i64::from(draft.weight))
    .bind(&draft.notes)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    ensure_changed(result.rows_affected(), "Offer not found")?;
    get_offer(pool, id).await
}

pub async fn get_offer(pool: &SqlitePool, id: &str) -> ServerResult<Offer> {
    let row = sqlx::query("SELECT * FROM offers WHERE id = ? AND archived = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServerError::not_found("Offer not found"))?;
    row_to_offer(row)
}

pub async fn archive_offer(pool: &SqlitePool, id: &str) -> ServerResult<()> {
    archive_entity(pool, "offers", id, "Offer not found").await
}

pub async fn create_landing_page(
    pool: &SqlitePool,
    draft: LandingPageDraft,
) -> ServerResult<LandingPage> {
    ensure_valid(&draft)?;
    let id = new_id();
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO landing_pages
         (id, country, name, tags_json, url, url_tokens_json, cta_count, language, vertical,
          weight, notes, archived, created_at_millis, updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&draft.country)
    .bind(&draft.name)
    .bind(json_string(&draft.tags)?)
    .bind(&draft.url)
    .bind(json_string(&draft.url_tokens)?)
    .bind(i64::from(draft.cta_count))
    .bind(&draft.language)
    .bind(&draft.vertical)
    .bind(i64::from(draft.weight))
    .bind(&draft.notes)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    get_landing_page(pool, &id).await
}

pub async fn update_landing_page(
    pool: &SqlitePool,
    id: &str,
    draft: LandingPageDraft,
) -> ServerResult<LandingPage> {
    ensure_valid(&draft)?;
    let now = now_millis()?;
    let result = sqlx::query(
        "UPDATE landing_pages SET
            country = ?, name = ?, tags_json = ?, url = ?, url_tokens_json = ?,
            cta_count = ?, language = ?, vertical = ?, weight = ?, notes = ?,
            updated_at_millis = ?
         WHERE id = ? AND archived = 0",
    )
    .bind(&draft.country)
    .bind(&draft.name)
    .bind(json_string(&draft.tags)?)
    .bind(&draft.url)
    .bind(json_string(&draft.url_tokens)?)
    .bind(i64::from(draft.cta_count))
    .bind(&draft.language)
    .bind(&draft.vertical)
    .bind(i64::from(draft.weight))
    .bind(&draft.notes)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    ensure_changed(result.rows_affected(), "Landing page not found")?;
    get_landing_page(pool, id).await
}

pub async fn get_landing_page(pool: &SqlitePool, id: &str) -> ServerResult<LandingPage> {
    let row = sqlx::query("SELECT * FROM landing_pages WHERE id = ? AND archived = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServerError::not_found("Landing page not found"))?;
    row_to_landing_page(row)
}

pub async fn archive_landing_page(pool: &SqlitePool, id: &str) -> ServerResult<()> {
    archive_entity(pool, "landing_pages", id, "Landing page not found").await
}

pub async fn create_traffic_source(
    pool: &SqlitePool,
    draft: TrafficSourceDraft,
) -> ServerResult<TrafficSource> {
    ensure_valid(&draft)?;
    let id = new_id();
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO traffic_sources
         (id, name, external_id_parameter, cost_parameter, custom_parameters_json, currency,
          postback_urls_json, pixel_url, track_impressions, direct_tracking, notes, archived,
          created_at_millis, updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&draft.name)
    .bind(&draft.external_id_parameter)
    .bind(&draft.cost_parameter)
    .bind(json_string(&draft.custom_parameters)?)
    .bind(&draft.currency)
    .bind(json_string(&draft.postback_urls)?)
    .bind(&draft.pixel_url)
    .bind(draft.track_impressions)
    .bind(draft.direct_tracking)
    .bind(&draft.notes)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    get_traffic_source(pool, &id).await
}

pub async fn update_traffic_source(
    pool: &SqlitePool,
    id: &str,
    draft: TrafficSourceDraft,
) -> ServerResult<TrafficSource> {
    ensure_valid(&draft)?;
    let now = now_millis()?;
    let result = sqlx::query(
        "UPDATE traffic_sources SET
            name = ?, external_id_parameter = ?, cost_parameter = ?, custom_parameters_json = ?,
            currency = ?, postback_urls_json = ?, pixel_url = ?, track_impressions = ?,
            direct_tracking = ?, notes = ?, updated_at_millis = ?
         WHERE id = ? AND archived = 0",
    )
    .bind(&draft.name)
    .bind(&draft.external_id_parameter)
    .bind(&draft.cost_parameter)
    .bind(json_string(&draft.custom_parameters)?)
    .bind(&draft.currency)
    .bind(json_string(&draft.postback_urls)?)
    .bind(&draft.pixel_url)
    .bind(draft.track_impressions)
    .bind(draft.direct_tracking)
    .bind(&draft.notes)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    ensure_changed(result.rows_affected(), "Traffic source not found")?;
    refresh_campaign_urls_for_traffic_source(pool, id).await?;
    get_traffic_source(pool, id).await
}

pub async fn get_traffic_source(pool: &SqlitePool, id: &str) -> ServerResult<TrafficSource> {
    let row = sqlx::query("SELECT * FROM traffic_sources WHERE id = ? AND archived = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServerError::not_found("Traffic source not found"))?;
    row_to_traffic_source(row)
}

pub async fn archive_traffic_source(pool: &SqlitePool, id: &str) -> ServerResult<()> {
    archive_entity(pool, "traffic_sources", id, "Traffic source not found").await
}

pub async fn create_funnel(pool: &SqlitePool, draft: FunnelDraft) -> ServerResult<Funnel> {
    ensure_valid(&draft)?;
    ensure_funnel_references(pool, &draft).await?;
    let id = new_id();
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO funnels
         (id, country, name, redirect_handling, referrer_handling, conditional_sequences_json,
          default_sequences_json, notes, archived, created_at_millis, updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&draft.country)
    .bind(&draft.name)
    .bind(&draft.redirect_handling)
    .bind(&draft.referrer_handling)
    .bind(json_string(&draft.conditional_sequences)?)
    .bind(json_string(&draft.default_sequences)?)
    .bind(&draft.notes)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    get_funnel(pool, &id).await
}

pub async fn update_funnel(
    pool: &SqlitePool,
    id: &str,
    draft: FunnelDraft,
) -> ServerResult<Funnel> {
    ensure_valid(&draft)?;
    ensure_funnel_references(pool, &draft).await?;
    let now = now_millis()?;
    let result = sqlx::query(
        "UPDATE funnels SET
            country = ?, name = ?, redirect_handling = ?, referrer_handling = ?,
            conditional_sequences_json = ?, default_sequences_json = ?, notes = ?,
            updated_at_millis = ?
         WHERE id = ? AND archived = 0",
    )
    .bind(&draft.country)
    .bind(&draft.name)
    .bind(&draft.redirect_handling)
    .bind(&draft.referrer_handling)
    .bind(json_string(&draft.conditional_sequences)?)
    .bind(json_string(&draft.default_sequences)?)
    .bind(&draft.notes)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    ensure_changed(result.rows_affected(), "Funnel not found")?;
    get_funnel(pool, id).await
}

pub async fn get_funnel(pool: &SqlitePool, id: &str) -> ServerResult<Funnel> {
    let row = sqlx::query("SELECT * FROM funnels WHERE id = ? AND archived = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServerError::not_found("Funnel not found"))?;
    row_to_funnel(row)
}

pub async fn archive_funnel(pool: &SqlitePool, id: &str) -> ServerResult<()> {
    archive_entity(pool, "funnels", id, "Funnel not found").await
}

pub async fn create_campaign(
    pool: &SqlitePool,
    public_base_url: &str,
    draft: CampaignDraft,
) -> ServerResult<Campaign> {
    ensure_valid(&draft)?;
    ensure_campaign_references(pool, &draft).await?;
    let traffic_source = get_traffic_source(pool, &draft.traffic_source_id).await?;
    let id = new_id();
    let now = now_millis()?;
    let tracking_url = tracking_url(public_base_url, &id);
    let traffic_source_query_template = traffic_query_template(&traffic_source.draft);
    sqlx::query(
        "INSERT INTO campaigns
         (id, traffic_source_id, destination_type, funnel_id, direct_sequence_json, cost_model,
          cost_value, country, name, notes, tracking_url, traffic_source_query_template,
          last_clicked_at_millis, archived, created_at_millis, updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&draft.traffic_source_id)
    .bind(destination_type_to_str(&draft.destination_type))
    .bind(&draft.funnel_id)
    .bind(json_option_string(&draft.direct_sequence)?)
    .bind(&draft.cost_model)
    .bind(draft.cost_value)
    .bind(&draft.country)
    .bind(&draft.name)
    .bind(&draft.notes)
    .bind(&tracking_url)
    .bind(&traffic_source_query_template)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    get_campaign(pool, &id).await
}

pub async fn update_campaign(
    pool: &SqlitePool,
    public_base_url: &str,
    id: &str,
    draft: CampaignDraft,
) -> ServerResult<Campaign> {
    ensure_valid(&draft)?;
    ensure_campaign_references(pool, &draft).await?;
    let traffic_source = get_traffic_source(pool, &draft.traffic_source_id).await?;
    let now = now_millis()?;
    let tracking_url = tracking_url(public_base_url, id);
    let traffic_source_query_template = traffic_query_template(&traffic_source.draft);
    let result = sqlx::query(
        "UPDATE campaigns SET
            traffic_source_id = ?, destination_type = ?, funnel_id = ?, direct_sequence_json = ?,
            cost_model = ?, cost_value = ?, country = ?, name = ?, notes = ?, tracking_url = ?,
            traffic_source_query_template = ?, updated_at_millis = ?
         WHERE id = ? AND archived = 0",
    )
    .bind(&draft.traffic_source_id)
    .bind(destination_type_to_str(&draft.destination_type))
    .bind(&draft.funnel_id)
    .bind(json_option_string(&draft.direct_sequence)?)
    .bind(&draft.cost_model)
    .bind(draft.cost_value)
    .bind(&draft.country)
    .bind(&draft.name)
    .bind(&draft.notes)
    .bind(&tracking_url)
    .bind(&traffic_source_query_template)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    ensure_changed(result.rows_affected(), "Campaign not found")?;
    get_campaign(pool, id).await
}

pub async fn get_campaign(pool: &SqlitePool, id: &str) -> ServerResult<Campaign> {
    let row = sqlx::query("SELECT * FROM campaigns WHERE id = ? AND archived = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServerError::not_found("Campaign not found"))?;
    row_to_campaign(row)
}

pub async fn archive_campaign(pool: &SqlitePool, id: &str) -> ServerResult<()> {
    archive_entity(pool, "campaigns", id, "Campaign not found").await
}

pub fn campaign_link_info(campaign: &Campaign) -> CampaignLinkInfo {
    CampaignLinkInfo {
        campaign_id: campaign.id.clone(),
        campaign_name: campaign.name.clone(),
        tracking_url: campaign.tracking_url.clone(),
        traffic_source_query_template: campaign.traffic_source_query_template.clone(),
    }
}

pub async fn mark_campaign_clicked(
    pool: &SqlitePool,
    campaign_id: &str,
    clicked_at: i64,
) -> ServerResult<()> {
    sqlx::query("UPDATE campaigns SET last_clicked_at_millis = ? WHERE id = ?")
        .bind(clicked_at)
        .bind(campaign_id)
        .execute(pool)
        .await?;
    Ok(())
}

fn bind_visit_date_filter<'q>(
    query: Query<'q, Sqlite, SqliteArguments<'q>>,
    date_filter: VisitDateFilter,
) -> Query<'q, Sqlite, SqliteArguments<'q>> {
    query
        .bind(date_filter.start_at_millis)
        .bind(date_filter.start_at_millis)
        .bind(date_filter.end_at_millis)
        .bind(date_filter.end_at_millis)
}

fn counted_entity_row(row: SqliteRow) -> ServerResult<EntityRow> {
    Ok(EntityRow {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
        detail: row.try_get("detail")?,
        visits: row.try_get("visits")?,
        unique_visits: row.try_get("unique_visits")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
        tracking_url: None,
    })
}

fn row_to_offer_source(row: SqliteRow) -> ServerResult<OfferSource> {
    Ok(OfferSource {
        id: row.try_get("id")?,
        created_at_millis: row.try_get("created_at_millis")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
        archived: row.try_get("archived")?,
        draft: OfferSourceDraft {
            name: row.try_get("name")?,
            tokens: json_value(row.try_get::<String, _>("tokens_json")?)?,
            tracking_domain: row.try_get("tracking_domain")?,
            tracking_method: row.try_get("tracking_method")?,
            payout_currency: row.try_get("payout_currency")?,
            postback_url: row.try_get("postback_url")?,
            append_click_id: row.try_get("append_click_id")?,
            accept_duplicate_postbacks: row.try_get("accept_duplicate_postbacks")?,
            whitelist_postback_ips: json_value(
                row.try_get::<String, _>("whitelist_postback_ips_json")?,
            )?,
            referrer_handling: row.try_get("referrer_handling")?,
            notes: row.try_get("notes")?,
        },
    })
}

fn row_to_offer(row: SqliteRow) -> ServerResult<Offer> {
    Ok(Offer {
        id: row.try_get("id")?,
        created_at_millis: row.try_get("created_at_millis")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
        archived: row.try_get("archived")?,
        draft: OfferDraft {
            offer_source_id: row.try_get("offer_source_id")?,
            country: row.try_get("country")?,
            name: row.try_get("name")?,
            tags: json_value(row.try_get::<String, _>("tags_json")?)?,
            url: row.try_get("url")?,
            url_tokens: json_value(row.try_get::<String, _>("url_tokens_json")?)?,
            payout_model: row.try_get("payout_model")?,
            payout_value: row.try_get("payout_value")?,
            currency: row.try_get("currency")?,
            language: row.try_get("language")?,
            vertical: row.try_get("vertical")?,
            weight: integer_to_u32(row.try_get("weight")?, "weight")?,
            notes: row.try_get("notes")?,
        },
    })
}

fn row_to_landing_page(row: SqliteRow) -> ServerResult<LandingPage> {
    Ok(LandingPage {
        id: row.try_get("id")?,
        created_at_millis: row.try_get("created_at_millis")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
        archived: row.try_get("archived")?,
        draft: LandingPageDraft {
            country: row.try_get("country")?,
            name: row.try_get("name")?,
            tags: json_value(row.try_get::<String, _>("tags_json")?)?,
            url: row.try_get("url")?,
            url_tokens: json_value(row.try_get::<String, _>("url_tokens_json")?)?,
            cta_count: integer_to_u8(row.try_get("cta_count")?, "cta_count")?,
            language: row.try_get("language")?,
            vertical: row.try_get("vertical")?,
            weight: integer_to_u32(row.try_get("weight")?, "weight")?,
            notes: row.try_get("notes")?,
        },
    })
}

fn row_to_traffic_source(row: SqliteRow) -> ServerResult<TrafficSource> {
    Ok(TrafficSource {
        id: row.try_get("id")?,
        created_at_millis: row.try_get("created_at_millis")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
        archived: row.try_get("archived")?,
        draft: TrafficSourceDraft {
            name: row.try_get("name")?,
            external_id_parameter: row.try_get("external_id_parameter")?,
            cost_parameter: row.try_get("cost_parameter")?,
            custom_parameters: json_value(row.try_get::<String, _>("custom_parameters_json")?)?,
            currency: row.try_get("currency")?,
            postback_urls: json_value(row.try_get::<String, _>("postback_urls_json")?)?,
            pixel_url: row.try_get("pixel_url")?,
            track_impressions: row.try_get("track_impressions")?,
            direct_tracking: row.try_get("direct_tracking")?,
            notes: row.try_get("notes")?,
        },
    })
}

fn row_to_funnel(row: SqliteRow) -> ServerResult<Funnel> {
    Ok(Funnel {
        id: row.try_get("id")?,
        created_at_millis: row.try_get("created_at_millis")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
        archived: row.try_get("archived")?,
        draft: FunnelDraft {
            country: row.try_get("country")?,
            name: row.try_get("name")?,
            redirect_handling: row.try_get("redirect_handling")?,
            referrer_handling: row.try_get("referrer_handling")?,
            conditional_sequences: json_value(
                row.try_get::<String, _>("conditional_sequences_json")?,
            )?,
            default_sequences: json_value(row.try_get::<String, _>("default_sequences_json")?)?,
            notes: row.try_get("notes")?,
        },
    })
}

fn row_to_campaign(row: SqliteRow) -> ServerResult<Campaign> {
    let destination_type =
        destination_type_from_str(row.try_get::<String, _>("destination_type")?)?;
    let direct_sequence = row
        .try_get::<Option<String>, _>("direct_sequence_json")?
        .map(json_value)
        .transpose()?;
    Ok(Campaign {
        id: row.try_get("id")?,
        created_at_millis: row.try_get("created_at_millis")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
        archived: row.try_get("archived")?,
        tracking_url: row.try_get("tracking_url")?,
        traffic_source_query_template: row.try_get("traffic_source_query_template")?,
        last_clicked_at_millis: row.try_get("last_clicked_at_millis")?,
        draft: CampaignDraft {
            traffic_source_id: row.try_get("traffic_source_id")?,
            destination_type,
            funnel_id: row.try_get("funnel_id")?,
            direct_sequence,
            cost_model: row.try_get("cost_model")?,
            cost_value: row.try_get("cost_value")?,
            country: row.try_get("country")?,
            name: row.try_get("name")?,
            notes: row.try_get("notes")?,
        },
    })
}

async fn ensure_offer_source_exists(pool: &SqlitePool, id: &str) -> ServerResult<()> {
    ensure_exists(pool, "offer_sources", id, "Offer source not found").await
}

async fn ensure_campaign_references(pool: &SqlitePool, draft: &CampaignDraft) -> ServerResult<()> {
    ensure_exists(
        pool,
        "traffic_sources",
        &draft.traffic_source_id,
        "Traffic source not found",
    )
    .await?;
    if let Some(funnel_id) = draft.funnel_id.as_deref()
        && !funnel_id.trim().is_empty()
    {
        ensure_exists(pool, "funnels", funnel_id, "Funnel not found").await?;
    }
    Ok(())
}

async fn ensure_funnel_references(pool: &SqlitePool, draft: &FunnelDraft) -> ServerResult<()> {
    let mut landing_page_ids = Vec::new();
    let mut offer_ids = Vec::new();
    for sequence in draft
        .default_sequences
        .iter()
        .chain(draft.conditional_sequences.iter())
    {
        collect_sequence_references(sequence, &mut landing_page_ids, &mut offer_ids);
    }
    for landing_page_id in landing_page_ids {
        ensure_exists(
            pool,
            "landing_pages",
            &landing_page_id,
            "Landing page not found",
        )
        .await?;
    }
    for offer_id in offer_ids {
        ensure_exists(pool, "offers", &offer_id, "Offer not found").await?;
    }
    Ok(())
}

fn collect_sequence_references(
    sequence: &FunnelSequence,
    landing_page_ids: &mut Vec<String>,
    offer_ids: &mut Vec<String>,
) {
    for path in &sequence.paths {
        collect_path_references(path, landing_page_ids, offer_ids);
    }
}

fn collect_path_references(
    path: &ad_buy_engine_domain::FunnelPath,
    landing_page_ids: &mut Vec<String>,
    offer_ids: &mut Vec<String>,
) {
    if let Some(landing_page_id) = path.landing_page_id.as_deref()
        && !landing_page_id.trim().is_empty()
    {
        landing_page_ids.push(landing_page_id.to_string());
    }
    offer_ids.extend(path.offers.iter().map(|offer| offer.id.clone()));
    for child in &path.children {
        collect_path_references(child, landing_page_ids, offer_ids);
    }
}

async fn ensure_exists(
    pool: &SqlitePool,
    table: &str,
    id: &str,
    message: &str,
) -> ServerResult<()> {
    let sql = match table {
        "offer_sources" => "SELECT id FROM offer_sources WHERE id = ? AND archived = 0",
        "offers" => "SELECT id FROM offers WHERE id = ? AND archived = 0",
        "landing_pages" => "SELECT id FROM landing_pages WHERE id = ? AND archived = 0",
        "traffic_sources" => "SELECT id FROM traffic_sources WHERE id = ? AND archived = 0",
        "funnels" => "SELECT id FROM funnels WHERE id = ? AND archived = 0",
        _ => return Err(ServerError::internal("invalid repository table")),
    };
    let found: Option<String> = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    found
        .map(|_| ())
        .ok_or_else(|| ServerError::not_found(message))
}

async fn archive_entity(
    pool: &SqlitePool,
    table: &str,
    id: &str,
    message: &str,
) -> ServerResult<()> {
    let sql = match table {
        "offer_sources" => {
            "UPDATE offer_sources SET archived = 1, updated_at_millis = ? WHERE id = ? AND archived = 0"
        }
        "offers" => {
            "UPDATE offers SET archived = 1, updated_at_millis = ? WHERE id = ? AND archived = 0"
        }
        "landing_pages" => {
            "UPDATE landing_pages SET archived = 1, updated_at_millis = ? WHERE id = ? AND archived = 0"
        }
        "traffic_sources" => {
            "UPDATE traffic_sources SET archived = 1, updated_at_millis = ? WHERE id = ? AND archived = 0"
        }
        "funnels" => {
            "UPDATE funnels SET archived = 1, updated_at_millis = ? WHERE id = ? AND archived = 0"
        }
        "campaigns" => {
            "UPDATE campaigns SET archived = 1, updated_at_millis = ? WHERE id = ? AND archived = 0"
        }
        _ => return Err(ServerError::internal("invalid repository table")),
    };
    let result = sqlx::query(sql)
        .bind(now_millis()?)
        .bind(id)
        .execute(pool)
        .await?;
    ensure_changed(result.rows_affected(), message)
}

async fn refresh_campaign_urls_for_traffic_source(
    pool: &SqlitePool,
    traffic_source_id: &str,
) -> ServerResult<()> {
    let traffic_source = get_traffic_source(pool, traffic_source_id).await?;
    let template = traffic_query_template(&traffic_source.draft);
    sqlx::query(
        "UPDATE campaigns SET traffic_source_query_template = ?, updated_at_millis = ?
         WHERE traffic_source_id = ? AND archived = 0",
    )
    .bind(template)
    .bind(now_millis()?)
    .bind(traffic_source_id)
    .execute(pool)
    .await?;
    Ok(())
}

fn ensure_valid<T: ValidateDraft>(draft: &T) -> ServerResult<()> {
    let errors = draft.validate();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(ServerError::validation("Validation failed", errors))
    }
}

fn ensure_changed(rows_affected: u64, message: &str) -> ServerResult<()> {
    if rows_affected == 0 {
        Err(ServerError::not_found(message))
    } else {
        Ok(())
    }
}

fn destination_type_to_str(destination_type: &DestinationType) -> &'static str {
    match destination_type {
        DestinationType::Funnel => "funnel",
        DestinationType::DirectSequence => "direct_sequence",
    }
}

fn destination_type_from_str(value: String) -> ServerResult<DestinationType> {
    match value.as_str() {
        "funnel" => Ok(DestinationType::Funnel),
        "direct_sequence" => Ok(DestinationType::DirectSequence),
        _ => Err(ServerError::internal(format!(
            "invalid destination type stored: {value}"
        ))),
    }
}

fn tracking_url(public_base_url: &str, campaign_id: &str) -> String {
    format!(
        "{}/c/{}",
        public_base_url.trim_end_matches('/'),
        campaign_id
    )
}

fn traffic_query_template(draft: &TrafficSourceDraft) -> String {
    let mut params = Vec::new();
    if !draft.external_id_parameter.trim().is_empty() {
        params.push(format!(
            "{}={{external_id}}",
            urlencoding::encode(&draft.external_id_parameter)
        ));
    }
    if !draft.cost_parameter.trim().is_empty() {
        params.push(format!(
            "{}={{cost}}",
            urlencoding::encode(&draft.cost_parameter)
        ));
    }
    for token in &draft.custom_parameters {
        if !token.name.trim().is_empty() {
            params.push(format!(
                "{}={}",
                urlencoding::encode(&token.name),
                urlencoding::encode(&token.token)
            ));
        }
    }
    if params.is_empty() {
        String::new()
    } else {
        format!("?{}", params.join("&"))
    }
}

fn json_string<T: Serialize>(value: &T) -> ServerResult<String> {
    serde_json::to_string(value).map_err(ServerError::from)
}

fn json_option_string<T: Serialize>(value: &Option<T>) -> ServerResult<Option<String>> {
    value.as_ref().map(json_string).transpose()
}

fn json_value<T: DeserializeOwned>(value: String) -> ServerResult<T> {
    serde_json::from_str(&value).map_err(ServerError::from)
}

fn integer_to_u32(value: i64, field: &'static str) -> ServerResult<u32> {
    u32::try_from(value).map_err(|error| {
        ServerError::internal(format!("invalid {field} value in database: {error}"))
    })
}

fn integer_to_u8(value: i64, field: &'static str) -> ServerResult<u8> {
    u8::try_from(value).map_err(|error| {
        ServerError::internal(format!("invalid {field} value in database: {error}"))
    })
}

fn new_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn required_field_error(field: impl Into<String>, message: impl Into<String>) -> FieldError {
    FieldError {
        field: field.into(),
        message: message.into(),
    }
}
