use ad_buy_engine_domain::{
    DashboardConversionPathStep, DashboardDecision, DashboardKpi, DashboardMetricUnit,
    DashboardPerformancePoint, DashboardRecentEvent, DashboardSetupHealthItem,
    DashboardSummaryResponse, DashboardTone, DashboardTopMover, DashboardTrafficMix,
};
use web_sys::HtmlSelectElement;
use yew::TargetCast;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::client;
use crate::state::report::{DATE_RANGE_OPTIONS, ReportDateRange};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DashboardPageProps {
    pub date_range: ReportDateRange,
    pub on_date_range_change: Callback<ReportDateRange>,
}

#[function_component(DashboardPage)]
pub fn dashboard_page(props: &DashboardPageProps) -> Html {
    let summary = use_state(|| None::<DashboardSummaryResponse>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let refresh_version = use_state(|| 0_u64);
    let request_sequence = use_mut_ref(|| 0_u64);

    {
        let summary = summary.clone();
        let loading = loading.clone();
        let error = error.clone();
        let request_sequence = request_sequence.clone();
        let selected_date_range = props.date_range;
        let refresh_version = *refresh_version;
        use_effect_with(
            (selected_date_range, refresh_version),
            move |(date_range, _)| {
                let date_range = *date_range;
                let request_id = {
                    let mut sequence = request_sequence.borrow_mut();
                    *sequence += 1;
                    *sequence
                };
                loading.set(true);
                error.set(None);
                spawn_local(async move {
                    let result = client::get_dashboard_summary(date_range).await;
                    if *request_sequence.borrow() != request_id {
                        return;
                    }
                    match result {
                        Ok(response) => summary.set(Some(response)),
                        Err(message) => error.set(Some(message)),
                    }
                    loading.set(false);
                });
                || ()
            },
        );
    }

    let on_refresh = {
        let refresh_version = refresh_version.clone();
        Callback::from(move |_| refresh_version.set(*refresh_version + 1))
    };

    html! {
        <main class="abe-dashboard">
            <DashboardHeader
                date_range={props.date_range}
                loading={*loading}
                on_date_range_change={props.on_date_range_change.clone()}
                on_refresh={on_refresh}
            />
            {
                if *loading && summary.is_none() {
                    html! { <DashboardStatus message="Loading dashboard" /> }
                } else if let Some(message) = error.as_ref() {
                    html! { <DashboardStatus message={message.clone()} /> }
                } else if let Some(summary) = summary.as_ref() {
                    render_dashboard_summary(summary)
                } else {
                    html! { <DashboardStatus message="No dashboard data yet" /> }
                }
            }
        </main>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct DashboardHeaderProps {
    date_range: ReportDateRange,
    loading: bool,
    on_date_range_change: Callback<ReportDateRange>,
    on_refresh: Callback<MouseEvent>,
}

#[function_component(DashboardHeader)]
fn dashboard_header(props: &DashboardHeaderProps) -> Html {
    let on_change = {
        let on_date_range_change = props.on_date_range_change.clone();
        Callback::from(move |event: Event| {
            let select: HtmlSelectElement = event.target_unchecked_into();
            if let Some(date_range) = ReportDateRange::from_storage_key(&select.value()) {
                on_date_range_change.emit(date_range);
            }
        })
    };

    html! {
        <header class="abe-dashboard-header">
            <div>
                <h1>{ "Dashboard" }</h1>
                <p>{ "Performance, tracking health, and optimization signals" }</p>
            </div>
            <div class="abe-dashboard-controls">
                <label class="abe-dashboard-select">
                    <span>{ "Date Range" }</span>
                    <select
                        class="uk-select"
                        onchange={on_change}
                        value={props.date_range.storage_key().to_string()}
                    >
                        { for DATE_RANGE_OPTIONS.iter().map(|option| html! {
                            <option value={option.storage_key()}>
                                { option.label() }
                            </option>
                        }) }
                    </select>
                </label>
                <button
                    class="uk-button uk-button-default uk-button-small abe-dashboard-refresh"
                    type="button"
                    disabled={props.loading}
                    onclick={props.on_refresh.clone()}
                >
                    <span uk-icon="icon: refresh"></span>
                    { if props.loading { "Refreshing" } else { "Refresh" } }
                </button>
            </div>
        </header>
    }
}

fn render_dashboard_summary(summary: &DashboardSummaryResponse) -> Html {
    html! {
        <>
            <section class="abe-kpi-ribbon" aria-label="Dashboard KPIs">
                { for summary.kpis.iter().map(render_kpi) }
            </section>
            <section class="abe-dashboard-grid abe-dashboard-grid-primary">
                <PerformancePanel points={summary.performance.clone()} />
                <DecisionFeed decisions={summary.decision_feed.clone()} />
            </section>
            <section class="abe-dashboard-grid abe-dashboard-grid-secondary">
                <TopMovers movers={summary.top_movers.clone()} />
                <ConversionPath steps={summary.conversion_path.clone()} />
                <TrafficMix mixes={summary.traffic_mix.clone()} />
            </section>
            <section class="abe-dashboard-grid abe-dashboard-grid-bottom">
                <RecentEvents events={summary.recent_events.clone()} />
                <SetupHealth items={summary.setup_health.clone()} />
            </section>
        </>
    }
}

fn render_kpi(kpi: &DashboardKpi) -> Html {
    html! {
        <article class={classes!("abe-kpi", tone_class(kpi.tone))}>
            <div class="abe-kpi-label">
                <span>{ kpi.label.clone() }</span>
                { if kpi.estimated { html! { <em>{ "est." }</em> } } else { Html::default() } }
            </div>
            <strong>{ dashboard_metric_text(kpi.value, kpi.unit) }</strong>
            <small>{ dashboard_delta_text(kpi.delta_percent) }</small>
        </article>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct PerformancePanelProps {
    points: Vec<DashboardPerformancePoint>,
}

#[function_component(PerformancePanel)]
fn performance_panel(props: &PerformancePanelProps) -> Html {
    let max_value = props
        .points
        .iter()
        .flat_map(|point| [point.revenue.abs(), point.cost.abs(), point.profit.abs()])
        .fold(0.0, f64::max)
        .max(1.0);

    html! {
        <section class="abe-dashboard-panel abe-performance-panel">
            <PanelHeader title="Performance" subtitle="Revenue, cost, profit, and visits" />
            {
                if props.points.is_empty() {
                    html! { <p class="abe-dashboard-empty">{ "No performance data yet" }</p> }
                } else {
                    html! {
                        <div class="abe-performance-chart">
                            { for props.points.iter().map(|point| render_performance_point(point, max_value)) }
                        </div>
                    }
                }
            }
        </section>
    }
}

fn render_performance_point(point: &DashboardPerformancePoint, max_value: f64) -> Html {
    html! {
        <div class="abe-performance-day">
            <div class="abe-performance-bars" title={point.label.clone()}>
                <span
                    class="abe-chart-bar abe-chart-revenue"
                    style={bar_style(point.revenue, max_value)}
                    aria-label="Revenue"
                ></span>
                <span
                    class="abe-chart-bar abe-chart-cost"
                    style={bar_style(point.cost, max_value)}
                    aria-label="Cost"
                ></span>
                <span
                    class={classes!("abe-chart-bar", if point.profit >= 0.0 { "abe-chart-profit" } else { "abe-chart-loss" })}
                    style={bar_style(point.profit.abs(), max_value)}
                    aria-label="Profit"
                ></span>
                <i style={visit_marker_style(point.visits)}></i>
            </div>
            <small>{ short_date_label(&point.label) }</small>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct DecisionFeedProps {
    decisions: Vec<DashboardDecision>,
}

#[function_component(DecisionFeed)]
fn decision_feed(props: &DecisionFeedProps) -> Html {
    html! {
        <section class="abe-dashboard-panel">
            <PanelHeader title="Decision Feed" subtitle="Prioritized account signals" />
            <div class="abe-decision-list">
                { for props.decisions.iter().map(render_decision) }
            </div>
        </section>
    }
}

fn render_decision(decision: &DashboardDecision) -> Html {
    html! {
        <article class={classes!("abe-decision", tone_class(decision.tone))}>
            <div>
                <strong>{ decision.title.clone() }</strong>
                <p>{ decision.detail.clone() }</p>
            </div>
            { render_route_action(decision.route_path.as_deref(), &decision.action_label) }
        </article>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct TopMoversProps {
    movers: Vec<DashboardTopMover>,
}

#[function_component(TopMovers)]
fn top_movers(props: &TopMoversProps) -> Html {
    html! {
        <section class="abe-dashboard-panel abe-top-movers">
            <PanelHeader title="Top Movers" subtitle="Campaigns ranked by estimated profit" />
            <div class="abe-dashboard-table-wrap">
                <table class="uk-table uk-table-small abe-dashboard-table">
                    <thead>
                        <tr>
                            <th>{ "Name" }</th>
                            <th>{ "Visits" }</th>
                            <th>{ "Conv." }</th>
                            <th>{ "Profit" }</th>
                            <th>{ "ROI" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            if props.movers.is_empty() {
                                html! { <tr><td colspan="5">{ "No active movers yet" }</td></tr> }
                            } else {
                                html! { <>{ for props.movers.iter().map(render_mover_row) }</> }
                            }
                        }
                    </tbody>
                </table>
            </div>
        </section>
    }
}

fn render_mover_row(mover: &DashboardTopMover) -> Html {
    html! {
        <tr>
            <td>
                <strong>{ mover.name.clone() }</strong>
                <span>{ mover.detail.clone() }</span>
            </td>
            <td>{ mover.visits }</td>
            <td>{ mover.conversions }</td>
            <td class={classes!(if mover.profit >= 0.0 { "abe-money-positive" } else { "abe-money-negative" })}>
                { dashboard_metric_text(mover.profit, DashboardMetricUnit::Currency) }
            </td>
            <td>{ dashboard_metric_text(mover.roi, DashboardMetricUnit::Percentage) }</td>
        </tr>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct ConversionPathProps {
    steps: Vec<DashboardConversionPathStep>,
}

#[function_component(ConversionPath)]
fn conversion_path(props: &ConversionPathProps) -> Html {
    let max_count = props
        .steps
        .iter()
        .map(|step| step.count)
        .max()
        .unwrap_or(1)
        .max(1);
    html! {
        <section class="abe-dashboard-panel">
            <PanelHeader title="Conversion Path" subtitle="Visit to conversion flow" />
            <div class="abe-path-list">
                { for props.steps.iter().map(|step| render_path_step(step, max_count)) }
            </div>
        </section>
    }
}

fn render_path_step(step: &DashboardConversionPathStep, max_count: i64) -> Html {
    let width = percentage_ratio(step.count as f64, max_count as f64).max(4.0);
    html! {
        <div class="abe-path-step">
            <div>
                <strong>{ step.label.clone() }</strong>
                <span>{ step.count }</span>
            </div>
            <div class="abe-path-track">
                <i style={format!("width: {width:.1}%;")}></i>
            </div>
            <small>{ dashboard_delta_text(step.rate_from_previous) }</small>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct TrafficMixProps {
    mixes: Vec<DashboardTrafficMix>,
}

#[function_component(TrafficMix)]
fn traffic_mix(props: &TrafficMixProps) -> Html {
    html! {
        <section class="abe-dashboard-panel">
            <PanelHeader title="Traffic Mix" subtitle="Geo, device, browser, and OS" />
            <div class="abe-traffic-mix-list">
                { for props.mixes.iter().map(render_mix) }
            </div>
        </section>
    }
}

fn render_mix(mix: &DashboardTrafficMix) -> Html {
    html! {
        <article class="abe-traffic-mix">
            <h3>{ mix.dimension.clone() }</h3>
            {
                if mix.segments.is_empty() {
                    html! { <p>{ "No visits" }</p> }
                } else {
                    html! {
                        <>
                            { for mix.segments.iter().map(|segment| html! {
                                <div class="abe-mix-row">
                                    <span>{ segment.label.clone() }</span>
                                    <div><i style={format!("width: {:.1}%;", segment.share_percent.max(2.0))}></i></div>
                                    <em>{ dashboard_metric_text(segment.share_percent, DashboardMetricUnit::Percentage) }</em>
                                </div>
                            }) }
                        </>
                    }
                }
            }
        </article>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct RecentEventsProps {
    events: Vec<DashboardRecentEvent>,
}

#[function_component(RecentEvents)]
fn recent_events(props: &RecentEventsProps) -> Html {
    html! {
        <section class="abe-dashboard-panel">
            <PanelHeader title="Recent Events" subtitle="Latest tracker activity" />
            <div class="abe-recent-list">
                {
                    if props.events.is_empty() {
                        html! { <p class="abe-dashboard-empty">{ "No recent events" }</p> }
                    } else {
                        html! { <>{ for props.events.iter().map(render_recent_event) }</> }
                    }
                }
            </div>
        </section>
    }
}

fn render_recent_event(event: &DashboardRecentEvent) -> Html {
    html! {
        <article class={classes!("abe-recent-event", tone_class(event.tone))}>
            <span></span>
            <div>
                <strong>{ event.label.clone() }</strong>
                <p>{ event.detail.clone() }</p>
            </div>
        </article>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct SetupHealthProps {
    items: Vec<DashboardSetupHealthItem>,
}

#[function_component(SetupHealth)]
fn setup_health(props: &SetupHealthProps) -> Html {
    html! {
        <section class="abe-dashboard-panel">
            <PanelHeader title="Setup Health" subtitle="Tracking configuration status" />
            <div class="abe-health-list">
                { for props.items.iter().map(render_health_item) }
            </div>
        </section>
    }
}

fn render_health_item(item: &DashboardSetupHealthItem) -> Html {
    html! {
        <article class={classes!("abe-health-item", tone_class(item.tone))}>
            <div>
                <strong>{ item.label.clone() }</strong>
                <p>{ item.detail.clone() }</p>
            </div>
            { render_route_action(item.route_path.as_deref(), "Open") }
        </article>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct PanelHeaderProps {
    title: &'static str,
    subtitle: &'static str,
}

#[function_component(PanelHeader)]
fn panel_header(props: &PanelHeaderProps) -> Html {
    html! {
        <header class="abe-panel-header">
            <div>
                <h2>{ props.title }</h2>
                <p>{ props.subtitle }</p>
            </div>
        </header>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct DashboardStatusProps {
    message: String,
}

#[function_component(DashboardStatus)]
fn dashboard_status(props: &DashboardStatusProps) -> Html {
    html! {
        <section class="abe-dashboard-panel abe-dashboard-status">
            <p>{ props.message.clone() }</p>
        </section>
    }
}

fn render_route_action(route_path: Option<&str>, label: &str) -> Html {
    match route_path {
        Some(path) => html! {
            <a class="uk-button uk-button-default uk-button-small abe-dashboard-action" href={path.to_string()}>
                { label }
            </a>
        },
        None => Html::default(),
    }
}

pub fn dashboard_metric_text(value: f64, unit: DashboardMetricUnit) -> String {
    let value = if value.abs() < f64::EPSILON {
        0.0
    } else {
        value
    };
    match unit {
        DashboardMetricUnit::Count => format!("{:.0}", value),
        DashboardMetricUnit::Currency if value < 0.0 => format!("-${:.2}", value.abs()),
        DashboardMetricUnit::Currency => format!("${:.2}", value),
        DashboardMetricUnit::Percentage => format!("{:.1}%", value),
        DashboardMetricUnit::Ratio => format!("{:.2}", value),
    }
}

pub fn dashboard_delta_text(delta_percent: Option<f64>) -> String {
    match delta_percent {
        Some(delta) if delta > 0.0 => format!("+{delta:.1}%"),
        Some(delta) => format!("{delta:.1}%"),
        None => "No comparison".to_string(),
    }
}

fn tone_class(tone: DashboardTone) -> &'static str {
    match tone {
        DashboardTone::Neutral => "abe-tone-neutral",
        DashboardTone::Positive => "abe-tone-positive",
        DashboardTone::Warning => "abe-tone-warning",
        DashboardTone::Critical => "abe-tone-critical",
    }
}

fn bar_style(value: f64, max_value: f64) -> String {
    let height = percentage_ratio(value, max_value).clamp(2.0, 100.0);
    format!("height: {height:.1}%;")
}

fn visit_marker_style(visits: i64) -> String {
    let bottom = (visits as f64).clamp(3.0, 100.0);
    format!("bottom: {bottom:.1}%;")
}

fn percentage_ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        (numerator / denominator) * 100.0
    }
}

fn short_date_label(label: &str) -> String {
    label.split('-').skip(1).collect::<Vec<_>>().join("/")
}
