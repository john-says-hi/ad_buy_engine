use std::env;
use std::path::PathBuf;
use std::time::Duration;

use ad_buy_engine_domain::UpdateSlot;
use anyhow::{Context, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateAgentConfig {
    pub control_dir: PathBuf,
    pub release_root: PathBuf,
    pub repo: String,
    pub target_triple: String,
    pub active_upstream_path: PathBuf,
    pub nginx_template_path: PathBuf,
    pub public_health_url: String,
    pub github_token: Option<String>,
    pub drain_seconds: u64,
    pub poll_seconds: u64,
    pub releases_to_keep: usize,
    pub current_schema_version: i64,
    pub slots: SlotPorts,
    pub default_active_slot: UpdateSlot,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotPorts {
    pub blue: u16,
    pub green: u16,
}

impl SlotPorts {
    pub const fn port(&self, slot: UpdateSlot) -> u16 {
        match slot {
            UpdateSlot::Blue => self.blue,
            UpdateSlot::Green => self.green,
        }
    }
}

impl UpdateAgentConfig {
    pub fn from_environment() -> Result<Self> {
        let current_dir = env::current_dir().context("failed to read current directory")?;
        Ok(Self {
            control_dir: env_path("ABE_UPDATE_CONTROL_DIR")
                .unwrap_or_else(|| current_dir.join("runtime/update_control")),
            release_root: env_path("ABE_RELEASE_ROOT")
                .unwrap_or_else(|| PathBuf::from("/opt/ad_buy_engine/releases")),
            repo: env::var("ABE_UPDATE_REPO")
                .unwrap_or_else(|_| "john-says-hi/ad_buy_engine".to_string()),
            target_triple: env::var("ABE_UPDATE_TARGET_TRIPLE")
                .unwrap_or_else(|_| "x86_64-unknown-linux-gnu".to_string()),
            active_upstream_path: env_path("ABE_NGINX_ACTIVE_UPSTREAM")
                .unwrap_or_else(|| PathBuf::from("/etc/nginx/conf.d/ad-buy-engine-upstream.conf")),
            nginx_template_path: env_path("ABE_NGINX_UPSTREAM_TEMPLATE").unwrap_or_else(|| {
                current_dir.join("infra/nginx/ad-buy-engine-upstream.conf.template")
            }),
            public_health_url: env::var("ABE_PUBLIC_HEALTH_URL")
                .unwrap_or_else(|_| "http://127.0.0.1/api/health".to_string()),
            github_token: env::var("ABE_GITHUB_TOKEN")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            drain_seconds: env_u64("ABE_UPDATE_DRAIN_SECONDS", 10),
            poll_seconds: env_u64("ABE_UPDATE_AGENT_POLL_SECONDS", 5),
            releases_to_keep: usize::try_from(env_u64("ABE_RELEASES_TO_KEEP", 5))
                .context("ABE_RELEASES_TO_KEEP does not fit in usize")?,
            current_schema_version: env_i64("ABE_CURRENT_SCHEMA_VERSION", 3)?,
            slots: SlotPorts {
                blue: env_u16("ABE_BLUE_PORT", 18081)?,
                green: env_u16("ABE_GREEN_PORT", 18082)?,
            },
            default_active_slot: env::var("ABE_ACTIVE_SLOT")
                .ok()
                .and_then(|value| UpdateSlot::from_env(&value))
                .unwrap_or(UpdateSlot::Blue),
        })
    }

    pub const fn drain_duration(&self) -> Duration {
        Duration::from_secs(self.drain_seconds)
    }
}

fn env_path(key: &str) -> Option<PathBuf> {
    env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
}

fn env_u64(key: &str, default: u64) -> u64 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default)
}

fn env_u16(key: &str, default: u16) -> Result<u16> {
    env::var(key)
        .ok()
        .map(|value| {
            value
                .parse::<u16>()
                .with_context(|| format!("{key} must be a TCP port"))
        })
        .unwrap_or(Ok(default))
}

fn env_i64(key: &str, default: i64) -> Result<i64> {
    env::var(key)
        .ok()
        .map(|value| {
            value
                .parse::<i64>()
                .with_context(|| format!("{key} must be an integer"))
        })
        .unwrap_or(Ok(default))
}
