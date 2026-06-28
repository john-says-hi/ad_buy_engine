use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{GeolocationDownloadResponse, GeolocationDownloadedDatabase};
use flate2::read::GzDecoder;
use maxminddb::Reader;
use reqwest::Client;
use tar::Archive;

use crate::error::{ServerError, ServerResult};
use crate::storage::settings::StoredGeolocationSettings;
use crate::time::now_millis;

struct DownloadTarget<'a> {
    edition_id: &'static str,
    target_path: &'a str,
}

pub async fn download_geolite_databases(
    settings: &StoredGeolocationSettings,
) -> ServerResult<GeolocationDownloadResponse> {
    let client = Client::new();
    let targets = [
        DownloadTarget {
            edition_id: "GeoLite2-City",
            target_path: &settings.city_database_path,
        },
        DownloadTarget {
            edition_id: "GeoLite2-Country",
            target_path: &settings.country_database_path,
        },
        DownloadTarget {
            edition_id: "GeoLite2-ASN",
            target_path: &settings.asn_database_path,
        },
    ];

    let mut downloaded = Vec::new();
    for target in targets {
        downloaded.push(download_database(&client, settings, target).await?);
    }

    Ok(GeolocationDownloadResponse {
        message: format!("Downloaded {} GeoLite databases", downloaded.len()),
        downloaded,
    })
}

async fn download_database(
    client: &Client,
    settings: &StoredGeolocationSettings,
    target: DownloadTarget<'_>,
) -> ServerResult<GeolocationDownloadedDatabase> {
    let url = download_url(target.edition_id, &settings.license_key);
    let response = client.get(url).send().await.map_err(|_| {
        ServerError::internal(format!("MaxMind download failed for {}", target.edition_id))
    })?;
    if !response.status().is_success() {
        return Err(ServerError::bad_request(format!(
            "MaxMind rejected the {} download request",
            target.edition_id
        )));
    }
    let archive_bytes = response.bytes().await.map_err(|_| {
        ServerError::internal(format!("MaxMind response failed for {}", target.edition_id))
    })?;
    let target_path = Path::new(target.target_path);
    let temporary_path = temporary_database_path(target_path, target.edition_id)?;
    extract_mmdb(&archive_bytes, &temporary_path)?;
    let reader = Reader::open_readfile(&temporary_path).map_err(|error| {
        ServerError::internal(format!("Downloaded database is invalid: {error}"))
    })?;
    let database_type = reader.metadata.database_type.clone();
    let build_epoch = reader.metadata.build_epoch;
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::rename(&temporary_path, target_path)?;

    Ok(GeolocationDownloadedDatabase {
        edition_id: target.edition_id.to_string(),
        path: target_path.display().to_string(),
        database_type,
        build_epoch,
    })
}

fn download_url(edition_id: &str, license_key: &str) -> String {
    format!(
        "https://download.maxmind.com/app/geoip_download?edition_id={}&license_key={}&suffix=tar.gz",
        urlencoding::encode(edition_id),
        urlencoding::encode(license_key)
    )
}

fn temporary_database_path(target_path: &Path, edition_id: &str) -> ServerResult<PathBuf> {
    let parent = target_path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    Ok(parent.join(format!(".{}-{}.download", edition_id, now_millis()?)))
}

fn extract_mmdb(archive_bytes: &[u8], temporary_path: &Path) -> ServerResult<()> {
    let decoder = GzDecoder::new(archive_bytes);
    let mut archive = Archive::new(decoder);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".mmdb"))
        {
            let mut output = fs::File::create(temporary_path)?;
            io::copy(&mut entry, &mut output)?;
            return Ok(());
        }
    }
    Err(ServerError::internal(
        "MaxMind archive did not contain an mmdb database",
    ))
}
