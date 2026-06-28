use std::collections::{BTreeMap, HashMap, HashSet};

use ad_buy_engine_domain::{
    DashboardConversionPathStep, DashboardDateWindow, DashboardDecision, DashboardKpi,
    DashboardMetricUnit, DashboardPerformancePoint, DashboardRecentEvent, DashboardSetupHealthItem,
    DashboardSummaryResponse, DashboardTone, DashboardTopMover, DashboardTrafficMix,
    DashboardTrafficSegment, VisitEnrichment, VisitEventType,
};
use chrono::{Datelike, TimeZone, Utc};
use sqlx::{Row, SqlitePool};

use crate::error::ServerResult;
use crate::services::user_agent::{detect_browser, detect_device_type, detect_operating_system};
use crate::storage::date_filter::{
    VisitDateFilter, bind_visit_date_filter, bind_visit_date_filter_scalar,
};
use crate::storage::visit_identity::unique_visit_key;
use crate::storage::visits::visit_event_type_key;
use crate::time::now_millis;

const MAX_PERFORMANCE_POINTS: usize = 90;

#[derive(Clone, Debug, Default)]
struct PeriodMetrics {
    visits: i64,
    unique_visits: i64,
    clicks: i64,
    conversions: i64,
    revenue: f64,
    cost: f64,
    suspicious_visits: i64,
    issue_events: i64,
}

impl PeriodMetrics {
    fn profit(&self) -> f64 {
        self.revenue - self.cost
    }

    fn roi(&self) -> f64 {
        percentage_ratio(self.profit(), self.cost)
    }

    fn epc(&self) -> f64 {
        ratio(self.revenue, self.clicks)
    }

    fn cpa(&self) -> f64 {
        ratio(self.cost, self.conversions)
    }

    fn conversion_rate(&self) -> f64 {
        percentage_ratio(self.conversions as f64, self.visits as f64)
    }
}

#[derive(Clone, Debug, Default)]
struct CampaignMetrics {
    id: String,
    name: String,
    detail: String,
    cost_model: String,
    cost_value: f64,
    visits: i64,
    conversions: i64,
    cost_conversions: i64,
    revenue: f64,
}

impl CampaignMetrics {
    fn cost(&self) -> f64 {
        estimated_cost(
            &self.cost_model,
            self.cost_value,
            self.visits,
            self.cost_conversions,
            self.revenue,
        )
    }

    fn profit(&self) -> f64 {
        self.revenue - self.cost()
    }

    fn roi(&self) -> f64 {
        percentage_ratio(self.profit(), self.cost())
    }
}

#[derive(Clone, Debug)]
struct VisitFact {
    id: String,
    campaign_id: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    enrichment: VisitEnrichment,
    suspicious: bool,
    created_at_millis: i64,
}

#[derive(Clone, Debug)]
struct ConversionFact {
    campaign_id: String,
    revenue: f64,
    counts_as_conversion: bool,
    counts_as_revenue: bool,
    counts_for_cost: bool,
    created_at_millis: i64,
}

#[derive(Clone, Debug)]
struct DashboardSnapshot {
    visits: Vec<VisitFact>,
    conversions: Vec<ConversionFact>,
    lander_clicks: i64,
    offer_clicks: i64,
    issue_events: i64,
    recent_events: Vec<DashboardRecentEvent>,
}

impl DashboardSnapshot {
    fn clicks(&self) -> i64 {
        self.lander_clicks + self.offer_clicks
    }
}

pub async fn dashboard_summary(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<DashboardSummaryResponse> {
    let campaign_templates = active_campaigns(pool).await?;
    let comparison_filter = previous_period(date_filter);
    let current_snapshot = dashboard_snapshot(pool, date_filter, true).await?;
    let current_campaigns = campaign_metrics_for_snapshot(&campaign_templates, &current_snapshot);
    let current_metrics = period_metrics(&current_snapshot, &current_campaigns);
    let previous_metrics = match comparison_filter {
        Some(filter) => {
            let snapshot = dashboard_snapshot(pool, filter, false).await?;
            let campaigns = campaign_metrics_for_snapshot(&campaign_templates, &snapshot);
            Some(period_metrics(&snapshot, &campaigns))
        }
        None => None,
    };
    let top_movers = top_movers(current_campaigns.clone());
    let generated_at_millis = now_millis()?;

    Ok(DashboardSummaryResponse {
        generated_at_millis,
        current_window: date_window("Selected range", date_filter),
        comparison_window: comparison_filter.map(|filter| date_window("Previous period", filter)),
        kpis: kpis(&current_metrics, previous_metrics.as_ref()),
        performance: performance_points(&current_snapshot, &campaign_templates),
        decision_feed: decision_feed(&current_metrics, &top_movers),
        top_movers,
        conversion_path: conversion_path(&current_snapshot),
        traffic_mix: traffic_mix(&current_snapshot.visits),
        recent_events: current_snapshot.recent_events,
        setup_health: setup_health(pool).await?,
    })
}

async fn dashboard_snapshot(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
    include_recent_events: bool,
) -> ServerResult<DashboardSnapshot> {
    Ok(DashboardSnapshot {
        visits: visit_facts(pool, date_filter).await?,
        conversions: conversion_facts(pool, date_filter).await?,
        lander_clicks: event_count(pool, date_filter, &[VisitEventType::LanderClick]).await?,
        offer_clicks: event_count(pool, date_filter, &[VisitEventType::OfferClick]).await?,
        issue_events: event_count(
            pool,
            date_filter,
            &[VisitEventType::Error, VisitEventType::ConditionDataMissing],
        )
        .await?,
        recent_events: if include_recent_events {
            recent_events(pool, date_filter).await?
        } else {
            Vec::new()
        },
    })
}

fn period_metrics(snapshot: &DashboardSnapshot, campaigns: &[CampaignMetrics]) -> PeriodMetrics {
    let conversions = snapshot
        .conversions
        .iter()
        .filter(|conversion| conversion.counts_as_conversion)
        .count();
    let revenue = snapshot
        .conversions
        .iter()
        .filter(|conversion| conversion.counts_as_revenue)
        .map(|conversion| conversion.revenue)
        .sum();

    PeriodMetrics {
        visits: i64::try_from(snapshot.visits.len()).unwrap_or(i64::MAX),
        unique_visits: unique_visit_count(&snapshot.visits),
        clicks: snapshot.clicks(),
        conversions: i64::try_from(conversions).unwrap_or(i64::MAX),
        revenue,
        cost: campaigns.iter().map(CampaignMetrics::cost).sum(),
        suspicious_visits: i64::try_from(
            snapshot
                .visits
                .iter()
                .filter(|visit| visit.suspicious)
                .count(),
        )
        .unwrap_or(i64::MAX),
        issue_events: snapshot.issue_events,
    }
}

fn campaign_metrics_for_snapshot(
    campaign_templates: &HashMap<String, CampaignMetrics>,
    snapshot: &DashboardSnapshot,
) -> Vec<CampaignMetrics> {
    let mut campaigns = campaign_templates.clone();

    for visit in &snapshot.visits {
        if let Some(campaign) = campaigns.get_mut(&visit.campaign_id) {
            campaign.visits += 1;
        }
    }
    for conversion in &snapshot.conversions {
        if let Some(campaign) = campaigns.get_mut(&conversion.campaign_id) {
            if conversion.counts_as_conversion {
                campaign.conversions += 1;
            }
            if conversion.counts_for_cost {
                campaign.cost_conversions += 1;
            }
            if conversion.counts_as_revenue {
                campaign.revenue += conversion.revenue;
            }
        }
    }

    campaigns.into_values().collect()
}

async fn active_campaigns(pool: &SqlitePool) -> ServerResult<HashMap<String, CampaignMetrics>> {
    let rows = sqlx::query(
        "SELECT campaigns.id, campaigns.name, traffic_sources.name AS detail,
                campaigns.cost_model, campaigns.cost_value
         FROM campaigns
         JOIN traffic_sources ON traffic_sources.id = campaigns.traffic_source_id
         WHERE campaigns.archived = 0",
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            let id: String = row.try_get("id")?;
            Ok((
                id.clone(),
                CampaignMetrics {
                    id,
                    name: row.try_get("name")?,
                    detail: row.try_get("detail")?,
                    cost_model: row.try_get("cost_model")?,
                    cost_value: row.try_get("cost_value")?,
                    ..CampaignMetrics::default()
                },
            ))
        })
        .collect()
}

async fn visit_facts(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<VisitFact>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
            "SELECT visits.id, visits.campaign_id, visits.ip_address, visits.user_agent,
                    visits.country, visits.region, visits.city, visits.timezone,
                    visits.postal_code, visits.metro_code, visits.asn, visits.asn_organization,
                    visits.isp, visits.connection_type, visits.proxy_type, visits.carrier,
                    visits.browser, visits.browser_version, visits.operating_system,
                    visits.operating_system_version, visits.device_type, visits.device_brand,
                    visits.device_model, visits.suspicious, visits.created_at_millis
             FROM visits
             JOIN campaigns ON campaigns.id = visits.campaign_id
                AND campaigns.archived = 0
             WHERE (? IS NULL OR visits.created_at_millis >= ?)
                AND (? IS NULL OR visits.created_at_millis < ?)",
        ),
        date_filter,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(VisitFact {
                id: row.try_get("id")?,
                campaign_id: row.try_get("campaign_id")?,
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
                suspicious: row.try_get("suspicious")?,
                created_at_millis: row.try_get("created_at_millis")?,
            })
        })
        .collect()
}

async fn conversion_facts(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<ConversionFact>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
            "SELECT conversion_events.campaign_id, conversion_events.revenue_value,
                    conversion_event_types.include_in_conversions,
                    conversion_event_types.include_in_revenue,
                    conversion_event_types.include_in_cost,
                    conversion_events.created_at_millis
             FROM conversion_events
             JOIN campaigns ON campaigns.id = conversion_events.campaign_id
                AND campaigns.archived = 0
             JOIN conversion_event_types
                ON conversion_event_types.id = conversion_events.event_type_id
                AND conversion_event_types.archived = 0
             WHERE conversion_events.duplicate = 0
                AND (? IS NULL OR conversion_events.created_at_millis >= ?)
                AND (? IS NULL OR conversion_events.created_at_millis < ?)",
        ),
        date_filter,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(ConversionFact {
                campaign_id: row.try_get("campaign_id")?,
                revenue: row.try_get("revenue_value")?,
                counts_as_conversion: row.try_get("include_in_conversions")?,
                counts_as_revenue: row.try_get("include_in_revenue")?,
                counts_for_cost: row.try_get("include_in_cost")?,
                created_at_millis: row.try_get("created_at_millis")?,
            })
        })
        .collect()
}

async fn event_count(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
    event_types: &[VisitEventType],
) -> ServerResult<i64> {
    let placeholders = event_types
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT COUNT(*)
         FROM visit_events
         JOIN campaigns ON campaigns.id = visit_events.campaign_id
            AND campaigns.archived = 0
         WHERE visit_events.event_type IN ({placeholders})
            AND (? IS NULL OR visit_events.created_at_millis >= ?)
            AND (? IS NULL OR visit_events.created_at_millis < ?)"
    );
    let mut query = sqlx::query_scalar::<_, i64>(&sql);
    for event_type in event_types {
        query = query.bind(visit_event_type_key(event_type.clone()));
    }
    Ok(bind_visit_date_filter_scalar(query, date_filter)
        .fetch_one(pool)
        .await?)
}

fn unique_visit_count(visits: &[VisitFact]) -> i64 {
    let mut keys = HashSet::new();
    for visit in visits {
        keys.insert(unique_visit_key(
            &visit.id,
            visit.ip_address.as_deref(),
            visit.user_agent.as_deref(),
        ));
    }
    i64::try_from(keys.len()).unwrap_or(i64::MAX)
}

fn top_movers(mut campaigns: Vec<CampaignMetrics>) -> Vec<DashboardTopMover> {
    campaigns.sort_by(|left, right| {
        right
            .profit()
            .partial_cmp(&left.profit())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| right.visits.cmp(&left.visits))
            .then_with(|| left.name.cmp(&right.name))
    });

    campaigns
        .into_iter()
        .filter(|campaign| campaign.visits > 0 || campaign.conversions > 0)
        .take(6)
        .map(|campaign| {
            let cost = campaign.cost();
            let profit = campaign.profit();
            let roi = campaign.roi();
            DashboardTopMover {
                category: "Campaign".to_string(),
                name: campaign.name,
                detail: campaign.detail,
                route_path: Some("/campaigns".to_string()),
                visits: campaign.visits,
                conversions: campaign.conversions,
                revenue: campaign.revenue,
                cost,
                profit,
                roi,
            }
        })
        .collect()
}

fn performance_points(
    snapshot: &DashboardSnapshot,
    campaign_templates: &HashMap<String, CampaignMetrics>,
) -> Vec<DashboardPerformancePoint> {
    let mut daily_campaigns = BTreeMap::<(i64, String), CampaignMetrics>::new();

    for visit in &snapshot.visits {
        let Some(day_start) = day_start_millis(visit.created_at_millis) else {
            continue;
        };
        let Some(campaign) = campaign_templates.get(&visit.campaign_id) else {
            continue;
        };
        let bucket = daily_campaigns
            .entry((day_start, campaign.id.clone()))
            .or_insert_with(|| campaign.clone());
        bucket.visits += 1;
    }

    for conversion in &snapshot.conversions {
        let Some(day_start) = day_start_millis(conversion.created_at_millis) else {
            continue;
        };
        let Some(campaign) = campaign_templates.get(&conversion.campaign_id) else {
            continue;
        };
        let bucket = daily_campaigns
            .entry((day_start, campaign.id.clone()))
            .or_insert_with(|| campaign.clone());
        if conversion.counts_as_conversion {
            bucket.conversions += 1;
        }
        if conversion.counts_for_cost {
            bucket.cost_conversions += 1;
        }
        if conversion.counts_as_revenue {
            bucket.revenue += conversion.revenue;
        }
    }

    let mut daily = BTreeMap::<i64, PeriodMetrics>::new();
    for ((day_start, _), campaign) in daily_campaigns {
        let bucket = daily.entry(day_start).or_default();
        bucket.visits += campaign.visits;
        bucket.conversions += campaign.conversions;
        bucket.revenue += campaign.revenue;
        bucket.cost += campaign.cost();
    }

    let mut points = daily
        .into_iter()
        .map(|(day_start, metrics)| DashboardPerformancePoint {
            label: day_label(day_start),
            start_at_millis: day_start,
            visits: metrics.visits,
            revenue: metrics.revenue,
            cost: metrics.cost,
            profit: metrics.profit(),
        })
        .collect::<Vec<_>>();
    if points.len() > MAX_PERFORMANCE_POINTS {
        points.split_off(points.len() - MAX_PERFORMANCE_POINTS)
    } else {
        points
    }
}

fn kpis(current: &PeriodMetrics, previous: Option<&PeriodMetrics>) -> Vec<DashboardKpi> {
    vec![
        kpi(
            "visits",
            "Visits",
            current.visits as f64,
            previous.map(|metrics| metrics.visits as f64),
            DashboardMetricUnit::Count,
            neutral_count_tone(current.visits),
            false,
        ),
        kpi(
            "unique_visits",
            "Unique",
            current.unique_visits as f64,
            previous.map(|metrics| metrics.unique_visits as f64),
            DashboardMetricUnit::Count,
            neutral_count_tone(current.unique_visits),
            false,
        ),
        kpi(
            "clicks",
            "Clicks",
            current.clicks as f64,
            previous.map(|metrics| metrics.clicks as f64),
            DashboardMetricUnit::Count,
            neutral_count_tone(current.clicks),
            false,
        ),
        kpi(
            "conversions",
            "Conversions",
            current.conversions as f64,
            previous.map(|metrics| metrics.conversions as f64),
            DashboardMetricUnit::Count,
            if current.visits > 0 && current.conversions == 0 {
                DashboardTone::Warning
            } else {
                neutral_count_tone(current.conversions)
            },
            false,
        ),
        kpi(
            "revenue",
            "Revenue",
            current.revenue,
            previous.map(|metrics| metrics.revenue),
            DashboardMetricUnit::Currency,
            neutral_money_tone(current.revenue),
            false,
        ),
        kpi(
            "cost",
            "Cost",
            current.cost,
            previous.map(|metrics| metrics.cost),
            DashboardMetricUnit::Currency,
            DashboardTone::Neutral,
            true,
        ),
        kpi(
            "profit",
            "Profit",
            current.profit(),
            previous.map(PeriodMetrics::profit),
            DashboardMetricUnit::Currency,
            profit_tone(current.profit()),
            true,
        ),
        kpi(
            "roi",
            "ROI",
            current.roi(),
            previous.map(PeriodMetrics::roi),
            DashboardMetricUnit::Percentage,
            profit_tone(current.roi()),
            true,
        ),
        kpi(
            "epc",
            "EPC",
            current.epc(),
            previous.map(PeriodMetrics::epc),
            DashboardMetricUnit::Currency,
            neutral_money_tone(current.epc()),
            false,
        ),
        kpi(
            "cpa",
            "CPA",
            current.cpa(),
            previous.map(PeriodMetrics::cpa),
            DashboardMetricUnit::Currency,
            DashboardTone::Neutral,
            true,
        ),
        kpi(
            "cr",
            "CR",
            current.conversion_rate(),
            previous.map(PeriodMetrics::conversion_rate),
            DashboardMetricUnit::Percentage,
            neutral_money_tone(current.conversion_rate()),
            false,
        ),
    ]
}

fn kpi(
    key: &str,
    label: &str,
    value: f64,
    previous_value: Option<f64>,
    unit: DashboardMetricUnit,
    tone: DashboardTone,
    estimated: bool,
) -> DashboardKpi {
    let value = clean_zero(value);
    let previous_value = previous_value.map(clean_zero);
    DashboardKpi {
        key: key.to_string(),
        label: label.to_string(),
        value,
        previous_value,
        delta_percent: previous_value.and_then(|previous| delta_percent(value, previous)),
        unit,
        tone,
        estimated,
    }
}

fn decision_feed(
    metrics: &PeriodMetrics,
    top_movers: &[DashboardTopMover],
) -> Vec<DashboardDecision> {
    let mut decisions = Vec::new();

    if metrics.visits > 0 && metrics.conversions == 0 {
        decisions.push(DashboardDecision {
            title: "No conversions".to_string(),
            detail: "Traffic is arriving, but no included conversions were tracked.".to_string(),
            tone: DashboardTone::Warning,
            action_label: "Review conversions".to_string(),
            route_path: Some("/conversions".to_string()),
        });
    }

    if metrics.profit() < 0.0 {
        decisions.push(DashboardDecision {
            title: "Negative profit".to_string(),
            detail: "Estimated cost is above tracked revenue in this range.".to_string(),
            tone: DashboardTone::Critical,
            action_label: "Open campaigns".to_string(),
            route_path: Some("/campaigns".to_string()),
        });
    }

    if let Some(winner) = top_movers
        .iter()
        .find(|mover| mover.conversions > 0 && mover.profit > 0.0 && mover.roi >= 50.0)
    {
        decisions.push(DashboardDecision {
            title: "Scale winner".to_string(),
            detail: format!(
                "{} is profitable with {:.1}% estimated ROI.",
                winner.name, winner.roi
            ),
            tone: DashboardTone::Positive,
            action_label: "Open campaign".to_string(),
            route_path: winner.route_path.clone(),
        });
    }

    if metrics.issue_events > 0 {
        decisions.push(DashboardDecision {
            title: "Fix tracking".to_string(),
            detail: format!(
                "{} error or missing-condition events need review.",
                metrics.issue_events
            ),
            tone: DashboardTone::Warning,
            action_label: "Review reports".to_string(),
            route_path: Some("/campaigns".to_string()),
        });
    }

    if metrics.suspicious_visits > 0 {
        decisions.push(DashboardDecision {
            title: "Check traffic quality".to_string(),
            detail: format!(
                "{} visits were flagged as suspicious.",
                metrics.suspicious_visits
            ),
            tone: DashboardTone::Warning,
            action_label: "Open traffic sources".to_string(),
            route_path: Some("/traffic-sources".to_string()),
        });
    }

    if decisions.is_empty() {
        decisions.push(DashboardDecision {
            title: "Dashboard healthy".to_string(),
            detail: "No urgent conversion, profit, tracking, or traffic-quality issues found."
                .to_string(),
            tone: DashboardTone::Positive,
            action_label: "View campaigns".to_string(),
            route_path: Some("/campaigns".to_string()),
        });
    }

    decisions
}

fn conversion_path(snapshot: &DashboardSnapshot) -> Vec<DashboardConversionPathStep> {
    let conversions = i64::try_from(
        snapshot
            .conversions
            .iter()
            .filter(|conversion| conversion.counts_as_conversion)
            .count(),
    )
    .unwrap_or(i64::MAX);
    let visit_count = i64::try_from(snapshot.visits.len()).unwrap_or(i64::MAX);

    vec![
        path_step("Visits", visit_count, None),
        path_step("Lander clicks", snapshot.lander_clicks, Some(visit_count)),
        path_step(
            "Offer clicks",
            snapshot.offer_clicks,
            Some(snapshot.lander_clicks.max(visit_count)),
        ),
        path_step(
            "Conversions",
            conversions,
            Some(snapshot.offer_clicks.max(visit_count)),
        ),
    ]
}

fn path_step(label: &str, count: i64, previous_count: Option<i64>) -> DashboardConversionPathStep {
    DashboardConversionPathStep {
        label: label.to_string(),
        count,
        rate_from_previous: previous_count
            .map(|previous| percentage_ratio(count as f64, previous as f64)),
    }
}

fn traffic_mix(visits: &[VisitFact]) -> Vec<DashboardTrafficMix> {
    [
        ("Geo", TrafficDimension::Geo),
        ("Device", TrafficDimension::Device),
        ("Browser", TrafficDimension::Browser),
        ("OS", TrafficDimension::Os),
    ]
    .into_iter()
    .map(|(label, dimension)| DashboardTrafficMix {
        dimension: label.to_string(),
        segments: traffic_segments(visits, dimension),
    })
    .collect()
}

#[derive(Clone, Copy, Debug)]
enum TrafficDimension {
    Geo,
    Device,
    Browser,
    Os,
}

fn traffic_segments(
    visits: &[VisitFact],
    dimension: TrafficDimension,
) -> Vec<DashboardTrafficSegment> {
    let mut buckets = HashMap::<String, i64>::new();
    for visit in visits {
        *buckets
            .entry(traffic_segment_label(visit, dimension))
            .or_default() += 1;
    }
    let total_visits = i64::try_from(visits.len()).unwrap_or(i64::MAX);
    let mut segments = buckets
        .into_iter()
        .map(|(label, visits)| DashboardTrafficSegment {
            label,
            visits,
            share_percent: percentage_ratio(visits as f64, total_visits as f64),
        })
        .collect::<Vec<_>>();
    segments.sort_by(|left, right| {
        right
            .visits
            .cmp(&left.visits)
            .then_with(|| left.label.cmp(&right.label))
    });
    segments.truncate(5);
    segments
}

fn traffic_segment_label(visit: &VisitFact, dimension: TrafficDimension) -> String {
    match dimension {
        TrafficDimension::Geo => label_optional(visit.enrichment.country.as_deref()),
        TrafficDimension::Device => label_with_user_agent_fallback(
            visit.enrichment.device_type.as_deref(),
            visit.user_agent.as_deref(),
            detect_device_type,
        ),
        TrafficDimension::Browser => label_with_user_agent_fallback(
            visit.enrichment.browser.as_deref(),
            visit.user_agent.as_deref(),
            detect_browser,
        ),
        TrafficDimension::Os => label_with_user_agent_fallback(
            visit.enrichment.operating_system.as_deref(),
            visit.user_agent.as_deref(),
            detect_operating_system,
        ),
    }
}

fn label_with_user_agent_fallback(
    persisted_value: Option<&str>,
    user_agent: Option<&str>,
    fallback: impl Fn(&str) -> String,
) -> String {
    normalized_label(persisted_value).unwrap_or_else(|| {
        user_agent
            .map(fallback)
            .unwrap_or_else(|| "Unknown".to_string())
    })
}

fn label_optional(value: Option<&str>) -> String {
    normalized_label(value).unwrap_or_else(|| "Unknown".to_string())
}

fn normalized_label(value: Option<&str>) -> Option<String> {
    let value = value?.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

async fn recent_events(
    pool: &SqlitePool,
    date_filter: VisitDateFilter,
) -> ServerResult<Vec<DashboardRecentEvent>> {
    let rows = bind_visit_date_filter(
        sqlx::query(
            "SELECT visit_events.event_type, campaigns.name AS campaign_name,
                    visit_events.created_at_millis
             FROM visit_events
             JOIN campaigns ON campaigns.id = visit_events.campaign_id
                AND campaigns.archived = 0
             WHERE (? IS NULL OR visit_events.created_at_millis >= ?)
                AND (? IS NULL OR visit_events.created_at_millis < ?)
             ORDER BY visit_events.created_at_millis DESC
             LIMIT 8",
        ),
        date_filter,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            let event_type: String = row.try_get("event_type")?;
            Ok(DashboardRecentEvent {
                label: event_label(&event_type).to_string(),
                detail: row
                    .try_get::<Option<String>, _>("campaign_name")?
                    .map(|campaign| format!("Campaign: {campaign}"))
                    .unwrap_or_else(|| "Tracker event".to_string()),
                occurred_at_millis: row.try_get("created_at_millis")?,
                tone: event_tone(&event_type),
            })
        })
        .collect()
}

async fn setup_health(pool: &SqlitePool) -> ServerResult<Vec<DashboardSetupHealthItem>> {
    let settings = sqlx::query(
        "SELECT tracking_base_url, domain_setup_status, geolite_last_download_at_millis
         FROM app_settings
         WHERE id = 1",
    )
    .fetch_one(pool)
    .await?;
    let active_campaigns: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM campaigns WHERE archived = 0")
            .fetch_one(pool)
            .await?;
    let conversion_types: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM conversion_event_types WHERE archived = 0")
            .fetch_one(pool)
            .await?;
    let tracking_base_url: String = settings.try_get("tracking_base_url")?;
    let domain_setup_status: String = settings.try_get("domain_setup_status")?;
    let geolite_last_download_at_millis: Option<i64> =
        settings.try_get("geolite_last_download_at_millis")?;

    Ok(vec![
        DashboardSetupHealthItem {
            label: "Tracking domain".to_string(),
            detail: if tracking_base_url.trim().is_empty() {
                "Tracking base URL is not configured".to_string()
            } else {
                format!("{tracking_base_url} ({domain_setup_status})")
            },
            tone: if tracking_base_url.trim().is_empty() {
                DashboardTone::Warning
            } else {
                DashboardTone::Positive
            },
            route_path: Some("/settings/domain".to_string()),
        },
        DashboardSetupHealthItem {
            label: "Active campaigns".to_string(),
            detail: format!("{active_campaigns} active campaigns"),
            tone: if active_campaigns > 0 {
                DashboardTone::Positive
            } else {
                DashboardTone::Warning
            },
            route_path: Some("/campaigns".to_string()),
        },
        DashboardSetupHealthItem {
            label: "Conversion events".to_string(),
            detail: format!("{conversion_types} configured event types"),
            tone: if conversion_types > 0 {
                DashboardTone::Positive
            } else {
                DashboardTone::Warning
            },
            route_path: Some("/conversions".to_string()),
        },
        DashboardSetupHealthItem {
            label: "Geo enrichment".to_string(),
            detail: geolite_last_download_at_millis
                .map(|_| "GeoLite databases have been downloaded".to_string())
                .unwrap_or_else(|| "GeoLite databases are not downloaded yet".to_string()),
            tone: if geolite_last_download_at_millis.is_some() {
                DashboardTone::Positive
            } else {
                DashboardTone::Warning
            },
            route_path: Some("/settings/geolocation".to_string()),
        },
    ])
}

fn estimated_cost(
    cost_model: &str,
    cost_value: f64,
    visits: i64,
    conversions: i64,
    revenue: f64,
) -> f64 {
    match normalized_cost_model(cost_model).as_str() {
        "cpc" => visits as f64 * cost_value,
        "cpm" => visits as f64 * cost_value / 1000.0,
        "cpa" => conversions as f64 * cost_value,
        "revshare" => revenue * cost_value / 100.0,
        _ => 0.0,
    }
}

fn normalized_cost_model(cost_model: &str) -> String {
    cost_model
        .trim()
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn previous_period(date_filter: VisitDateFilter) -> Option<VisitDateFilter> {
    let start = date_filter.start_at_millis?;
    let end = date_filter.end_at_millis?;
    if end <= start {
        return None;
    }
    let duration = end - start;
    Some(VisitDateFilter::new(Some(start - duration), Some(start)))
}

fn date_window(label: &str, date_filter: VisitDateFilter) -> DashboardDateWindow {
    DashboardDateWindow {
        label: if date_filter.start_at_millis.is_none() && date_filter.end_at_millis.is_none() {
            "All time".to_string()
        } else {
            label.to_string()
        },
        start_at_millis: date_filter.start_at_millis,
        end_at_millis: date_filter.end_at_millis,
    }
}

fn ratio(numerator: f64, denominator: i64) -> f64 {
    if denominator <= 0 {
        0.0
    } else {
        numerator / denominator as f64
    }
}

fn percentage_ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        (numerator / denominator) * 100.0
    }
}

fn delta_percent(current: f64, previous: f64) -> Option<f64> {
    if previous.abs() < f64::EPSILON {
        None
    } else {
        Some(((current - previous) / previous.abs()) * 100.0)
    }
}

fn clean_zero(value: f64) -> f64 {
    if value.abs() < f64::EPSILON {
        0.0
    } else {
        value
    }
}

fn neutral_count_tone(value: i64) -> DashboardTone {
    if value > 0 {
        DashboardTone::Positive
    } else {
        DashboardTone::Neutral
    }
}

fn neutral_money_tone(value: f64) -> DashboardTone {
    if value > 0.0 {
        DashboardTone::Positive
    } else {
        DashboardTone::Neutral
    }
}

fn profit_tone(value: f64) -> DashboardTone {
    if value > 0.0 {
        DashboardTone::Positive
    } else if value < 0.0 {
        DashboardTone::Critical
    } else {
        DashboardTone::Neutral
    }
}

fn event_label(event_type: &str) -> &'static str {
    if event_type == visit_event_type_key(VisitEventType::CampaignClick) {
        "Visit"
    } else if event_type == visit_event_type_key(VisitEventType::LanderClick) {
        "Lander click"
    } else if event_type == visit_event_type_key(VisitEventType::OfferClick) {
        "Offer click"
    } else if event_type == visit_event_type_key(VisitEventType::Conversion) {
        "Conversion"
    } else if event_type == visit_event_type_key(VisitEventType::CustomConversion) {
        "Custom conversion"
    } else if event_type == visit_event_type_key(VisitEventType::ConditionDataMissing) {
        "Missing condition data"
    } else if event_type == visit_event_type_key(VisitEventType::Error) {
        "Tracking error"
    } else {
        "Tracker event"
    }
}

fn event_tone(event_type: &str) -> DashboardTone {
    if event_type == visit_event_type_key(VisitEventType::Conversion)
        || event_type == visit_event_type_key(VisitEventType::CustomConversion)
    {
        DashboardTone::Positive
    } else if event_type == visit_event_type_key(VisitEventType::ConditionDataMissing) {
        DashboardTone::Warning
    } else if event_type == visit_event_type_key(VisitEventType::Error) {
        DashboardTone::Critical
    } else {
        DashboardTone::Neutral
    }
}

fn day_start_millis(value: i64) -> Option<i64> {
    let datetime = Utc.timestamp_millis_opt(value).single()?;
    Utc.with_ymd_and_hms(datetime.year(), datetime.month(), datetime.day(), 0, 0, 0)
        .single()
        .map(|day| day.timestamp_millis())
}

fn day_label(day_start: i64) -> String {
    match Utc.timestamp_millis_opt(day_start).single() {
        Some(datetime) => format!(
            "{:04}-{:02}-{:02}",
            datetime.year(),
            datetime.month(),
            datetime.day()
        ),
        None => "Unknown".to_string(),
    }
}
