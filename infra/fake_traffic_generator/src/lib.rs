pub mod cli;
pub mod config;
pub mod conversions;
pub mod metrics;
pub mod output;
pub mod profiles;
pub mod redirects;
pub mod safety;
pub mod scheduler;
pub mod session;

use std::sync::Arc;

use config::RunConfig;
use metrics::RunSummary;
use scheduler::build_schedule;
use session::run_session;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

#[derive(Debug, Error)]
pub enum TrafficGeneratorError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("scheduled session task failed: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),
}

pub async fn run(config: RunConfig) -> Result<RunSummary, TrafficGeneratorError> {
    let schedule = build_schedule(&config);
    let planned_sessions = schedule.len() as u64;
    if config.dry_run {
        return Ok(RunSummary::dry_run(&config, planned_sessions));
    }

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(config.request_timeout)
        .build()?;

    let mut summary = RunSummary::new(&config, planned_sessions);
    if let Some(record) = session::preflight_health(&client, &config).await {
        summary.record_request(&record);
    }

    let semaphore = Arc::new(Semaphore::new(config.concurrency));
    let shared_config = Arc::new(config);
    let shared_client = Arc::new(client);
    let mut join_set = JoinSet::new();

    for scheduled_session in schedule {
        let semaphore = Arc::clone(&semaphore);
        let config = Arc::clone(&shared_config);
        let client = Arc::clone(&shared_client);
        join_set.spawn(async move {
            tokio::time::sleep(scheduled_session.start_after).await;
            let permit = semaphore
                .acquire_owned()
                .await
                .map_err(|error| error.to_string())?;
            let outcome = run_session(&client, &config, scheduled_session).await;
            drop(permit);
            Ok::<_, String>(outcome)
        });
    }

    while let Some(result) = join_set.join_next().await {
        match result? {
            Ok(outcome) => summary.record_session(&outcome),
            Err(error) => summary.record_internal_error(error),
        }
    }

    summary.finalize();
    Ok(summary)
}
