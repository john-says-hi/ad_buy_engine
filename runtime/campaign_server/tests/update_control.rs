use std::fs;

use ad_buy_engine_domain::{
    RollbackEligibility, UPDATE_LOCK_FILE, UPDATE_STATUS_FILE, UpdateLock, UpdatePhase,
    UpdateRequestKind, UpdateResult, UpdateSlot, UpdateStatusResponse,
};
use campaign_server::config::UpdateConfig;
use campaign_server::services::updates::{queue_request, status};
use tempfile::tempdir;

#[test]
fn only_one_update_request_can_be_pending() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let config = update_config(tempdir.path().join("control"));

    let first = queue_request(
        &config,
        "0.1.0",
        UpdateRequestKind::Install,
        "operator".to_string(),
        Some("v0.2.0".to_string()),
    )?;
    let second = queue_request(
        &config,
        "0.1.0",
        UpdateRequestKind::Install,
        "operator".to_string(),
        Some("v0.2.0".to_string()),
    );

    assert_eq!(first.phase, UpdatePhase::UpdateRequested);
    assert!(second.is_err());
    Ok(())
}

#[test]
fn update_status_survives_process_restart() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let config = update_config(tempdir.path().join("control"));
    fs::create_dir_all(&config.control_dir)?;
    fs::write(
        config.control_dir.join(UPDATE_STATUS_FILE),
        serde_json::to_vec(&UpdateStatusResponse {
            enabled: true,
            current_version: "old".to_string(),
            latest_version: Some("v0.2.0".to_string()),
            active_slot: Some(UpdateSlot::Green),
            phase: UpdatePhase::Succeeded,
            last_result: Some(UpdateResult {
                phase: UpdatePhase::Succeeded,
                success: true,
                version: Some("v0.2.0".to_string()),
                message: "Installed".to_string(),
                completed_at_millis: 100,
            }),
            rollback: RollbackEligibility::allowed("v0.1.0"),
            message: None,
        })?,
    )?;

    let loaded = status(&config, "0.2.0")?;

    assert_eq!(loaded.current_version, "0.2.0");
    assert_eq!(loaded.latest_version, Some("v0.2.0".to_string()));
    assert_eq!(loaded.active_slot, Some(UpdateSlot::Green));
    assert_eq!(loaded.phase, UpdatePhase::Succeeded);
    assert!(loaded.rollback.eligible);
    Ok(())
}

#[test]
fn pending_request_status_survives_dashboard_reload() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let config = update_config(tempdir.path().join("control"));
    queue_request(
        &config,
        "0.1.0",
        UpdateRequestKind::Install,
        "operator".to_string(),
        Some("v0.2.0".to_string()),
    )?;

    let loaded = status(&config, "0.1.0")?;

    assert_eq!(loaded.phase, UpdatePhase::UpdateRequested);
    assert_eq!(loaded.message, Some("Update request queued".to_string()));
    Ok(())
}

#[test]
fn partial_status_writes_are_ignored() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let config = update_config(tempdir.path().join("control"));
    fs::create_dir_all(&config.control_dir)?;
    fs::write(config.control_dir.join(UPDATE_STATUS_FILE), b"{ partial")?;

    let loaded = status(&config, "0.1.0")?;

    assert_eq!(loaded.phase, UpdatePhase::Idle);
    assert_eq!(loaded.current_version, "0.1.0");
    Ok(())
}

#[test]
fn stale_locks_are_removed_before_queueing() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempdir()?;
    let config = update_config(tempdir.path().join("control"));
    fs::create_dir_all(&config.control_dir)?;
    fs::write(
        config.control_dir.join(UPDATE_LOCK_FILE),
        serde_json::to_vec(&UpdateLock {
            operation_id: "old".to_string(),
            created_at_millis: 1,
        })?,
    )?;

    let queued = queue_request(
        &config,
        "0.1.0",
        UpdateRequestKind::Check,
        "operator".to_string(),
        None,
    )?;

    assert_eq!(queued.phase, UpdatePhase::Checking);
    Ok(())
}

fn update_config(control_dir: std::path::PathBuf) -> UpdateConfig {
    UpdateConfig {
        enabled: true,
        control_dir,
        repo: "john-says-hi/ad_buy_engine".to_string(),
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        active_slot: Some(UpdateSlot::Blue),
    }
}
