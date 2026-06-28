use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::{Argon2, PasswordHash};
use rand_core::OsRng;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Executor, Row, SqlitePool};

use crate::config::ServerConfig;
use crate::error::{ServerError, ServerResult};
use crate::storage::settings::domain_from_base_url;
use crate::time::now_millis;

const INIT_MIGRATION: &str = include_str!("../../migrations/0001_init.sql");
const CURRENT_SCHEMA_VERSION: i64 = 3;

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
    ensure_schema_columns(&mut connection).await?;
    sqlx::query(
        "INSERT OR IGNORE INTO schema_migrations (version, applied_at_millis) VALUES (?, ?)",
    )
    .bind(CURRENT_SCHEMA_VERSION)
    .bind(now_millis()?)
    .execute(&mut *connection)
    .await?;

    Ok(())
}

async fn ensure_schema_columns(
    connection: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
) -> ServerResult<()> {
    for column in APP_SETTINGS_COLUMNS {
        ensure_column(connection, "app_settings", *column).await?;
    }
    for column in VISIT_ENRICHMENT_COLUMNS {
        ensure_column(connection, "visits", *column).await?;
    }
    Ok(())
}

async fn ensure_column(
    connection: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
    table: &str,
    column: ColumnDefinition,
) -> ServerResult<()> {
    if table_has_column(connection, table, column.name).await? {
        return Ok(());
    }
    connection
        .execute(format!("ALTER TABLE {table} ADD COLUMN {}", column.sql).as_str())
        .await?;
    Ok(())
}

async fn table_has_column(
    connection: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
    table: &str,
    column_name: &str,
) -> ServerResult<bool> {
    let rows = sqlx::query(&format!("PRAGMA table_info({table})"))
        .fetch_all(&mut **connection)
        .await?;
    Ok(rows
        .iter()
        .filter_map(|row| row.try_get::<String, _>("name").ok())
        .any(|name| name == column_name))
}

#[derive(Clone, Copy)]
struct ColumnDefinition {
    name: &'static str,
    sql: &'static str,
}

const APP_SETTINGS_COLUMNS: &[ColumnDefinition] = &[
    ColumnDefinition {
        name: "maxmind_account_id",
        sql: "maxmind_account_id TEXT NOT NULL DEFAULT ''",
    },
    ColumnDefinition {
        name: "primary_tracking_domain",
        sql: "primary_tracking_domain TEXT NOT NULL DEFAULT ''",
    },
    ColumnDefinition {
        name: "tracking_base_url",
        sql: "tracking_base_url TEXT NOT NULL DEFAULT ''",
    },
    ColumnDefinition {
        name: "admin_dashboard_domain",
        sql: "admin_dashboard_domain TEXT NOT NULL DEFAULT ''",
    },
    ColumnDefinition {
        name: "admin_dashboard_base_url",
        sql: "admin_dashboard_base_url TEXT NOT NULL DEFAULT ''",
    },
    ColumnDefinition {
        name: "domain_setup_status",
        sql: "domain_setup_status TEXT NOT NULL DEFAULT 'not_configured'",
    },
    ColumnDefinition {
        name: "maxmind_license_key",
        sql: "maxmind_license_key TEXT NOT NULL DEFAULT ''",
    },
    ColumnDefinition {
        name: "geolite_city_database_path",
        sql: "geolite_city_database_path TEXT NOT NULL DEFAULT 'runtime/data/GeoLite2-City.mmdb'",
    },
    ColumnDefinition {
        name: "geolite_country_database_path",
        sql: "geolite_country_database_path TEXT NOT NULL DEFAULT 'runtime/data/GeoLite2-Country.mmdb'",
    },
    ColumnDefinition {
        name: "geolite_asn_database_path",
        sql: "geolite_asn_database_path TEXT NOT NULL DEFAULT 'runtime/data/GeoLite2-ASN.mmdb'",
    },
    ColumnDefinition {
        name: "geolite_last_download_at_millis",
        sql: "geolite_last_download_at_millis INTEGER",
    },
    ColumnDefinition {
        name: "geolite_last_download_error",
        sql: "geolite_last_download_error TEXT",
    },
];

const VISIT_ENRICHMENT_COLUMNS: &[ColumnDefinition] = &[
    ColumnDefinition {
        name: "country",
        sql: "country TEXT",
    },
    ColumnDefinition {
        name: "region",
        sql: "region TEXT",
    },
    ColumnDefinition {
        name: "city",
        sql: "city TEXT",
    },
    ColumnDefinition {
        name: "timezone",
        sql: "timezone TEXT",
    },
    ColumnDefinition {
        name: "postal_code",
        sql: "postal_code TEXT",
    },
    ColumnDefinition {
        name: "metro_code",
        sql: "metro_code TEXT",
    },
    ColumnDefinition {
        name: "asn",
        sql: "asn TEXT",
    },
    ColumnDefinition {
        name: "asn_organization",
        sql: "asn_organization TEXT",
    },
    ColumnDefinition {
        name: "isp",
        sql: "isp TEXT",
    },
    ColumnDefinition {
        name: "connection_type",
        sql: "connection_type TEXT",
    },
    ColumnDefinition {
        name: "proxy_type",
        sql: "proxy_type TEXT",
    },
    ColumnDefinition {
        name: "carrier",
        sql: "carrier TEXT",
    },
    ColumnDefinition {
        name: "browser",
        sql: "browser TEXT",
    },
    ColumnDefinition {
        name: "browser_version",
        sql: "browser_version TEXT",
    },
    ColumnDefinition {
        name: "operating_system",
        sql: "operating_system TEXT",
    },
    ColumnDefinition {
        name: "operating_system_version",
        sql: "operating_system_version TEXT",
    },
    ColumnDefinition {
        name: "device_type",
        sql: "device_type TEXT",
    },
    ColumnDefinition {
        name: "device_brand",
        sql: "device_brand TEXT",
    },
    ColumnDefinition {
        name: "device_model",
        sql: "device_model TEXT",
    },
];

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
    let primary_tracking_domain =
        domain_from_base_url(&config.tracking_base_url).unwrap_or_default();
    let admin_dashboard_domain =
        domain_from_base_url(&config.admin_dashboard_base_url).unwrap_or_default();
    sqlx::query(
        "INSERT INTO app_settings
         (id, public_base_url, primary_tracking_domain, tracking_base_url,
          admin_dashboard_domain, admin_dashboard_base_url, domain_setup_status,
          session_key_generated_at_millis, schema_version, app_version, maxmind_account_id,
          maxmind_license_key, geolite_city_database_path, geolite_country_database_path,
          geolite_asn_database_path, created_at_millis, updated_at_millis)
         VALUES (1, ?, ?, ?, ?, ?, 'not_configured', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
            schema_version = excluded.schema_version,
            app_version = excluded.app_version,
            updated_at_millis = excluded.updated_at_millis",
    )
    .bind(&config.public_base_url)
    .bind(primary_tracking_domain)
    .bind(&config.tracking_base_url)
    .bind(admin_dashboard_domain)
    .bind(&config.admin_dashboard_base_url)
    .bind(now)
    .bind(CURRENT_SCHEMA_VERSION)
    .bind(&config.app_version)
    .bind(&config.maxmind_account_id)
    .bind(&config.maxmind_license_key)
    .bind(&config.geolite_city_database_path)
    .bind(&config.geolite_country_database_path)
    .bind(&config.geolite_asn_database_path)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    backfill_missing_domain_settings(pool, config).await?;
    Ok(())
}

async fn backfill_missing_domain_settings(
    pool: &SqlitePool,
    config: &ServerConfig,
) -> ServerResult<()> {
    let row = sqlx::query(
        "SELECT public_base_url, primary_tracking_domain, tracking_base_url,
                admin_dashboard_domain, admin_dashboard_base_url, domain_setup_status
         FROM app_settings
         WHERE id = 1",
    )
    .fetch_one(pool)
    .await?;

    let public_base_url: String = row.try_get("public_base_url")?;
    let tracking_base_url = first_non_empty(
        row.try_get::<String, _>("tracking_base_url")?,
        [&public_base_url, &config.tracking_base_url],
    );
    let admin_dashboard_base_url = first_non_empty(
        row.try_get::<String, _>("admin_dashboard_base_url")?,
        [&public_base_url, &config.admin_dashboard_base_url],
    );
    let tracking_domain_from_base = domain_from_base_url(&tracking_base_url).unwrap_or_default();
    let tracking_domain_from_config =
        domain_from_base_url(&config.tracking_base_url).unwrap_or_default();
    let primary_tracking_domain = first_non_empty(
        row.try_get::<String, _>("primary_tracking_domain")?,
        [&tracking_domain_from_base, &tracking_domain_from_config],
    );
    let admin_domain_from_base =
        domain_from_base_url(&admin_dashboard_base_url).unwrap_or_default();
    let admin_domain_from_config =
        domain_from_base_url(&config.admin_dashboard_base_url).unwrap_or_default();
    let admin_dashboard_domain = first_non_empty(
        row.try_get::<String, _>("admin_dashboard_domain")?,
        [&admin_domain_from_base, &admin_domain_from_config],
    );
    let domain_setup_status = first_non_empty(
        row.try_get::<String, _>("domain_setup_status")?,
        ["not_configured", "not_configured"],
    );

    sqlx::query(
        "UPDATE app_settings SET
            primary_tracking_domain = ?,
            tracking_base_url = ?,
            admin_dashboard_domain = ?,
            admin_dashboard_base_url = ?,
            domain_setup_status = ?
         WHERE id = 1",
    )
    .bind(primary_tracking_domain)
    .bind(tracking_base_url)
    .bind(admin_dashboard_domain)
    .bind(admin_dashboard_base_url)
    .bind(domain_setup_status)
    .execute(pool)
    .await?;
    Ok(())
}

fn first_non_empty<const N: usize>(current: String, fallbacks: [&str; N]) -> String {
    if !current.trim().is_empty() {
        return current.trim().trim_end_matches('/').to_string();
    }
    fallbacks
        .into_iter()
        .find(|value| !value.trim().is_empty())
        .unwrap_or_default()
        .trim()
        .trim_end_matches('/')
        .to_string()
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
