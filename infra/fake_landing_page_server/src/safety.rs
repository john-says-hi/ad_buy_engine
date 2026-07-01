use std::net::{Ipv4Addr, Ipv6Addr};

use serde::Serialize;
use thiserror::Error;
use url::{Host, Url};

#[derive(Clone, Debug)]
pub struct SafetyPolicy {
    allow_hosts: Vec<String>,
    allow_private_network: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SafetySummary {
    pub allow_hosts: Vec<String>,
    pub allow_private_network: bool,
    pub default_loopback_only: bool,
}

#[derive(Debug, Error)]
pub enum SafetyError {
    #[error("URL must include a host")]
    MissingHost,
    #[error("private network host {host} requires --allow-private-network or --allow-host")]
    PrivateNetworkBlocked { host: String },
    #[error("public host {host} requires --allow-host")]
    PublicHostBlocked { host: String },
}

impl SafetyPolicy {
    pub fn new(allow_hosts: Vec<String>, allow_private_network: bool) -> Self {
        let allow_hosts = allow_hosts
            .into_iter()
            .map(|host| host.trim().to_ascii_lowercase())
            .filter(|host| !host.is_empty())
            .collect();
        Self {
            allow_hosts,
            allow_private_network,
        }
    }

    pub fn summary(&self) -> SafetySummary {
        SafetySummary {
            allow_hosts: self.allow_hosts.clone(),
            allow_private_network: self.allow_private_network,
            default_loopback_only: self.allow_hosts.is_empty() && !self.allow_private_network,
        }
    }

    pub fn ensure_url_allowed(&self, url: &Url) -> Result<(), SafetyError> {
        let host = normalized_host(url)?;
        if is_loopback_url(url) || self.host_is_allowlisted(url, &host) {
            return Ok(());
        }
        if is_private_network_url(url) {
            if self.allow_private_network {
                return Ok(());
            }
            return Err(SafetyError::PrivateNetworkBlocked { host });
        }
        Err(SafetyError::PublicHostBlocked { host })
    }

    fn host_is_allowlisted(&self, url: &Url, host: &str) -> bool {
        let host_with_port = match url.port() {
            Some(port) => format!("{host}:{port}"),
            None => host.to_string(),
        };
        self.allow_hosts
            .iter()
            .any(|allowed| allowed == host || allowed == &host_with_port)
    }
}

pub fn is_loopback_url(url: &Url) -> bool {
    match url.host() {
        Some(Host::Domain(domain)) => domain.eq_ignore_ascii_case("localhost"),
        Some(Host::Ipv4(ip)) => ip.is_loopback(),
        Some(Host::Ipv6(ip)) => ip.is_loopback(),
        None => false,
    }
}

pub fn is_private_network_url(url: &Url) -> bool {
    match url.host() {
        Some(Host::Ipv4(ip)) => is_private_ipv4(ip),
        Some(Host::Ipv6(ip)) => is_private_ipv6(ip),
        Some(Host::Domain(_)) | None => false,
    }
}

fn normalized_host(url: &Url) -> Result<String, SafetyError> {
    url.host_str()
        .map(str::to_ascii_lowercase)
        .ok_or(SafetyError::MissingHost)
}

fn is_private_ipv4(ip: Ipv4Addr) -> bool {
    ip.is_private() || ip.is_link_local()
}

fn is_private_ipv6(ip: Ipv6Addr) -> bool {
    ip.is_unicast_link_local() || ip.is_unique_local()
}
