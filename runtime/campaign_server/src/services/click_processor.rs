use ad_buy_engine_domain::{
    Campaign, ClickContext, ClickMapEntry, DestinationType, FunnelPath, FunnelSequence,
    LandingPage, Offer, TokenValue, VisitEnrichment, VisitEventType,
};
use axum::http::HeaderMap;
use chrono::{Datelike, Timelike, Utc};
use sqlx::SqlitePool;
use url::form_urlencoded;

use crate::error::{ServerError, ServerResult};
use crate::services::conditions::evaluate_all;
use crate::services::geoip::SharedGeoIpService;
use crate::services::user_agent::user_agent_enrichment;
use crate::storage::entities::{get_campaign, get_funnel, get_landing_page, get_offer};
use crate::storage::visits::{
    NewVisit, get_visit, insert_event, insert_visit_with_event, is_unique_visit, new_visit_id,
};
use crate::time::now_millis;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedirectOutcome {
    pub target: String,
}

#[derive(Clone, Debug)]
struct SelectedRoute {
    sequence: FunnelSequence,
    landing_page_id: Option<String>,
    offer_id: String,
}

pub async fn process_campaign_click(
    pool: &SqlitePool,
    campaign_id: &str,
    headers: &HeaderMap,
    raw_query: Option<&str>,
    geoip: &SharedGeoIpService,
) -> ServerResult<RedirectOutcome> {
    let campaign = get_campaign(pool, campaign_id).await?;
    let tracking_base_url = campaign_tracking_base_url(&campaign);
    let query_params = parse_query(raw_query.unwrap_or_default());
    let (mut context, enrichment) =
        click_context(pool, &campaign, headers, &query_params, geoip).await?;
    let (selected_route, missing_fields) = select_route(pool, &campaign, &context).await?;

    let visit_id = new_visit_id();
    let offer = get_offer(pool, &selected_route.offer_id).await?;
    let (redirect_target, click_map) =
        if let Some(landing_page_id) = selected_route.landing_page_id.as_deref() {
            let landing_page = get_landing_page(pool, landing_page_id).await?;
            let click_map = lander_click_map(
                &tracking_base_url,
                &visit_id,
                &campaign,
                &offer,
                &landing_page,
                &query_params,
            );
            (
                substitute_url(
                    &landing_page.url,
                    UrlSubstitution {
                        tracking_base_url: &tracking_base_url,
                        visit_id: &visit_id,
                        campaign: &campaign,
                        landing_page: Some(&landing_page),
                        offer: Some(&offer),
                        query_params: &query_params,
                        click_map: &click_map,
                    },
                ),
                click_map,
            )
        } else {
            (
                substitute_url(
                    &offer.url,
                    UrlSubstitution {
                        tracking_base_url: &tracking_base_url,
                        visit_id: &visit_id,
                        campaign: &campaign,
                        landing_page: None,
                        offer: Some(&offer),
                        query_params: &query_params,
                        click_map: &[],
                    },
                ),
                Vec::new(),
            )
        };

    let mut transaction = pool.begin().await?;
    let selected_funnel_id = campaign.funnel_id.as_deref();
    insert_visit_with_event(
        &mut transaction,
        NewVisit {
            id: &visit_id,
            campaign_id: &campaign.id,
            traffic_source_id: &campaign.traffic_source_id,
            selected_funnel_id,
            selected_sequence: Some(&selected_route.sequence),
            selected_landing_page_id: selected_route.landing_page_id.as_deref(),
            selected_offer_id: Some(&selected_route.offer_id),
            referrer: context.referrer.as_deref(),
            ip_address: context.ip_address.as_deref(),
            user_agent: context.user_agent.as_deref(),
            enrichment: &enrichment,
            query_params: &query_params,
            click_map: &click_map,
            redirect_target: &redirect_target,
            suspicious: false,
        },
    )
    .await?;
    sqlx::query("UPDATE campaigns SET last_clicked_at_millis = ? WHERE id = ?")
        .bind(now_millis()?)
        .bind(&campaign.id)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;

    for missing_field in missing_fields {
        insert_event(
            pool,
            Some(&visit_id),
            Some(&campaign.id),
            VisitEventType::ConditionDataMissing,
            serde_json::json!({ "field": missing_field }),
        )
        .await?;
    }

    context.is_unique_visit = Some(false);
    Ok(RedirectOutcome {
        target: redirect_target,
    })
}

pub async fn process_lander_click(
    pool: &SqlitePool,
    visit_id: &str,
    slot: u8,
) -> ServerResult<RedirectOutcome> {
    let visit = get_visit(pool, visit_id).await?;
    let entry = visit
        .click_map
        .iter()
        .find(|entry| entry.slot == slot)
        .ok_or_else(|| ServerError::not_found("Click slot not found"))?;
    insert_event(
        pool,
        Some(visit_id),
        Some(&visit.campaign_id),
        VisitEventType::LanderClick,
        serde_json::json!({ "slot": slot, "offer_id": entry.offer_id }),
    )
    .await?;
    insert_event(
        pool,
        Some(visit_id),
        Some(&visit.campaign_id),
        VisitEventType::OfferClick,
        serde_json::json!({ "slot": slot, "offer_id": entry.offer_id }),
    )
    .await?;
    Ok(RedirectOutcome {
        target: entry.target_url.clone(),
    })
}

async fn click_context(
    pool: &SqlitePool,
    campaign: &Campaign,
    headers: &HeaderMap,
    query_params: &[(String, String)],
    geoip: &SharedGeoIpService,
) -> ServerResult<(ClickContext, VisitEnrichment)> {
    let user_agent = header_value(headers, "user-agent");
    let ip_address = header_value(headers, "x-forwarded-for")
        .and_then(|value| {
            value
                .split(',')
                .next()
                .map(str::trim)
                .map(ToOwned::to_owned)
        })
        .or_else(|| header_value(headers, "x-real-ip"));
    let referrer = header_value(headers, "referer").or_else(|| header_value(headers, "referrer"));
    let referrer_domain = referrer.as_deref().and_then(referrer_domain);
    let now = Utc::now();
    let is_unique = is_unique_visit(
        pool,
        &campaign.id,
        ip_address.as_deref(),
        user_agent.as_deref(),
    )
    .await?;
    let query = query_params
        .iter()
        .map(|(key, value)| TokenValue {
            key: key.clone(),
            value: value.clone(),
        })
        .collect();
    let language = header_value(headers, "accept-language").and_then(|value| {
        value
            .split(',')
            .next()
            .map(str::trim)
            .map(ToOwned::to_owned)
    });

    let mut enrichment = geoip_enrichment(geoip, ip_address.as_deref());
    let user_agent_enrichment = user_agent_enrichment(user_agent.as_deref());
    enrichment.browser = user_agent_enrichment.browser;
    enrichment.browser_version = user_agent_enrichment.browser_version;
    enrichment.operating_system = user_agent_enrichment.operating_system;
    enrichment.operating_system_version = user_agent_enrichment.operating_system_version;
    enrichment.device_type = user_agent_enrichment.device_type;
    enrichment.device_brand = user_agent_enrichment.device_brand;
    enrichment.device_model = user_agent_enrichment.device_model;

    let context = ClickContext {
        ip_address,
        user_agent: user_agent.clone(),
        referrer,
        referrer_domain,
        query,
        country: enrichment.country.clone(),
        region: enrichment.region.clone(),
        city: enrichment.city.clone(),
        isp: enrichment.isp.clone(),
        connection_type: enrichment.connection_type.clone(),
        proxy_type: enrichment.proxy_type.clone(),
        carrier: enrichment.carrier.clone(),
        browser: enrichment.browser.clone(),
        operating_system: enrichment.operating_system.clone(),
        device_type: enrichment.device_type.clone(),
        device_brand: enrichment.device_brand.clone(),
        language,
        weekday: Some(now.weekday().to_string()),
        minute_of_day: u16::try_from(now.hour() * 60 + now.minute()).ok(),
        is_unique_visit: Some(is_unique),
    };
    Ok((context, enrichment))
}

fn geoip_enrichment(geoip: &SharedGeoIpService, ip_address: Option<&str>) -> VisitEnrichment {
    match geoip.read() {
        Ok(service) => service.lookup(ip_address),
        Err(_) => VisitEnrichment::default(),
    }
}

async fn select_route(
    pool: &SqlitePool,
    campaign: &Campaign,
    context: &ClickContext,
) -> ServerResult<(SelectedRoute, Vec<String>)> {
    let (conditional_sequences, default_sequences) = match campaign.destination_type {
        DestinationType::Funnel => {
            let Some(funnel_id) = campaign.funnel_id.as_deref() else {
                return Err(ServerError::not_found("Campaign funnel is missing"));
            };
            let funnel = get_funnel(pool, funnel_id).await?;
            (
                funnel.conditional_sequences.clone(),
                funnel.default_sequences.clone(),
            )
        }
        DestinationType::DirectSequence => {
            let Some(sequence) = campaign.direct_sequence.clone() else {
                return Err(ServerError::not_found(
                    "Campaign direct sequence is missing",
                ));
            };
            (Vec::new(), vec![sequence])
        }
    };

    let mut missing_fields = Vec::new();
    let mut matched_conditionals = Vec::new();
    for sequence in conditional_sequences
        .into_iter()
        .filter(|sequence| sequence.active)
    {
        let evaluation = evaluate_all(&sequence.conditions, context);
        missing_fields.extend(evaluation.missing_fields);
        if evaluation.matched {
            matched_conditionals.push(sequence);
        }
    }

    let seed = now_millis()?;
    let sequence = weighted_sequence(&matched_conditionals, seed)
        .or_else(|| weighted_sequence(&default_sequences, seed))
        .ok_or_else(|| ServerError::not_found("No active campaign sequence is available"))?;
    let path = weighted_path(&sequence.paths, seed)
        .ok_or_else(|| ServerError::not_found("No active campaign path is available"))?;
    let offer_id = weighted_offer_id(path, seed)
        .ok_or_else(|| ServerError::not_found("No offer is available for the selected path"))?;
    let landing_page_id = path.landing_page_id.clone();

    Ok((
        SelectedRoute {
            sequence,
            landing_page_id,
            offer_id,
        },
        missing_fields,
    ))
}

fn weighted_sequence(sequences: &[FunnelSequence], seed: i64) -> Option<FunnelSequence> {
    let active: Vec<&FunnelSequence> = sequences
        .iter()
        .filter(|sequence| sequence.active && sequence.weight > 0)
        .collect();
    let index = weighted_index(active.iter().map(|sequence| sequence.weight), seed)?;
    active.get(index).map(|sequence| (*sequence).clone())
}

fn weighted_path(paths: &[FunnelPath], seed: i64) -> Option<&FunnelPath> {
    let active: Vec<&FunnelPath> = paths.iter().filter(|path| path.weight > 0).collect();
    let index = weighted_index(active.iter().map(|path| path.weight), seed)?;
    let selected = *active.get(index)?;
    if selected.children.is_empty() {
        Some(selected)
    } else {
        weighted_path(&selected.children, seed + 1).or(Some(selected))
    }
}

fn weighted_offer_id(path: &FunnelPath, seed: i64) -> Option<String> {
    if path.offers.is_empty() {
        return None;
    }
    let index = weighted_index(path.offers.iter().map(|offer| offer.weight), seed)?;
    path.offers.get(index).map(|offer| offer.id.clone())
}

fn weighted_index(weights: impl Iterator<Item = u32>, seed: i64) -> Option<usize> {
    let weights: Vec<u32> = weights.collect();
    let total: u64 = weights.iter().map(|weight| u64::from(*weight)).sum();
    if total == 0 {
        return None;
    }
    let seed = u64::try_from(seed).unwrap_or(0);
    let mut cursor = seed % total;
    for (index, weight) in weights.iter().enumerate() {
        let weight = u64::from(*weight);
        if cursor < weight {
            return Some(index);
        }
        cursor -= weight;
    }
    None
}

fn lander_click_map(
    tracking_base_url: &str,
    visit_id: &str,
    campaign: &Campaign,
    offer: &Offer,
    landing_page: &LandingPage,
    query_params: &[(String, String)],
) -> Vec<ClickMapEntry> {
    (1..=landing_page.cta_count)
        .map(|slot| ClickMapEntry {
            slot,
            offer_id: offer.id.clone(),
            target_url: substitute_url(
                &offer.url,
                UrlSubstitution {
                    tracking_base_url,
                    visit_id,
                    campaign,
                    landing_page: Some(landing_page),
                    offer: Some(offer),
                    query_params,
                    click_map: &[],
                },
            ),
        })
        .collect()
}

fn campaign_tracking_base_url(campaign: &Campaign) -> String {
    let campaign_path = format!("/c/{}", campaign.id);
    campaign
        .tracking_url
        .strip_suffix(&campaign_path)
        .unwrap_or_else(|| campaign.tracking_url.trim_end_matches('/'))
        .to_string()
}

struct UrlSubstitution<'a> {
    tracking_base_url: &'a str,
    visit_id: &'a str,
    campaign: &'a Campaign,
    landing_page: Option<&'a LandingPage>,
    offer: Option<&'a Offer>,
    query_params: &'a [(String, String)],
    click_map: &'a [ClickMapEntry],
}

fn substitute_url(url: &str, context: UrlSubstitution<'_>) -> String {
    let mut output = url
        .replace("{clickid}", context.visit_id)
        .replace("{click_id}", context.visit_id)
        .replace("{visit_id}", context.visit_id)
        .replace("{campaign_id}", &context.campaign.id)
        .replace("{campaign_name}", &context.campaign.name)
        .replace("{traffic_source_id}", &context.campaign.traffic_source_id);

    if let Some(landing_page) = context.landing_page {
        output = output
            .replace("{lander_id}", &landing_page.id)
            .replace("{lander_name}", &landing_page.name);
    }
    if let Some(offer) = context.offer {
        output = output
            .replace("{offer_id}", &offer.id)
            .replace("{offer_name}", &offer.name);
    }
    for (key, value) in context.query_params {
        output = output.replace(&format!("{{{key}}}"), value);
    }
    for entry in context.click_map {
        output = output.replace(
            &format!("{{click_url_{}}}", entry.slot),
            &format!(
                "{}/go/{}/{}",
                context.tracking_base_url.trim_end_matches('/'),
                context.visit_id,
                entry.slot
            ),
        );
    }
    output
}

fn parse_query(raw_query: &str) -> Vec<(String, String)> {
    form_urlencoded::parse(raw_query.as_bytes())
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

fn referrer_domain(referrer: &str) -> Option<String> {
    url::Url::parse(referrer)
        .ok()
        .and_then(|url| url.host_str().map(ToOwned::to_owned))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ad_buy_engine_domain::{CampaignDraft, DestinationType};

    #[test]
    fn token_substitution_adds_click_urls() {
        let campaign = Campaign {
            id: "campaign-1".to_string(),
            created_at_millis: 1,
            updated_at_millis: 1,
            archived: false,
            tracking_url: "http://127.0.0.1:8088/c/campaign-1".to_string(),
            traffic_source_query_template: "?sub={external_id}".to_string(),
            last_clicked_at_millis: None,
            draft: CampaignDraft {
                traffic_source_id: "traffic-1".to_string(),
                destination_type: DestinationType::Funnel,
                funnel_id: Some("funnel-1".to_string()),
                direct_sequence: None,
                cost_model: "CPC".to_string(),
                cost_value: 0.0,
                country: "Global".to_string(),
                name: "Campaign".to_string(),
                notes: String::new(),
            },
        };
        let result = substitute_url(
            "https://lander.test/?cid={campaign_id}&go={click_url_1}",
            UrlSubstitution {
                tracking_base_url: "http://127.0.0.1:8088",
                visit_id: "visit-1",
                campaign: &campaign,
                landing_page: None,
                offer: None,
                query_params: &[("src".to_string(), "paid".to_string())],
                click_map: &[ClickMapEntry {
                    slot: 1,
                    offer_id: "offer-1".to_string(),
                    target_url: "https://offer.test".to_string(),
                }],
            },
        );

        assert_eq!(
            result,
            "https://lander.test/?cid=campaign-1&go=http://127.0.0.1:8088/go/visit-1/1"
        );
    }
}
