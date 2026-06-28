use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

use ad_buy_engine_domain::{
    CURRENT_RELEASE_SCHEMA, ReleaseArtifact, ReleaseManifest, ReleaseRollbackPolicy,
    ReleaseSchemaCompatibility, RollbackEligibility, UPDATE_REQUEST_FILE, UpdateControlRequest,
    UpdatePhase, UpdateRequestKind, UpdateSlot, UpdateStatusResponse,
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use tempfile::{TempDir, tempdir};
use update_agent::core::agent::{
    HealthProbe, ProxySwitcher, ReleaseInstaller, ReleaseSource, SlotSupervisor, UpdateAgent,
    UpdateAgentClients,
};
use update_agent::core::config::{SlotPorts, UpdateAgentConfig};
use update_agent::core::control::ControlDirectory;
use update_agent::core::package::{InstalledRelease, ReleasePackage};

#[tokio::test]
async fn installs_release_with_blue_green_cutover() -> Result<()> {
    let harness = AgentHarness::new(FailurePoint::None, true)?;

    let ran = harness.agent.run_once().await?;
    let status = harness
        .control
        .read_status()?
        .ok_or_else(|| anyhow!("missing status"))?;

    assert!(ran);
    assert_eq!(status.phase, UpdatePhase::Succeeded);
    assert_eq!(status.current_version, "v0.2.0");
    assert_eq!(status.active_slot, Some(UpdateSlot::Green));
    assert!(status.rollback.eligible);
    assert_eq!(harness.events()?, expected_happy_events());
    Ok(())
}

#[tokio::test]
async fn download_failure_records_failed_status() -> Result<()> {
    let harness = AgentHarness::new(FailurePoint::Download, true)?;

    let result = harness.agent.run_once().await;
    let status = harness
        .control
        .read_status()?
        .ok_or_else(|| anyhow!("missing status"))?;

    assert!(result.is_err());
    assert_eq!(status.phase, UpdatePhase::Failed);
    assert!(harness.events()?.is_empty());
    Ok(())
}

#[tokio::test]
async fn checksum_failure_never_starts_candidate_slot() -> Result<()> {
    let harness = AgentHarness::new(FailurePoint::Checksum, true)?;

    let result = harness.agent.run_once().await;
    let status = harness
        .control
        .read_status()?
        .ok_or_else(|| anyhow!("missing status"))?;

    assert!(result.is_err());
    assert_eq!(status.phase, UpdatePhase::Failed);
    assert!(harness.events()?.is_empty());
    Ok(())
}

#[tokio::test]
async fn start_failure_never_switches_traffic() -> Result<()> {
    let harness = AgentHarness::new(FailurePoint::Start, true)?;

    let result = harness.agent.run_once().await;

    assert!(result.is_err());
    assert_eq!(harness.events()?, vec!["install:green", "start:green"]);
    Ok(())
}

#[tokio::test]
async fn candidate_health_failure_never_switches_traffic() -> Result<()> {
    let harness = AgentHarness::new(FailurePoint::CandidateHealth, true)?;

    let result = harness.agent.run_once().await;

    assert!(result.is_err());
    assert_eq!(
        harness.events()?,
        vec!["install:green", "start:green", "local-health:green"]
    );
    Ok(())
}

#[tokio::test]
async fn nginx_reload_failure_records_failure_after_switch_attempt() -> Result<()> {
    let harness = AgentHarness::new(FailurePoint::NginxReload, true)?;

    let result = harness.agent.run_once().await;
    let events = harness.events()?;

    assert!(result.is_err());
    assert!(events.contains(&"switch:green".to_string()));
    assert!(events.contains(&"reload".to_string()));
    assert!(events.contains(&"switch:blue".to_string()));
    Ok(())
}

#[tokio::test]
async fn post_cutover_health_failure_switches_back_to_old_slot() -> Result<()> {
    let harness = AgentHarness::new(FailurePoint::PublicHealth, true)?;

    let result = harness.agent.run_once().await;
    let events = harness.events()?;

    assert!(result.is_err());
    assert!(events.contains(&"switch:green".to_string()));
    assert!(events.contains(&"public-health".to_string()));
    assert!(events.contains(&"switch:blue".to_string()));
    Ok(())
}

#[tokio::test]
async fn rollback_is_blocked_when_schema_policy_is_unsafe() -> Result<()> {
    let harness = AgentHarness::new(FailurePoint::None, false)?;
    harness.write_request(UpdateRequestKind::Rollback)?;
    harness.control.write_status(&UpdateStatusResponse {
        enabled: true,
        current_version: "v0.2.0".to_string(),
        latest_version: Some("v0.2.0".to_string()),
        active_slot: Some(UpdateSlot::Green),
        phase: UpdatePhase::Idle,
        last_result: None,
        rollback: RollbackEligibility::blocked("Release manifest marks schema rollback as unsafe"),
        message: None,
    })?;

    let result = harness.agent.run_once().await;
    let status = harness
        .control
        .read_status()?
        .ok_or_else(|| anyhow!("missing status"))?;

    assert!(result.is_err());
    assert_eq!(status.phase, UpdatePhase::RollbackFailed);
    assert!(harness.events()?.is_empty());
    Ok(())
}

fn expected_happy_events() -> Vec<String> {
    vec![
        "install:green",
        "start:green",
        "local-health:green",
        "switch:green",
        "reload",
        "public-health",
        "drain-stop:blue",
    ]
    .into_iter()
    .map(ToOwned::to_owned)
    .collect()
}

struct AgentHarness {
    agent: UpdateAgent,
    control: ControlDirectory,
    state: Arc<Mutex<FakeState>>,
    tempdir: TempDir,
}

impl AgentHarness {
    fn new(failure: FailurePoint, rollback_safe: bool) -> Result<Self> {
        let tempdir = tempdir()?;
        let control_dir = tempdir.path().join("control");
        let release_root = tempdir.path().join("releases");
        let config = UpdateAgentConfig {
            control_dir: control_dir.clone(),
            release_root,
            repo: "john-says-hi/ad_buy_engine".to_string(),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            active_upstream_path: tempdir.path().join("upstream.conf"),
            nginx_template_path: tempdir.path().join("upstream.template"),
            public_health_url: "https://track.test/api/health".to_string(),
            github_token: None,
            drain_seconds: 0,
            poll_seconds: 1,
            releases_to_keep: 5,
            current_schema_version: 3,
            slots: SlotPorts {
                blue: 18081,
                green: 18082,
            },
            default_active_slot: UpdateSlot::Blue,
        };
        fs::write(&config.nginx_template_path, "server 127.0.0.1:{port};")?;
        let control = ControlDirectory::new(control_dir.clone());
        fs::create_dir_all(&control_dir)?;
        let initial_status =
            UpdateStatusResponse::idle("v0.1.0".to_string(), Some(UpdateSlot::Blue));
        control.write_status(&initial_status)?;
        let state = Arc::new(Mutex::new(FakeState {
            failure,
            rollback_safe,
            events: Vec::new(),
            package_root: tempdir.path().join("package"),
        }));
        let clients = UpdateAgentClients {
            release_source: Box::new(FakeReleaseSource::new(state.clone())),
            installer: Box::new(FakeInstaller::new(state.clone())),
            supervisor: Box::new(FakeSupervisor::new(state.clone())),
            proxy: Box::new(FakeProxy::new(state.clone())),
            health: Box::new(FakeHealth::new(state.clone())),
        };
        let agent = UpdateAgent::new(config, clients);
        let harness = Self {
            agent,
            control,
            state,
            tempdir,
        };
        harness.write_request(UpdateRequestKind::Install)?;
        Ok(harness)
    }

    fn write_request(&self, kind: UpdateRequestKind) -> Result<()> {
        let request = UpdateControlRequest {
            id: uuid::Uuid::new_v4().to_string(),
            kind,
            requested_version: Some("v0.2.0".to_string()),
            requested_at_millis: 1,
            requested_by: "operator".to_string(),
            repo: "john-says-hi/ad_buy_engine".to_string(),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
        };
        fs::write(
            self.tempdir
                .path()
                .join("control")
                .join(UPDATE_REQUEST_FILE),
            serde_json::to_vec(&request)?,
        )?;
        Ok(())
    }

    fn events(&self) -> Result<Vec<String>> {
        Ok(lock_state(&self.state)?.events.clone())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FailurePoint {
    None,
    Download,
    Checksum,
    Start,
    CandidateHealth,
    NginxReload,
    PublicHealth,
}

struct FakeState {
    failure: FailurePoint,
    rollback_safe: bool,
    events: Vec<String>,
    package_root: PathBuf,
}

#[derive(Clone)]
struct FakeReleaseSource {
    state: Arc<Mutex<FakeState>>,
}

impl FakeReleaseSource {
    fn new(state: Arc<Mutex<FakeState>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl ReleaseSource for FakeReleaseSource {
    async fn latest_version(&self, _request: &UpdateControlRequest) -> Result<Option<String>> {
        Ok(Some("v0.2.0".to_string()))
    }

    async fn fetch(&self, _request: &UpdateControlRequest) -> Result<ReleasePackage> {
        let state = lock_state(&self.state)?;
        if state.failure == FailurePoint::Download {
            return Err(anyhow!("download failed"));
        }
        fake_package(
            state.package_root.clone(),
            state.rollback_safe,
            state.failure == FailurePoint::Checksum,
        )
    }
}

#[derive(Clone)]
struct FakeInstaller {
    state: Arc<Mutex<FakeState>>,
}

impl FakeInstaller {
    fn new(state: Arc<Mutex<FakeState>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl ReleaseInstaller for FakeInstaller {
    async fn install(
        &self,
        package: ReleasePackage,
        slot: UpdateSlot,
        config: &UpdateAgentConfig,
    ) -> Result<InstalledRelease> {
        lock_state(&self.state)?
            .events
            .push(format!("install:{}", slot.as_str()));
        Ok(InstalledRelease {
            version: package.manifest.version.clone(),
            path: config.release_root.join(&package.manifest.version),
            slot,
            manifest: package.manifest,
        })
    }
}

#[derive(Clone)]
struct FakeSupervisor {
    state: Arc<Mutex<FakeState>>,
}

impl FakeSupervisor {
    fn new(state: Arc<Mutex<FakeState>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl SlotSupervisor for FakeSupervisor {
    async fn start_slot(&self, slot: UpdateSlot, _release: &InstalledRelease) -> Result<()> {
        let mut state = lock_state(&self.state)?;
        state.events.push(format!("start:{}", slot.as_str()));
        if state.failure == FailurePoint::Start {
            return Err(anyhow!("start failed"));
        }
        Ok(())
    }

    async fn drain_and_stop_slot(&self, slot: UpdateSlot, _drain_seconds: u64) -> Result<()> {
        lock_state(&self.state)?
            .events
            .push(format!("drain-stop:{}", slot.as_str()));
        Ok(())
    }
}

#[derive(Clone)]
struct FakeProxy {
    state: Arc<Mutex<FakeState>>,
}

impl FakeProxy {
    fn new(state: Arc<Mutex<FakeState>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl ProxySwitcher for FakeProxy {
    async fn switch_to_slot(
        &self,
        slot: UpdateSlot,
        _port: u16,
        _config: &UpdateAgentConfig,
    ) -> Result<()> {
        lock_state(&self.state)?
            .events
            .push(format!("switch:{}", slot.as_str()));
        Ok(())
    }

    async fn reload(&self) -> Result<()> {
        let mut state = lock_state(&self.state)?;
        state.events.push("reload".to_string());
        if state.failure == FailurePoint::NginxReload {
            return Err(anyhow!("nginx reload failed"));
        }
        Ok(())
    }
}

#[derive(Clone)]
struct FakeHealth {
    state: Arc<Mutex<FakeState>>,
}

impl FakeHealth {
    fn new(state: Arc<Mutex<FakeState>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl HealthProbe for FakeHealth {
    async fn check_local(&self, slot: UpdateSlot, _port: u16) -> Result<()> {
        let mut state = lock_state(&self.state)?;
        state.events.push(format!("local-health:{}", slot.as_str()));
        if state.failure == FailurePoint::CandidateHealth {
            return Err(anyhow!("candidate health failed"));
        }
        Ok(())
    }

    async fn check_public(&self, _url: &str) -> Result<()> {
        let mut state = lock_state(&self.state)?;
        state.events.push("public-health".to_string());
        if state.failure == FailurePoint::PublicHealth {
            return Err(anyhow!("public health failed"));
        }
        Ok(())
    }
}

fn fake_package(root: PathBuf, rollback_safe: bool, bad_checksum: bool) -> Result<ReleasePackage> {
    fs::create_dir_all(root.join("dist"))?;
    fs::write(root.join("campaign_server"), "binary")?;
    fs::write(root.join("dist/index.html"), "dashboard")?;
    let manifest = ReleaseManifest {
        version: "v0.2.0".to_string(),
        git_sha: "0123456789abcdef".to_string(),
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        schema: ReleaseSchemaCompatibility {
            manifest_version: CURRENT_RELEASE_SCHEMA,
            minimum_supported_schema: 1,
            maximum_supported_schema: 3,
            migrations_backward_compatible: true,
        },
        rollback: ReleaseRollbackPolicy {
            schema_rollback_safe: rollback_safe,
            requires_database_restore: !rollback_safe,
            notes: "test manifest".to_string(),
        },
        binary_path: "campaign_server".to_string(),
        dashboard_path: "dist/index.html".to_string(),
        artifacts: vec![
            ReleaseArtifact {
                path: "campaign_server".to_string(),
                sha256: "binary-sha".to_string(),
            },
            ReleaseArtifact {
                path: "dist/index.html".to_string(),
                sha256: "dashboard-sha".to_string(),
            },
        ],
    };
    let dashboard_sha = if bad_checksum {
        "wrong-sha"
    } else {
        "dashboard-sha"
    };
    let checksums = BTreeMap::from([
        ("campaign_server".to_string(), "binary-sha".to_string()),
        ("dist/index.html".to_string(), dashboard_sha.to_string()),
    ]);
    Ok(ReleasePackage::from_manifest_for_tests(
        root, manifest, checksums,
    ))
}

fn lock_state(state: &Arc<Mutex<FakeState>>) -> Result<MutexGuard<'_, FakeState>> {
    state
        .lock()
        .map_err(|error| anyhow!("fake state lock was poisoned: {error}"))
}
