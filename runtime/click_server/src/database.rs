use crate::config::ClickServerConfig;
use anyhow::Context;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn connect_database(config: &ClickServerConfig) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(&config.database_url)
        .with_context(|| format!("invalid SQLite URL: {}", config.database_url))?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .context("failed to connect to SQLite")
}

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    MIGRATOR
        .run(pool)
        .await
        .context("failed to run SQLite migrations")
}
