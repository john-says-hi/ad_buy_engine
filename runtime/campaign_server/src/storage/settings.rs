use ad_buy_engine_domain::{
    DomainSettingsResponse, DomainSettingsUpdate, DomainSetupStatus, GeolocationDatabaseStatus,
    GeolocationSettingsResponse, GeolocationSettingsUpdate,
};
use sqlx::{Row, SqlitePool};

use crate::config::BaseUrlOverrides;
use crate::error::ServerResult;
use crate::services::geoip::GeoIpSettings;
use crate::time::now_millis;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredGeolocationSettings {
    pub account_id: String,
    pub license_key: String,
    pub city_database_path: String,
    pub country_database_path: String,
    pub asn_database_path: String,
    pub updated_at_millis: i64,
}

impl StoredGeolocationSettings {
    pub fn geoip_settings(&self) -> GeoIpSettings {
        GeoIpSettings {
            city_database_path: self.city_database_path.clone(),
            country_database_path: self.country_database_path.clone(),
            asn_database_path: self.asn_database_path.clone(),
        }
    }

    pub fn to_response(
        &self,
        city_database: GeolocationDatabaseStatus,
        country_database: GeolocationDatabaseStatus,
        asn_database: GeolocationDatabaseStatus,
    ) -> GeolocationSettingsResponse {
        GeolocationSettingsResponse {
            account_id: self.account_id.clone(),
            license_key_configured: !self.license_key.trim().is_empty(),
            license_key_preview: license_key_preview(&self.license_key),
            city_database_path: self.city_database_path.clone(),
            country_database_path: self.country_database_path.clone(),
            asn_database_path: self.asn_database_path.clone(),
            city_database,
            country_database,
            asn_database,
            updated_at_millis: self.updated_at_millis,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredDomainSettings {
    pub primary_tracking_domain: String,
    pub tracking_base_url: String,
    pub admin_dashboard_domain: String,
    pub admin_dashboard_base_url: String,
    pub domain_setup_status: DomainSetupStatus,
    pub updated_at_millis: i64,
}

impl StoredDomainSettings {
    pub fn to_response(&self, overrides: &BaseUrlOverrides) -> DomainSettingsResponse {
        let tracking_base_url = effective_base_url(
            overrides.tracking_base_url.as_deref(),
            &self.tracking_base_url,
            &overrides.public_base_url_fallback,
        );
        let admin_dashboard_base_url = effective_base_url(
            overrides.admin_dashboard_base_url.as_deref(),
            &self.admin_dashboard_base_url,
            &overrides.public_base_url_fallback,
        );
        DomainSettingsResponse {
            primary_tracking_domain: response_domain(
                &self.primary_tracking_domain,
                &tracking_base_url,
            ),
            tracking_base_url,
            admin_dashboard_domain: response_domain(
                &self.admin_dashboard_domain,
                &admin_dashboard_base_url,
            ),
            admin_dashboard_base_url,
            domain_setup_status: self.domain_setup_status,
            updated_at_millis: self.updated_at_millis,
        }
    }

    pub fn effective_tracking_base_url(&self, overrides: &BaseUrlOverrides) -> String {
        effective_base_url(
            overrides.tracking_base_url.as_deref(),
            &self.tracking_base_url,
            &overrides.public_base_url_fallback,
        )
    }
}

pub async fn load_domain_settings(pool: &SqlitePool) -> ServerResult<StoredDomainSettings> {
    let row = sqlx::query(
        "SELECT primary_tracking_domain, tracking_base_url, admin_dashboard_domain,
                admin_dashboard_base_url, domain_setup_status, updated_at_millis
         FROM app_settings
         WHERE id = 1",
    )
    .fetch_one(pool)
    .await?;

    Ok(StoredDomainSettings {
        primary_tracking_domain: row.try_get("primary_tracking_domain")?,
        tracking_base_url: row.try_get("tracking_base_url")?,
        admin_dashboard_domain: row.try_get("admin_dashboard_domain")?,
        admin_dashboard_base_url: row.try_get("admin_dashboard_base_url")?,
        domain_setup_status: domain_setup_status_from_str(row.try_get("domain_setup_status")?),
        updated_at_millis: row.try_get("updated_at_millis")?,
    })
}

pub async fn update_domain_settings(
    pool: &SqlitePool,
    update: DomainSettingsUpdate,
) -> ServerResult<StoredDomainSettings> {
    validate_domain_update(&update)?;
    let now = now_millis()?;
    sqlx::query(
        "UPDATE app_settings SET
            primary_tracking_domain = ?,
            tracking_base_url = ?,
            admin_dashboard_domain = ?,
            admin_dashboard_base_url = ?,
            domain_setup_status = ?,
            updated_at_millis = ?
         WHERE id = 1",
    )
    .bind(update.normalized_primary_tracking_domain())
    .bind(update.tracking_base_url())
    .bind(update.normalized_admin_dashboard_domain())
    .bind(update.admin_dashboard_base_url())
    .bind(DomainSetupStatus::Configured.as_str())
    .bind(now)
    .execute(pool)
    .await?;

    load_domain_settings(pool).await
}

pub async fn effective_tracking_base_url(
    pool: &SqlitePool,
    overrides: &BaseUrlOverrides,
) -> ServerResult<String> {
    Ok(load_domain_settings(pool)
        .await?
        .effective_tracking_base_url(overrides))
}

pub async fn load_geolocation_settings(
    pool: &SqlitePool,
) -> ServerResult<StoredGeolocationSettings> {
    let row = sqlx::query(
        "SELECT maxmind_account_id, maxmind_license_key, geolite_city_database_path,
                geolite_country_database_path, geolite_asn_database_path, updated_at_millis
         FROM app_settings
         WHERE id = 1",
    )
    .fetch_one(pool)
    .await?;

    Ok(StoredGeolocationSettings {
        account_id: row.try_get("maxmind_account_id")?,
        license_key: row.try_get("maxmind_license_key")?,
        city_database_path: row.try_get("geolite_city_database_path")?,
        country_database_path: row.try_get("geolite_country_database_path")?,
        asn_database_path: row.try_get("geolite_asn_database_path")?,
        updated_at_millis: row.try_get("updated_at_millis")?,
    })
}

pub async fn update_geolocation_settings(
    pool: &SqlitePool,
    update: GeolocationSettingsUpdate,
) -> ServerResult<StoredGeolocationSettings> {
    let current = load_geolocation_settings(pool).await?;
    let license_key = match update.license_key {
        Some(license_key) => license_key,
        None => current.license_key,
    };
    let now = now_millis()?;
    sqlx::query(
        "UPDATE app_settings SET
            maxmind_account_id = ?,
            maxmind_license_key = ?,
            geolite_city_database_path = ?,
            geolite_country_database_path = ?,
            geolite_asn_database_path = ?,
            updated_at_millis = ?
         WHERE id = 1",
    )
    .bind(update.account_id.trim())
    .bind(license_key.trim())
    .bind(non_empty_or_default(
        &update.city_database_path,
        "runtime/data/GeoLite2-City.mmdb",
    ))
    .bind(non_empty_or_default(
        &update.country_database_path,
        "runtime/data/GeoLite2-Country.mmdb",
    ))
    .bind(non_empty_or_default(
        &update.asn_database_path,
        "runtime/data/GeoLite2-ASN.mmdb",
    ))
    .bind(now)
    .execute(pool)
    .await?;

    load_geolocation_settings(pool).await
}

fn non_empty_or_default<'a>(value: &'a str, default_value: &'a str) -> &'a str {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        default_value
    } else {
        trimmed
    }
}

fn validate_domain_update(update: &DomainSettingsUpdate) -> ServerResult<()> {
    let errors = update.validate();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(crate::error::ServerError::validation(
            "Domain settings update failed",
            errors,
        ))
    }
}

fn effective_base_url(override_url: Option<&str>, stored_url: &str, fallback_url: &str) -> String {
    override_url
        .filter(|url| !url.trim().is_empty())
        .or_else(|| non_empty(stored_url))
        .or_else(|| non_empty(fallback_url))
        .unwrap_or("http://127.0.0.1:8088")
        .trim_end_matches('/')
        .to_string()
}

fn response_domain(stored_domain: &str, effective_base_url: &str) -> String {
    non_empty(stored_domain)
        .map(ToOwned::to_owned)
        .or_else(|| domain_from_base_url(effective_base_url))
        .unwrap_or_default()
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

pub fn domain_from_base_url(base_url: &str) -> Option<String> {
    url::Url::parse(base_url)
        .ok()
        .and_then(|url| url.host_str().map(ToOwned::to_owned))
}

fn domain_setup_status_from_str(value: String) -> DomainSetupStatus {
    match value.as_str() {
        "configured" => DomainSetupStatus::Configured,
        _ => DomainSetupStatus::NotConfigured,
    }
}

fn license_key_preview(license_key: &str) -> Option<String> {
    let license_key = license_key.trim();
    if license_key.is_empty() {
        return None;
    }
    let suffix = license_key
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();
    Some(format!("...{suffix}"))
}

#[cfg(test)]
mod tests {
    use super::{domain_from_base_url, license_key_preview};

    #[test]
    fn license_key_preview_never_returns_full_key() {
        assert_eq!(license_key_preview(""), None);
        assert_eq!(
            license_key_preview("abcdef123456"),
            Some("...3456".to_string())
        );
    }

    #[test]
    fn domain_from_base_url_reads_host_without_scheme_or_port() {
        assert_eq!(
            domain_from_base_url("https://track.example.com:8443/path"),
            Some("track.example.com".to_string())
        );
    }
}
