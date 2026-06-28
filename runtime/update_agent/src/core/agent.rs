use ad_buy_engine_domain::{
    RollbackEligibility, UpdateControlRequest, UpdatePhase, UpdateRequestKind, UpdateResult,
    UpdateSlot, UpdateStatusResponse,
};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;

use crate::core::config::UpdateAgentConfig;
use crate::core::control::ControlDirectory;
use crate::core::package::{InstalledRelease, ReleasePackage};
use crate::core::time::now_millis;

#[async_trait]
pub trait ReleaseSource: Send + Sync {
    async fn latest_version(&self, request: &UpdateControlRequest) -> Result<Option<String>>;
    async fn fetch(&self, request: &UpdateControlRequest) -> Result<ReleasePackage>;
}

#[async_trait]
pub trait ReleaseInstaller: Send + Sync {
    async fn install(
        &self,
        package: ReleasePackage,
        slot: UpdateSlot,
        config: &UpdateAgentConfig,
    ) -> Result<InstalledRelease>;
}

#[async_trait]
pub trait SlotSupervisor: Send + Sync {
    async fn start_slot(&self, slot: UpdateSlot, release: &InstalledRelease) -> Result<()>;
    async fn drain_and_stop_slot(&self, slot: UpdateSlot, drain_seconds: u64) -> Result<()>;
}

#[async_trait]
pub trait ProxySwitcher: Send + Sync {
    async fn switch_to_slot(
        &self,
        slot: UpdateSlot,
        port: u16,
        config: &UpdateAgentConfig,
    ) -> Result<()>;
    async fn reload(&self) -> Result<()>;
}

#[async_trait]
pub trait HealthProbe: Send + Sync {
    async fn check_local(&self, slot: UpdateSlot, port: u16) -> Result<()>;
    async fn check_public(&self, url: &str) -> Result<()>;
}

pub struct UpdateAgent {
    config: UpdateAgentConfig,
    control: ControlDirectory,
    clients: UpdateAgentClients,
}

pub struct UpdateAgentClients {
    pub release_source: Box<dyn ReleaseSource>,
    pub installer: Box<dyn ReleaseInstaller>,
    pub supervisor: Box<dyn SlotSupervisor>,
    pub proxy: Box<dyn ProxySwitcher>,
    pub health: Box<dyn HealthProbe>,
}

impl UpdateAgent {
    pub fn new(config: UpdateAgentConfig, clients: UpdateAgentClients) -> Self {
        let control = ControlDirectory::new(config.control_dir.clone());
        Self {
            config,
            control,
            clients,
        }
    }

    pub async fn run_once(&self) -> Result<bool> {
        let Some(request) = self.control.pending_request()? else {
            return Ok(false);
        };
        if !self.control.acquire_lock(&request.id)? {
            return Ok(false);
        }

        let result = match request.kind {
            UpdateRequestKind::Check => self.check_latest(&request).await,
            UpdateRequestKind::Install => self.install_release(&request).await,
            UpdateRequestKind::Rollback => self.rollback(&request).await,
        };

        if let Err(error) = &result {
            let failure_phase = match request.kind {
                UpdateRequestKind::Rollback => UpdatePhase::RollbackFailed,
                _ => UpdatePhase::Failed,
            };
            self.record_failure(failure_phase, None, error.to_string())?;
        }
        self.control.complete_request(&request.id)?;
        result.map(|()| true)
    }

    async fn check_latest(&self, request: &UpdateControlRequest) -> Result<()> {
        self.record_phase(UpdatePhase::Checking, None, Some("Checking GitHub release"))?;
        let latest_version = self.clients.release_source.latest_version(request).await?;
        let mut status = self.current_status()?;
        status.phase = UpdatePhase::Succeeded;
        status.latest_version = latest_version;
        status.message = Some("Release check completed".to_string());
        status.last_result = Some(UpdateResult {
            phase: UpdatePhase::Succeeded,
            success: true,
            version: status.latest_version.clone(),
            message: "Release check completed".to_string(),
            completed_at_millis: now_millis()?,
        });
        self.control.write_status(&status)
    }

    async fn install_release(&self, request: &UpdateControlRequest) -> Result<()> {
        let previous_status = self.current_status()?;
        let active_slot = previous_status
            .active_slot
            .unwrap_or(self.config.default_active_slot);
        let candidate_slot = active_slot.other();

        self.record_phase(UpdatePhase::Downloading, None, Some("Downloading release"))?;
        let package = self.clients.release_source.fetch(request).await?;
        let candidate_version = package.manifest.version.clone();

        self.record_phase(
            UpdatePhase::Verifying,
            Some(candidate_version.clone()),
            Some("Verifying release manifest"),
        )?;
        package
            .manifest
            .validate(&request.target_triple, &package.actual_sha256_by_path)
            .map_err(|errors| anyhow!("release manifest validation failed: {errors:?}"))?;

        self.record_phase(
            UpdatePhase::Staging,
            Some(candidate_version.clone()),
            Some("Staging release"),
        )?;
        let installed = self
            .clients
            .installer
            .install(package, candidate_slot, &self.config)
            .await?;

        self.record_phase(
            UpdatePhase::StartingCandidate,
            Some(candidate_version.clone()),
            Some("Starting inactive slot"),
        )?;
        self.clients
            .supervisor
            .start_slot(candidate_slot, &installed)
            .await?;

        self.record_phase(
            UpdatePhase::CandidateHealthCheck,
            Some(candidate_version.clone()),
            Some("Checking inactive slot health"),
        )?;
        self.clients
            .health
            .check_local(candidate_slot, self.config.slots.port(candidate_slot))
            .await?;

        self.record_phase(
            UpdatePhase::SwitchingTraffic,
            Some(candidate_version.clone()),
            Some("Switching Nginx upstream"),
        )?;
        self.clients
            .proxy
            .switch_to_slot(
                candidate_slot,
                self.config.slots.port(candidate_slot),
                &self.config,
            )
            .await?;
        if let Err(error) = self.clients.proxy.reload().await {
            self.clients
                .proxy
                .switch_to_slot(
                    active_slot,
                    self.config.slots.port(active_slot),
                    &self.config,
                )
                .await
                .context("failed to restore previous Nginx upstream after reload failure")?;
            return Err(error).context("Nginx reload failed after candidate upstream switch");
        }

        self.record_phase(
            UpdatePhase::PublicHealthCheck,
            Some(candidate_version.clone()),
            Some("Checking public health endpoint"),
        )?;
        if let Err(error) = self
            .clients
            .health
            .check_public(&self.config.public_health_url)
            .await
        {
            self.rollback_cutover(active_slot).await?;
            return Err(error).context("post-cutover public health check failed");
        }

        self.record_phase(
            UpdatePhase::DrainingOldSlot,
            Some(candidate_version.clone()),
            Some("Draining old slot"),
        )?;
        self.clients
            .supervisor
            .drain_and_stop_slot(active_slot, self.config.drain_seconds)
            .await?;

        let rollback = if installed
            .manifest
            .rollback_allowed_to(self.config.current_schema_version)
        {
            RollbackEligibility::allowed(previous_status.current_version)
        } else {
            RollbackEligibility::blocked("Release manifest marks schema rollback as unsafe")
        };
        self.record_success(
            UpdatePhase::Succeeded,
            candidate_slot,
            candidate_version,
            "Release installed",
            rollback,
        )
    }

    async fn rollback(&self, _request: &UpdateControlRequest) -> Result<()> {
        let current = self.current_status()?;
        if !current.rollback.eligible {
            let reason = current
                .rollback
                .reason
                .unwrap_or_else(|| "Rollback is not eligible".to_string());
            self.record_failure(UpdatePhase::RollbackFailed, None, reason.clone())?;
            return Err(anyhow!(reason));
        }

        let active_slot = current
            .active_slot
            .unwrap_or(self.config.default_active_slot);
        let rollback_slot = active_slot.other();
        let target_version = current
            .rollback
            .target_version
            .clone()
            .unwrap_or_else(|| "previous".to_string());

        self.record_phase(
            UpdatePhase::RollingBack,
            Some(target_version.clone()),
            Some("Switching back to previous slot"),
        )?;
        self.clients
            .proxy
            .switch_to_slot(
                rollback_slot,
                self.config.slots.port(rollback_slot),
                &self.config,
            )
            .await?;
        self.clients.proxy.reload().await?;
        self.clients
            .health
            .check_public(&self.config.public_health_url)
            .await?;
        self.clients
            .supervisor
            .drain_and_stop_slot(active_slot, self.config.drain_seconds)
            .await?;
        self.record_success(
            UpdatePhase::RolledBack,
            rollback_slot,
            target_version,
            "Rollback completed",
            RollbackEligibility::blocked("No rollback target has been recorded"),
        )
    }

    async fn rollback_cutover(&self, active_slot: UpdateSlot) -> Result<()> {
        self.clients
            .proxy
            .switch_to_slot(
                active_slot,
                self.config.slots.port(active_slot),
                &self.config,
            )
            .await?;
        self.clients.proxy.reload().await
    }

    fn current_status(&self) -> Result<UpdateStatusResponse> {
        Ok(self.control.read_status()?.unwrap_or_else(|| {
            UpdateStatusResponse::idle("unknown".to_string(), Some(self.config.default_active_slot))
        }))
    }

    fn record_phase(
        &self,
        phase: UpdatePhase,
        version: Option<String>,
        message: Option<&str>,
    ) -> Result<()> {
        let mut status = self.current_status()?;
        status.phase = phase;
        status.latest_version = version;
        status.message = message.map(ToOwned::to_owned);
        self.control.write_status(&status)
    }

    fn record_success(
        &self,
        phase: UpdatePhase,
        active_slot: UpdateSlot,
        version: String,
        message: &str,
        rollback: RollbackEligibility,
    ) -> Result<()> {
        let status = UpdateStatusResponse {
            enabled: true,
            current_version: version.clone(),
            latest_version: Some(version.clone()),
            active_slot: Some(active_slot),
            phase,
            last_result: Some(UpdateResult {
                phase,
                success: true,
                version: Some(version),
                message: message.to_string(),
                completed_at_millis: now_millis()?,
            }),
            rollback,
            message: Some(message.to_string()),
        };
        self.control.write_status(&status)
    }

    fn record_failure(
        &self,
        phase: UpdatePhase,
        version: Option<String>,
        message: String,
    ) -> Result<()> {
        let mut status = self.current_status()?;
        status.phase = phase;
        status.message = Some(message.clone());
        status.last_result = Some(UpdateResult {
            phase,
            success: false,
            version,
            message,
            completed_at_millis: now_millis()?,
        });
        self.control.write_status(&status)
    }
}
