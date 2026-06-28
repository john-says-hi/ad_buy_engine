use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};

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
