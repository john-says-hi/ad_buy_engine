use std::fs;
use std::path::Path;
use std::process::Command;

use ad_buy_engine_domain::{UpdateControlRequest, UpdateSlot};
use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use flate2::read::GzDecoder;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use tar::Archive;
use tokio::time::sleep;

use crate::core::agent::{
    HealthProbe, ProxySwitcher, ReleaseInstaller, ReleaseSource, SlotSupervisor,
};
use crate::core::config::UpdateAgentConfig;
use crate::core::package::{InstalledRelease, ReleasePackage};

pub struct GithubReleaseSource {
    client: reqwest::Client,
}

impl GithubReleaseSource {
    pub fn new(token: Option<&str>) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("ad-buy-engine-update-agent"),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        if let Some(token) = token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {token}"))
                    .context("GitHub token could not be used as an HTTP header")?,
            );
        }
        Ok(Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .context("failed to build GitHub HTTP client")?,
        })
    }

    async fn release(&self, request: &UpdateControlRequest) -> Result<GithubRelease> {
        let url = if let Some(version) = request.requested_version.as_ref() {
            format!(
                "https://api.github.com/repos/{}/releases/tags/{}",
                request.repo, version
            )
        } else {
            format!(
                "https://api.github.com/repos/{}/releases/latest",
                request.repo
            )
        };
        let response = self.client.get(url).send().await?.error_for_status()?;
        let text = response.text().await?;
        serde_json::from_str(&text).context("GitHub release response was not valid JSON")
    }
}

#[async_trait]
impl ReleaseSource for GithubReleaseSource {
    async fn latest_version(&self, request: &UpdateControlRequest) -> Result<Option<String>> {
        Ok(Some(self.release(request).await?.tag_name))
    }

    async fn fetch(&self, request: &UpdateControlRequest) -> Result<ReleasePackage> {
        let release = self.release(request).await?;
        let asset = release
            .assets
            .iter()
            .find(|asset| {
                asset.name.contains(&request.target_triple) && asset.name.ends_with(".tar.gz")
            })
            .or_else(|| {
                release
                    .assets
                    .iter()
                    .find(|asset| asset.name.ends_with(".tar.gz"))
            })
            .with_context(|| {
                format!(
                    "release {} does not contain a .tar.gz asset for {}",
                    release.tag_name, request.target_triple
                )
            })?;
        let bytes = self
            .client
            .get(&asset.browser_download_url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let unpack_root = std::env::temp_dir().join(format!(
            "abe-release-{}-{}",
            release.tag_name,
            uuid::Uuid::new_v4()
        ));
        fs::create_dir_all(&unpack_root)?;
        let decoder = GzDecoder::new(bytes.as_ref());
        let mut archive = Archive::new(decoder);
        archive.unpack(&unpack_root)?;
        ReleasePackage::from_unpacked_root(unpack_root)
    }
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

pub struct FilesystemReleaseInstaller;

#[async_trait]
impl ReleaseInstaller for FilesystemReleaseInstaller {
    async fn install(
        &self,
        package: ReleasePackage,
        slot: UpdateSlot,
        config: &UpdateAgentConfig,
    ) -> Result<InstalledRelease> {
        let version = package.manifest.version.clone();
        let release_path = config.release_root.join(&version);
        if release_path.exists() {
            bail!(
                "release directory already exists: {}",
                release_path.display()
            );
        }
        copy_dir_all(&package.root, &release_path)?;
        let slot_link = config.release_root.join(slot.as_str());
        replace_symlink(&slot_link, &release_path)?;
        Ok(InstalledRelease {
            version,
            path: release_path,
            slot,
            manifest: package.manifest,
        })
    }
}

pub struct SystemdSlotSupervisor;

#[async_trait]
impl SlotSupervisor for SystemdSlotSupervisor {
    async fn start_slot(&self, slot: UpdateSlot, _release: &InstalledRelease) -> Result<()> {
        run_command(
            "systemctl",
            &["start", &format!("ad-buy-engine@{}.service", slot.as_str())],
        )
    }

    async fn drain_and_stop_slot(&self, slot: UpdateSlot, drain_seconds: u64) -> Result<()> {
        if drain_seconds > 0 {
            sleep(std::time::Duration::from_secs(drain_seconds)).await;
        }
        run_command(
            "systemctl",
            &["stop", &format!("ad-buy-engine@{}.service", slot.as_str())],
        )
    }
}

pub struct NginxProxySwitcher;

#[async_trait]
impl ProxySwitcher for NginxProxySwitcher {
    async fn switch_to_slot(
        &self,
        slot: UpdateSlot,
        port: u16,
        config: &UpdateAgentConfig,
    ) -> Result<()> {
        let template = fs::read_to_string(&config.nginx_template_path).with_context(|| {
            format!(
                "failed to read Nginx template {}",
                config.nginx_template_path.display()
            )
        })?;
        let rendered = template
            .replace("{slot}", slot.as_str())
            .replace("{port}", &port.to_string())
            .replace("{blue_port}", &config.slots.blue.to_string())
            .replace("{green_port}", &config.slots.green.to_string());
        let temporary_path = config
            .active_upstream_path
            .with_extension(format!("{}.tmp", uuid::Uuid::new_v4()));
        fs::write(&temporary_path, rendered)?;
        fs::rename(&temporary_path, &config.active_upstream_path)?;
        run_command("nginx", &["-t"])
    }

    async fn reload(&self) -> Result<()> {
        run_command("systemctl", &["reload", "nginx"])
    }
}

pub struct HttpHealthProbe {
    client: reqwest::Client,
}

impl HttpHealthProbe {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for HttpHealthProbe {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HealthProbe for HttpHealthProbe {
    async fn check_local(&self, _slot: UpdateSlot, port: u16) -> Result<()> {
        let url = format!("http://127.0.0.1:{port}/api/health");
        self.check_url(&url).await
    }

    async fn check_public(&self, url: &str) -> Result<()> {
        self.check_url(url).await
    }
}

impl HttpHealthProbe {
    async fn check_url(&self, url: &str) -> Result<()> {
        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()
            .with_context(|| format!("health check failed for {url}"))?;
        Ok(())
    }
}

fn copy_dir_all(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_all(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn replace_symlink(link: &Path, target: &Path) -> Result<()> {
    let temporary_link = link.with_extension(format!("{}.tmp", uuid::Uuid::new_v4()));
    std::os::unix::fs::symlink(target, &temporary_link)?;
    if link.exists() {
        fs::remove_file(link)?;
    }
    fs::rename(temporary_link, link)?;
    Ok(())
}

#[cfg(not(unix))]
fn replace_symlink(link: &Path, target: &Path) -> Result<()> {
    if link.exists() {
        fs::remove_dir_all(link)?;
    }
    copy_dir_all(target, link)
}

fn run_command(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to run command: {} {}", program, args.join(" ")))?;
    if status.success() {
        Ok(())
    } else {
        bail!("command failed: {} {}", program, args.join(" "))
    }
}
