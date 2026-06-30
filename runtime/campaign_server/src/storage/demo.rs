use ad_buy_engine_domain::{
    FAKE_AFFILIATE_CLICK_ID_TOKEN, FAKE_AFFILIATE_OFFER_SOURCE_ID,
    FAKE_AFFILIATE_OFFER_SOURCE_NAME, UrlToken, fake_affiliate_catalog, fake_affiliate_offer_url,
};
use sqlx::SqlitePool;

use crate::config::{ServerConfig, validate_fake_affiliate_network_base_url};
use crate::error::{ServerError, ServerResult};
use crate::time::now_millis;

pub async fn seed_fake_affiliate_network_catalog(
    pool: &SqlitePool,
    config: &ServerConfig,
) -> ServerResult<()> {
    validate_fake_affiliate_network_base_url(&config.fake_affiliate_network_base_url)
        .map_err(|error| ServerError::internal(error.to_string()))?;

    let now = now_millis()?;
    let mut transaction = pool.begin().await?;
    upsert_offer_source(&mut transaction, config, now).await?;
    for offer in fake_affiliate_catalog() {
        upsert_offer(&mut transaction, config, *offer, now).await?;
    }
    transaction.commit().await?;
    Ok(())
}

async fn upsert_offer_source(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    config: &ServerConfig,
    now: i64,
) -> ServerResult<()> {
    let tokens = json_string(&vec![UrlToken {
        name: "subid".to_string(),
        token: FAKE_AFFILIATE_CLICK_ID_TOKEN.to_string(),
    }])?;
    let postback_url = format!(
        "{}/postback?cid={{click_id}}&type={{event_type}}&payout={{payout}}&currency={{currency}}&status={{status}}&txid={{transaction_id}}",
        config.tracking_base_url.trim_end_matches('/')
    );

    sqlx::query(
        "INSERT INTO offer_sources
         (id, name, tokens_json, tracking_domain, tracking_method, payout_currency,
          postback_url, append_click_id, accept_duplicate_postbacks,
          whitelist_postback_ips_json, referrer_handling, notes, archived,
          created_at_millis, updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, 1, 0, ?, ?, ?, 0, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            tokens_json = excluded.tokens_json,
            tracking_domain = excluded.tracking_domain,
            tracking_method = excluded.tracking_method,
            payout_currency = excluded.payout_currency,
            postback_url = excluded.postback_url,
            append_click_id = excluded.append_click_id,
            accept_duplicate_postbacks = excluded.accept_duplicate_postbacks,
            whitelist_postback_ips_json = excluded.whitelist_postback_ips_json,
            referrer_handling = excluded.referrer_handling,
            notes = excluded.notes,
            archived = 0,
            updated_at_millis = excluded.updated_at_millis",
    )
    .bind(FAKE_AFFILIATE_OFFER_SOURCE_ID)
    .bind(FAKE_AFFILIATE_OFFER_SOURCE_NAME)
    .bind(tokens)
    .bind("127.0.0.1")
    .bind("postback")
    .bind("USD")
    .bind(postback_url)
    .bind(json_string(&Vec::<String>::new())?)
    .bind("do_nothing")
    .bind("Local/demo-only fake affiliate network source. Not seeded unless explicitly enabled.")
    .bind(now)
    .bind(now)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn upsert_offer(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    config: &ServerConfig,
    offer: ad_buy_engine_domain::FakeAffiliateOffer,
    now: i64,
) -> ServerResult<()> {
    let url = fake_affiliate_offer_url(&config.fake_affiliate_network_base_url, offer.id);
    let url_tokens = json_string(&vec![UrlToken {
        name: "subid".to_string(),
        token: FAKE_AFFILIATE_CLICK_ID_TOKEN.to_string(),
    }])?;
    let tags = json_string(&vec![
        "fake-affiliate-network".to_string(),
        offer.event_type().to_ascii_lowercase(),
    ])?;
    let notes = format!(
        "Local/demo-only fake offer. Deterministic {} threshold: {} qualifying clicks.",
        offer.event_type(),
        offer.default_threshold
    );

    sqlx::query(
        "INSERT INTO offers
         (id, offer_source_id, country, name, tags_json, url, url_tokens_json,
          payout_model, payout_value, currency, language, vertical, weight, notes,
          archived, created_at_millis, updated_at_millis)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
            offer_source_id = excluded.offer_source_id,
            country = excluded.country,
            name = excluded.name,
            tags_json = excluded.tags_json,
            url = excluded.url,
            url_tokens_json = excluded.url_tokens_json,
            payout_model = excluded.payout_model,
            payout_value = excluded.payout_value,
            currency = excluded.currency,
            language = excluded.language,
            vertical = excluded.vertical,
            weight = excluded.weight,
            notes = excluded.notes,
            archived = 0,
            updated_at_millis = excluded.updated_at_millis",
    )
    .bind(offer.id)
    .bind(FAKE_AFFILIATE_OFFER_SOURCE_ID)
    .bind("Global")
    .bind(offer.name)
    .bind(tags)
    .bind(url)
    .bind(url_tokens)
    .bind(offer.payout_model())
    .bind(offer.payout_value)
    .bind(offer.currency)
    .bind("en")
    .bind(offer.vertical)
    .bind(100_i64)
    .bind(notes)
    .bind(now)
    .bind(now)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

fn json_string<T: serde::Serialize + ?Sized>(value: &T) -> ServerResult<String> {
    serde_json::to_string(value).map_err(ServerError::from)
}
