use std::fs;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{
    UPDATE_LOCK_FILE, UPDATE_REQUEST_FILE, UPDATE_STATUS_FILE, UpdateControlRequest, UpdateLock,
    UpdatePhase, UpdateRequestKind, UpdateStatusResponse,
};
use uuid::Uuid;

use crate::config::UpdateConfig;
use crate::error::{ServerError, ServerResult};
use crate::time::now_millis;

const STALE_LOCK_MILLIS: i64 = 30 * 60 * 1000;

pub fn status(config: &UpdateConfig, current_version: &str) -> ServerResult<UpdateStatusResponse> {
    if !config.enabled {
        return Ok(UpdateStatusResponse::disabled(
            current_version.to_string(),
            config.active_slot,
        ));
    }

    let mut status =
        read_json::<UpdateStatusResponse>(&status_path(config))?.unwrap_or_else(|| {
            UpdateStatusResponse::idle(current_version.to_string(), config.active_slot)
        });
    status.enabled = true;
    status.current_version = current_version.to_string();
    if status.active_slot.is_none() {
        status.active_slot = config.active_slot;
    }
    if let Some(request) = read_json::<UpdateControlRequest>(&request_path(config))? {
        status.phase = queued_phase(request.kind);
        status.message = Some("Update request queued".to_string());
    }
    Ok(status)
}

pub fn queue_request(
    config: &UpdateConfig,
    current_version: &str,
    kind: UpdateRequestKind,
    requested_by: String,
    requested_version: Option<String>,
) -> ServerResult<UpdateStatusResponse> {
    ensure_enabled(config)?;
    fs::create_dir_all(&config.control_dir)?;
    reject_if_busy(&config.control_dir)?;

    let request = UpdateControlRequest {
        id: Uuid::new_v4().to_string(),
        kind,
        requested_version,
        requested_at_millis: now_millis()?,
        requested_by,
        repo: config.repo.clone(),
        target_triple: config.target_triple.clone(),
    };
    write_json_atomic(&request_path(config), &request)?;

    let mut response = status(config, current_version)?;
    response.phase = queued_phase(kind);
    response.message = Some("Update request queued".to_string());
    Ok(response)
}

fn ensure_enabled(config: &UpdateConfig) -> ServerResult<()> {
    if config.enabled {
        Ok(())
    } else {
        Err(ServerError::forbidden("Updates are disabled"))
    }
}

fn reject_if_busy(control_dir: &Path) -> ServerResult<()> {
    let lock_path = control_dir.join(UPDATE_LOCK_FILE);
    if let Some(lock) = read_json::<UpdateLock>(&lock_path)? {
        let age_millis = now_millis()?.saturating_sub(lock.created_at_millis);
        if age_millis <= STALE_LOCK_MILLIS {
            return Err(ServerError::conflict("An update is already running"));
        }
        fs::remove_file(&lock_path)?;
    }

    let request_path = control_dir.join(UPDATE_REQUEST_FILE);
    if read_json::<UpdateControlRequest>(&request_path)?.is_some() {
        return Err(ServerError::conflict(
            "An update request is already pending",
        ));
    }
    remove_file_if_exists(&request_path)?;
    Ok(())
}

fn queued_phase(kind: UpdateRequestKind) -> UpdatePhase {
    match kind {
        UpdateRequestKind::Check => UpdatePhase::Checking,
        UpdateRequestKind::Install => UpdatePhase::UpdateRequested,
        UpdateRequestKind::Rollback => UpdatePhase::RollbackRequested,
    }
}

fn request_path(config: &UpdateConfig) -> PathBuf {
    config.control_dir.join(UPDATE_REQUEST_FILE)
}

fn status_path(config: &UpdateConfig) -> PathBuf {
    config.control_dir.join(UPDATE_STATUS_FILE)
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> ServerResult<Option<T>> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    Ok(serde_json::from_slice(&bytes).ok())
}

fn remove_file_if_exists(path: &Path) -> ServerResult<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn write_json_atomic<T: serde::Serialize>(path: &Path, value: &T) -> ServerResult<()> {
    let Some(parent) = path.parent() else {
        return Err(ServerError::internal(
            "control path has no parent directory",
        ));
    };
    fs::create_dir_all(parent)?;
    let temporary_path = path.with_extension(format!("{}.tmp", Uuid::new_v4()));
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(&temporary_path, bytes)?;
    fs::rename(&temporary_path, path)?;
    Ok(())
}
