use std::env;
use std::path::PathBuf;

use ad_buy_engine_domain::{FAKE_AFFILIATE_DEFAULT_BASE_URL, UpdateSlot};
use anyhow::{Context, Result};
use url::{Host, Url};

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub database_url: String,
    pub tracking_base_url: String,
    pub tracking_base_url_override: Option<String>,
    pub admin_dashboard_base_url: String,
    pub admin_dashboard_base_url_override: Option<String>,
    pub public_base_url: String,
    pub listen_address: String,
    pub dashboard_dist: PathBuf,
    pub app_version: String,
    pub maxmind_account_id: String,
    pub maxmind_license_key: String,
    pub geolite_city_database_path: String,
    pub geolite_country_database_path: String,
    pub geolite_asn_database_path: String,
    pub demo_seed_fake_affiliate_network: bool,
    pub fake_affiliate_network_base_url: String,
    pub updates: UpdateConfig,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateConfig {
    pub enabled: bool,
    pub control_dir: PathBuf,
    pub repo: String,
    pub target_triple: String,
    pub active_slot: Option<UpdateSlot>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseUrlOverrides {
    pub tracking_base_url: Option<String>,
    pub admin_dashboard_base_url: Option<String>,
    pub public_base_url_fallback: String,
}

impl ServerConfig {
    pub fn from_environment() -> Result<Self> {
        let repo_root = env::current_dir().context("failed to read current directory")?;
        let database_url = env::var("ABE_DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:runtime/data/ad_buy_engine.sqlite3".to_string());
        let public_base_url = env_base_url("ABE_PUBLIC_BASE_URL")
            .unwrap_or_else(|| "http://127.0.0.1:8088".to_string());
        let tracking_base_url_override = env_base_url("ABE_TRACKING_BASE_URL");
        let admin_dashboard_base_url_override = env_base_url("ABE_ADMIN_BASE_URL");
        let tracking_base_url = tracking_base_url_override
            .clone()
            .unwrap_or_else(|| public_base_url.clone());
        let admin_dashboard_base_url = admin_dashboard_base_url_override
            .clone()
            .unwrap_or_else(|| public_base_url.clone());
        let listen_address =
            env::var("ABE_LISTEN_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8088".to_string());
        let dashboard_dist = env::var("ABE_DASHBOARD_DIST")
            .map(PathBuf::from)
            .unwrap_or_else(|_| repo_root.join("feats/admin_dashboard/dist"));
        let maxmind_account_id = env::var("ABE_MAXMIND_ACCOUNT_ID").unwrap_or_default();
        let maxmind_license_key = env::var("ABE_MAXMIND_LICENSE_KEY").unwrap_or_default();
        let geolite_city_database_path = env::var("ABE_GEOLITE_CITY_DATABASE_PATH")
            .unwrap_or_else(|_| "runtime/data/GeoLite2-City.mmdb".to_string());
        let geolite_country_database_path = env::var("ABE_GEOLITE_COUNTRY_DATABASE_PATH")
            .unwrap_or_else(|_| "runtime/data/GeoLite2-Country.mmdb".to_string());
        let geolite_asn_database_path = env::var("ABE_GEOLITE_ASN_DATABASE_PATH")
            .unwrap_or_else(|_| "runtime/data/GeoLite2-ASN.mmdb".to_string());
        let demo_seed_fake_affiliate_network = env_bool("ABE_DEMO_SEED_FAKE_AFFILIATE_NETWORK");
        let fake_affiliate_network_base_url = env_base_url("ABE_FAKE_AFFILIATE_NETWORK_BASE_URL")
            .unwrap_or_else(|| FAKE_AFFILIATE_DEFAULT_BASE_URL.to_string());
        if demo_seed_fake_affiliate_network {
            validate_fake_affiliate_network_base_url(&fake_affiliate_network_base_url)?;
        }
        let updates = UpdateConfig {
            enabled: env_bool("ABE_UPDATE_ENABLED"),
            control_dir: env::var("ABE_UPDATE_CONTROL_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| repo_root.join("runtime/update_control")),
            repo: env::var("ABE_UPDATE_REPO")
                .unwrap_or_else(|_| "john-says-hi/ad_buy_engine".to_string()),
            target_triple: env::var("ABE_UPDATE_TARGET_TRIPLE")
                .unwrap_or_else(|_| "x86_64-unknown-linux-gnu".to_string()),
            active_slot: env::var("ABE_ACTIVE_SLOT")
                .ok()
                .and_then(|value| UpdateSlot::from_env(&value)),
        };

        Ok(Self {
            database_url,
            tracking_base_url,
            tracking_base_url_override,
            admin_dashboard_base_url,
            admin_dashboard_base_url_override,
            public_base_url,
            listen_address,
            dashboard_dist,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            maxmind_account_id,
            maxmind_license_key,
            geolite_city_database_path,
            geolite_country_database_path,
            geolite_asn_database_path,
            demo_seed_fake_affiliate_network,
            fake_affiliate_network_base_url,
            updates,
        })
    }

    pub fn base_url_overrides(&self) -> BaseUrlOverrides {
        BaseUrlOverrides {
            tracking_base_url: self.tracking_base_url_override.clone(),
            admin_dashboard_base_url: self.admin_dashboard_base_url_override.clone(),
            public_base_url_fallback: self.public_base_url.clone(),
        }
    }
}

fn env_bool(key: &str) -> bool {
    env::var(key)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn env_base_url(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .and_then(|value| normalize_base_url(&value))
}

fn normalize_base_url(value: &str) -> Option<String> {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub fn validate_fake_affiliate_network_base_url(value: &str) -> Result<()> {
    let url = Url::parse(value).context("ABE_FAKE_AFFILIATE_NETWORK_BASE_URL must be a URL")?;
    if !matches!(url.scheme(), "http" | "https") {
        anyhow::bail!("ABE_FAKE_AFFILIATE_NETWORK_BASE_URL must use http or https");
    }
    if !is_loopback_url(&url) {
        anyhow::bail!("ABE_FAKE_AFFILIATE_NETWORK_BASE_URL must be loopback for demo seeding");
    }
    if url.query().is_some() || url.fragment().is_some() {
        anyhow::bail!(
            "ABE_FAKE_AFFILIATE_NETWORK_BASE_URL must not include a query string or fragment"
        );
    }
    Ok(())
}

fn is_loopback_url(url: &Url) -> bool {
    match url.host() {
        Some(Host::Domain(domain)) => domain.eq_ignore_ascii_case("localhost"),
        Some(Host::Ipv4(ip)) => ip.is_loopback(),
        Some(Host::Ipv6(ip)) => ip.is_loopback(),
        None => false,
    }
}
