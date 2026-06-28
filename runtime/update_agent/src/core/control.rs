use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{
    UPDATE_LOCK_FILE, UPDATE_REQUEST_FILE, UPDATE_STATUS_FILE, UpdateControlRequest, UpdateLock,
    UpdateStatusResponse,
};
use anyhow::{Context, Result, bail};
use uuid::Uuid;

use crate::core::time::now_millis;

const STALE_LOCK_MILLIS: i64 = 30 * 60 * 1000;

#[derive(Clone, Debug)]
pub struct ControlDirectory {
    path: PathBuf,
}

impl ControlDirectory {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn pending_request(&self) -> Result<Option<UpdateControlRequest>> {
        read_json(&self.request_path())
    }

    pub fn read_status(&self) -> Result<Option<UpdateStatusResponse>> {
        read_json(&self.status_path())
    }

    pub fn write_status(&self, status: &UpdateStatusResponse) -> Result<()> {
        write_json_atomic(&self.status_path(), status)
    }

    pub fn acquire_lock(&self, operation_id: &str) -> Result<bool> {
        fs::create_dir_all(&self.path)?;
        let lock_path = self.lock_path();
        if let Some(lock) = read_json::<UpdateLock>(&lock_path)? {
            let age_millis = now_millis()?.saturating_sub(lock.created_at_millis);
            if age_millis <= STALE_LOCK_MILLIS {
                return Ok(false);
            }
            fs::remove_file(&lock_path)?;
        } else {
            remove_if_exists(&lock_path)?;
        }

        let lock = UpdateLock {
            operation_id: operation_id.to_string(),
            created_at_millis: now_millis()?,
        };
        let mut file = match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&lock_path)
        {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        file.write_all(&serde_json::to_vec_pretty(&lock)?)?;
        file.sync_all()?;
        Ok(true)
    }

    pub fn complete_request(&self, operation_id: &str) -> Result<()> {
        let lock_path = self.lock_path();
        if let Some(lock) = read_json::<UpdateLock>(&lock_path)?
            && lock.operation_id != operation_id
        {
            bail!("lock owner changed while processing update request");
        }
        remove_if_exists(&self.request_path())?;
        remove_if_exists(&lock_path)?;
        Ok(())
    }

    fn request_path(&self) -> PathBuf {
        self.path.join(UPDATE_REQUEST_FILE)
    }

    fn status_path(&self) -> PathBuf {
        self.path.join(UPDATE_STATUS_FILE)
    }

    fn lock_path(&self) -> PathBuf {
        self.path.join(UPDATE_LOCK_FILE)
    }
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Option<T>> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", path.display()));
        }
    };
    Ok(serde_json::from_slice(&bytes).ok())
}

fn write_json_atomic<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let parent = path
        .parent()
        .with_context(|| format!("control path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent)?;
    let temporary_path = path.with_extension(format!("{}.tmp", Uuid::new_v4()));
    fs::write(&temporary_path, serde_json::to_vec_pretty(value)?)
        .with_context(|| format!("failed to write {}", temporary_path.display()))?;
    fs::rename(&temporary_path, path)
        .with_context(|| format!("failed to replace {}", path.display()))?;
    Ok(())
}

fn remove_if_exists(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}
