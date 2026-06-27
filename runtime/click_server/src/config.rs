use anyhow::{anyhow, Context};
use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ClickServerConfig {
    pub bind_addr: String,
    pub database_url: String,
    pub setup_secret: String,
    pub admin_dist_dir: PathBuf,
    pub cookie_secure: bool,
    pub session_ttl_seconds: i64,
    pub version: String,
}

impl ClickServerConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let setup_secret = required_env("ABE_SETUP_SECRET")?;

        Ok(Self {
            bind_addr: env_or("ABE_BIND_ADDR", "0.0.0.0:8080"),
            database_url: env_or("ABE_DATABASE_URL", "sqlite://./click_server.sqlite3"),
            setup_secret,
            admin_dist_dir: PathBuf::from(env_or("ABE_ADMIN_DIST_DIR", "./runtime/admin_web/dist")),
            cookie_secure: env_bool_or("ABE_COOKIE_SECURE", false)?,
            session_ttl_seconds: env_i64_or("ABE_SESSION_TTL_SECONDS", 604_800)?,
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}

fn required_env(key: &str) -> anyhow::Result<String> {
    let value = env::var(key).with_context(|| format!("{key} must be set"))?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{key} must not be empty"));
    }
    Ok(trimmed.to_string())
}

fn env_or(key: &str, fallback: &str) -> String {
    env::var(key).unwrap_or_else(|_| fallback.to_string())
}

fn env_bool_or(key: &str, fallback: bool) -> anyhow::Result<bool> {
    match env::var(key) {
        Ok(value) => value
            .parse::<bool>()
            .with_context(|| format!("{key} must be true or false")),
        Err(_) => Ok(fallback),
    }
}

fn env_i64_or(key: &str, fallback: i64) -> anyhow::Result<i64> {
    match env::var(key) {
        Ok(value) => value
            .parse::<i64>()
            .with_context(|| format!("{key} must be an integer")),
        Err(_) => Ok(fallback),
    }
}
