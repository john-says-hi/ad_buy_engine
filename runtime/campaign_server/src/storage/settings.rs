use ad_buy_engine_domain::{
    GeolocationDatabaseStatus, GeolocationSettingsResponse, GeolocationSettingsUpdate,
};
use sqlx::{Row, SqlitePool};

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
    use super::license_key_preview;

    #[test]
    fn license_key_preview_never_returns_full_key() {
        assert_eq!(license_key_preview(""), None);
        assert_eq!(
            license_key_preview("abcdef123456"),
            Some("...3456".to_string())
        );
    }
}
