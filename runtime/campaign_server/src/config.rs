use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub database_url: String,
    pub public_base_url: String,
    pub listen_address: String,
    pub dashboard_dist: PathBuf,
    pub app_version: String,
}

impl ServerConfig {
    pub fn from_environment() -> Result<Self> {
        let repo_root = env::current_dir().context("failed to read current directory")?;
        let database_url = env::var("ABE_DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:runtime/data/ad_buy_engine.sqlite3".to_string());
        let public_base_url =
            env::var("ABE_PUBLIC_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8088".to_string());
        let listen_address =
            env::var("ABE_LISTEN_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8088".to_string());
        let dashboard_dist = env::var("ABE_DASHBOARD_DIST")
            .map(PathBuf::from)
            .unwrap_or_else(|_| repo_root.join("feats/admin_dashboard/dist"));

        Ok(Self {
            database_url,
            public_base_url: public_base_url.trim_end_matches('/').to_string(),
            listen_address,
            dashboard_dist,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}
