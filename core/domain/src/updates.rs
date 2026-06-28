use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::FieldError;

pub const UPDATE_REQUEST_FILE: &str = "request.json";
pub const UPDATE_STATUS_FILE: &str = "status.json";
pub const UPDATE_LOCK_FILE: &str = "lock.json";
pub const CURRENT_RELEASE_SCHEMA: i64 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSlot {
    Blue,
    Green,
}

impl UpdateSlot {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Green => "green",
        }
    }

    pub const fn other(self) -> Self {
        match self {
            Self::Blue => Self::Green,
            Self::Green => Self::Blue,
        }
    }

    pub fn from_env(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "blue" => Some(Self::Blue),
            "green" => Some(Self::Green),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePhase {
    Disabled,
    Idle,
    Checking,
    UpdateRequested,
    Downloading,
    Verifying,
    Staging,
    StartingCandidate,
    CandidateHealthCheck,
    SwitchingTraffic,
    PublicHealthCheck,
    DrainingOldSlot,
    Succeeded,
    Failed,
    RollbackRequested,
    RollingBack,
    RolledBack,
    RollbackFailed,
}

impl UpdatePhase {
    pub const fn is_running(self) -> bool {
        matches!(
            self,
            Self::Checking
                | Self::UpdateRequested
                | Self::Downloading
                | Self::Verifying
                | Self::Staging
                | Self::StartingCandidate
                | Self::CandidateHealthCheck
                | Self::SwitchingTraffic
                | Self::PublicHealthCheck
                | Self::DrainingOldSlot
                | Self::RollbackRequested
                | Self::RollingBack
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateRequestKind {
    Check,
    Install,
    Rollback,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateStatusResponse {
    pub enabled: bool,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub active_slot: Option<UpdateSlot>,
    pub phase: UpdatePhase,
    pub last_result: Option<UpdateResult>,
    pub rollback: RollbackEligibility,
    pub message: Option<String>,
}

impl UpdateStatusResponse {
    pub fn disabled(current_version: String, active_slot: Option<UpdateSlot>) -> Self {
        Self {
            enabled: false,
            current_version,
            latest_version: None,
            active_slot,
            phase: UpdatePhase::Disabled,
            last_result: None,
            rollback: RollbackEligibility::blocked("Updates are disabled"),
            message: Some("Updates are disabled".to_string()),
        }
    }

    pub fn idle(current_version: String, active_slot: Option<UpdateSlot>) -> Self {
        Self {
            enabled: true,
            current_version,
            latest_version: None,
            active_slot,
            phase: UpdatePhase::Idle,
            last_result: None,
            rollback: RollbackEligibility::blocked("No rollback target has been recorded"),
            message: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateResult {
    pub phase: UpdatePhase,
    pub success: bool,
    pub version: Option<String>,
    pub message: String,
    pub completed_at_millis: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackEligibility {
    pub eligible: bool,
    pub target_version: Option<String>,
    pub reason: Option<String>,
}

impl RollbackEligibility {
    pub fn allowed(target_version: impl Into<String>) -> Self {
        Self {
            eligible: true,
            target_version: Some(target_version.into()),
            reason: None,
        }
    }

    pub fn blocked(reason: impl Into<String>) -> Self {
        Self {
            eligible: false,
            target_version: None,
            reason: Some(reason.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateCheckRequest {
    pub confirmation: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateStartRequest {
    pub current_password: String,
    pub confirmation: String,
    pub requested_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRollbackRequest {
    pub current_password: String,
    pub confirmation: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateControlRequest {
    pub id: String,
    pub kind: UpdateRequestKind,
    pub requested_version: Option<String>,
    pub requested_at_millis: i64,
    pub requested_by: String,
    pub repo: String,
    pub target_triple: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateLock {
    pub operation_id: String,
    pub created_at_millis: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub version: String,
    pub git_sha: String,
    pub target_triple: String,
    pub schema: ReleaseSchemaCompatibility,
    pub rollback: ReleaseRollbackPolicy,
    pub binary_path: String,
    pub dashboard_path: String,
    pub artifacts: Vec<ReleaseArtifact>,
}

impl ReleaseManifest {
    pub fn validate(
        &self,
        expected_target_triple: &str,
        actual_sha256_by_path: &BTreeMap<String, String>,
    ) -> Result<(), Vec<FieldError>> {
        let mut errors = Vec::new();
        if self.version.trim().is_empty() {
            errors.push(field_error("version", "Version is required"));
        }
        if self.git_sha.trim().is_empty() {
            errors.push(field_error("git_sha", "Git SHA is required"));
        }
        if self.target_triple != expected_target_triple {
            errors.push(field_error(
                "target_triple",
                format!(
                    "Expected target triple {expected_target_triple}, got {}",
                    self.target_triple
                ),
            ));
        }
        if self.schema.manifest_version != CURRENT_RELEASE_SCHEMA {
            errors.push(field_error(
                "schema.manifest_version",
                format!(
                    "Unsupported release manifest schema {}",
                    self.schema.manifest_version
                ),
            ));
        }
        if self.binary_path.trim().is_empty() {
            errors.push(field_error("binary_path", "Binary path is required"));
        }
        if self.dashboard_path.trim().is_empty() {
            errors.push(field_error("dashboard_path", "Dashboard path is required"));
        }
        if self.artifacts.is_empty() {
            errors.push(field_error(
                "artifacts",
                "At least one artifact is required",
            ));
        }

        let artifact_sha256_by_path: BTreeMap<&str, &str> = self
            .artifacts
            .iter()
            .map(|artifact| (artifact.path.as_str(), artifact.sha256.as_str()))
            .collect();
        for required_path in [&self.binary_path, &self.dashboard_path] {
            if !artifact_sha256_by_path.contains_key(required_path.as_str()) {
                errors.push(field_error(
                    "artifacts",
                    format!("Missing checksum metadata for {required_path}"),
                ));
            }
        }

        for artifact in &self.artifacts {
            match actual_sha256_by_path.get(&artifact.path) {
                Some(actual) if actual.eq_ignore_ascii_case(&artifact.sha256) => {}
                Some(actual) => errors.push(field_error(
                    "artifacts",
                    format!(
                        "Digest mismatch for {}: expected {}, got {actual}",
                        artifact.path, artifact.sha256
                    ),
                )),
                None => errors.push(field_error(
                    "artifacts",
                    format!("Missing release file {}", artifact.path),
                )),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn rollback_allowed_to(&self, current_schema_version: i64) -> bool {
        self.rollback.schema_rollback_safe
            && self.schema.minimum_supported_schema <= current_schema_version
            && self.schema.maximum_supported_schema >= current_schema_version
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseSchemaCompatibility {
    pub manifest_version: i64,
    pub minimum_supported_schema: i64,
    pub maximum_supported_schema: i64,
    pub migrations_backward_compatible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseRollbackPolicy {
    pub schema_rollback_safe: bool,
    pub requires_database_restore: bool,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseArtifact {
    pub path: String,
    pub sha256: String,
}

fn field_error(field: impl Into<String>, message: impl Into<String>) -> FieldError {
    FieldError {
        field: field.into(),
        message: message.into(),
    }
}
