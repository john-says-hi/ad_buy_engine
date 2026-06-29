use crate::config::OutputFormat;
use crate::metrics::RunSummary;

pub fn format_summary(
    summary: &RunSummary,
    output_format: OutputFormat,
) -> Result<String, serde_json::Error> {
    let mut summary = summary.clone();
    summary.finalize();
    match output_format {
        OutputFormat::Json => serde_json::to_string_pretty(&summary),
        OutputFormat::Table => Ok(format_table(&summary)),
    }
}

fn format_table(summary: &RunSummary) -> String {
    let status_line = if summary.http.status_buckets.is_empty() {
        "none".to_string()
    } else {
        summary
            .http
            .status_buckets
            .iter()
            .map(|(status, count)| format!("{status}={count}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let error_line = if summary.http.error_buckets.is_empty() {
        "none".to_string()
    } else {
        summary
            .http
            .error_buckets
            .iter()
            .map(|(error, count)| format!("{error}={count}"))
            .collect::<Vec<_>>()
            .join(", ")
    };

    format!(
        "\
Ad Buy Engine fake traffic run
campaign_url: {campaign_url}
dry_run: {dry_run}
sessions: {completed}/{planned} completed ({failed} failed)
requests: {requests}
statuses: {statuses}
errors: {errors}
redirects: {redirects} steps, {blocked_redirects} blocked
conversions: {conversions_sent}/{conversions_attempted} sent
latency_ms: min={min} p50={p50} p95={p95} p99={p99} max={max} avg={avg:.1}",
        campaign_url = summary.campaign_url,
        dry_run = summary.dry_run,
        completed = summary.completed_sessions,
        planned = summary.planned_sessions,
        failed = summary.failed_sessions,
        requests = summary.http.requests,
        statuses = status_line,
        errors = error_line,
        redirects = summary.redirects.steps,
        blocked_redirects = summary.redirects.blocked,
        conversions_sent = summary.conversions.sent,
        conversions_attempted = summary.conversions.attempted,
        min = summary.http.latency_ms.min,
        p50 = summary.http.latency_ms.p50,
        p95 = summary.http.latency_ms.p95,
        p99 = summary.http.latency_ms.p99,
        max = summary.http.latency_ms.max,
        avg = summary.http.latency_ms.average,
    )
}
