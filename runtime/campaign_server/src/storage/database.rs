use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::{Argon2, PasswordHash};
use rand_core::OsRng;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Executor, SqlitePool};

use crate::config::ServerConfig;
use crate::error::{ServerError, ServerResult};
use crate::time::now_millis;

const INIT_MIGRATION: &str = include_str!("../../migrations/0001_init.sql");
const CURRENT_SCHEMA_VERSION: i64 = 1;

pub async fn connect_database(config: &ServerConfig) -> ServerResult<SqlitePool> {
    create_parent_directory(&config.database_url)?;
    let options = SqliteConnectOptions::from_str(&config.database_url)
        .map_err(|error| ServerError::internal(format!("invalid database URL: {error}")))?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5));
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    run_migrations(&pool).await?;
    seed_operator_credentials(&pool).await?;
    seed_app_settings(&pool, config).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> ServerResult<()> {
    let mut connection = pool.acquire().await?;
    connection.execute("PRAGMA foreign_keys = ON").await?;
    connection.execute("PRAGMA journal_mode = WAL").await?;
    connection.execute("PRAGMA busy_timeout = 5000").await?;

    for statement in INIT_MIGRATION.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            connection.execute(trimmed).await?;
        }
    }
    sqlx::query(
        "INSERT OR IGNORE INTO schema_migrations (version, applied_at_millis) VALUES (?, ?)",
    )
    .bind(CURRENT_SCHEMA_VERSION)
    .bind(now_millis()?)
    .execute(&mut *connection)
    .await?;

    Ok(())
}

pub async fn seed_operator_credentials(pool: &SqlitePool) -> ServerResult<()> {
    let existing: Option<i64> =
        sqlx::query_scalar("SELECT id FROM operator_credentials WHERE id = 1")
            .fetch_optional(pool)
            .await?;
    if existing.is_some() {
        return Ok(());
    }

    let password_hash = hash_password("admin")?;
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO operator_credentials
         (id, username, password_hash, must_change_credentials, created_at_millis, updated_at_millis)
         VALUES (1, ?, ?, 1, ?, ?)",
    )
    .bind("admin")
    .bind(password_hash)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn seed_app_settings(pool: &SqlitePool, config: &ServerConfig) -> ServerResult<()> {
    let now = now_millis()?;
    sqlx::query(
        "INSERT INTO app_settings
         (id, public_base_url, session_key_generated_at_millis, schema_version, app_version,
          created_at_millis, updated_at_millis)
         VALUES (1, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
            public_base_url = excluded.public_base_url,
            schema_version = excluded.schema_version,
            app_version = excluded.app_version,
            updated_at_millis = excluded.updated_at_millis",
    )
    .bind(&config.public_base_url)
    .bind(now)
    .bind(CURRENT_SCHEMA_VERSION)
    .bind(&config.app_version)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub fn hash_password(password: &str) -> ServerResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| ServerError::Password(error.to_string()))
}

pub fn parse_password_hash(encoded: &str) -> ServerResult<PasswordHash<'_>> {
    PasswordHash::new(encoded).map_err(|error| ServerError::Password(error.to_string()))
}

fn create_parent_directory(database_url: &str) -> ServerResult<()> {
    let Some(path) = sqlite_path(database_url) else {
        return Ok(());
    };
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(parent)?;
    Ok(())
}

fn sqlite_path(database_url: &str) -> Option<&Path> {
    if database_url == "sqlite::memory:" || database_url == ":memory:" {
        return None;
    }
    let path = database_url
        .strip_prefix("sqlite://")
        .or_else(|| database_url.strip_prefix("sqlite:"))
        .unwrap_or(database_url);
    Some(Path::new(path))
}
