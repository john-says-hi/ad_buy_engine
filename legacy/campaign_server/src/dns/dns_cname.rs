use crate::utils::errors::ApiError;
use actix_web::client::Client;
use actix_web::web::Data;
use ad_buy_engine::constant::server_info::{DNS_LINODE_API_TOKEN_ENV, HOST_DOMAIN};
use std::env;
use std::process::Command;

pub async fn request_subdomain(
    _client: Data<Client>,
    subdomain: String,
) -> Result<String, ApiError> {
    let curl_executable = if cfg!(target_os = "freebsd") {
        "/usr/local/bin/curl"
    } else {
        "/usr/bin/curl"
    };

    let api_token = env::var(DNS_LINODE_API_TOKEN_ENV).map_err(|_| {
        ApiError::InternalServerError(format!("{} must be set", DNS_LINODE_API_TOKEN_ENV))
    })?;

    let new_subdomain = NewSubdomain {
        _type: "CNAME".to_string(),
        name: subdomain,
        target: HOST_DOMAIN.to_string(),
        priority: 50,
        weight: 50,
        port: 443,
        service: None,
        protocol: None,
        ttl_sec: 604800,
    };

    let request_body = serde_json::to_string(&new_subdomain)
        .map_err(|error| ApiError::InternalServerError(error.to_string()))?;

    let output = Command::new(curl_executable)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", api_token))
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(request_body)
        .arg("https://api.linode.com/v4/domains/1534143/records")
        .output()
        .map_err(|error| std::io::Error::new(error.kind(), error))?;

    if !output.status.success() {
        let error_body = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(ApiError::InternalServerError(error_body));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[derive(Serialize, Deserialize)]
pub struct NewSubdomain {
    #[serde(rename = "type")]
    _type: String,
    name: String,
    target: String,
    priority: u8,
    weight: u8,
    port: u32,
    service: Option<()>,
    protocol: Option<()>,
    ttl_sec: u64,
}
