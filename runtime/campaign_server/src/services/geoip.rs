use std::net::IpAddr;
use std::path::Path;
use std::sync::{Arc, RwLock};

use ad_buy_engine_domain::{GeolocationDatabaseStatus, VisitEnrichment};
use maxminddb::{Reader, geoip2};

use crate::error::ServerResult;
use crate::time::now_millis;

pub type SharedGeoIpService = Arc<RwLock<GeoIpService>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeoIpSettings {
    pub city_database_path: String,
    pub country_database_path: String,
    pub asn_database_path: String,
}

#[derive(Debug)]
pub struct GeoIpService {
    city: LoadedDatabase,
    country: LoadedDatabase,
    asn: LoadedDatabase,
}

impl GeoIpService {
    pub fn load(settings: &GeoIpSettings) -> ServerResult<Self> {
        let loaded_at_millis = now_millis()?;
        Ok(Self {
            city: LoadedDatabase::load(&settings.city_database_path, loaded_at_millis),
            country: LoadedDatabase::load(&settings.country_database_path, loaded_at_millis),
            asn: LoadedDatabase::load(&settings.asn_database_path, loaded_at_millis),
        })
    }

    pub fn shared(settings: &GeoIpSettings) -> ServerResult<SharedGeoIpService> {
        Ok(Arc::new(RwLock::new(Self::load(settings)?)))
    }

    pub fn reload(shared: &SharedGeoIpService, settings: &GeoIpSettings) -> ServerResult<()> {
        let service = Self::load(settings)?;
        if let Ok(mut guard) = shared.write() {
            *guard = service;
        }
        Ok(())
    }

    pub fn lookup(&self, ip_address: Option<&str>) -> VisitEnrichment {
        let Some(ip_address) = ip_address else {
            return VisitEnrichment::default();
        };
        let Ok(ip_address) = ip_address.parse::<IpAddr>() else {
            return VisitEnrichment::default();
        };

        let mut enrichment = VisitEnrichment::default();
        self.apply_city(ip_address, &mut enrichment);
        self.apply_country(ip_address, &mut enrichment);
        self.apply_asn(ip_address, &mut enrichment);
        enrichment
    }

    pub fn city_status(&self) -> GeolocationDatabaseStatus {
        self.city.status.clone()
    }

    pub fn country_status(&self) -> GeolocationDatabaseStatus {
        self.country.status.clone()
    }

    pub fn asn_status(&self) -> GeolocationDatabaseStatus {
        self.asn.status.clone()
    }

    fn apply_city(&self, ip_address: IpAddr, enrichment: &mut VisitEnrichment) {
        let Some(reader) = self.city.reader.as_ref() else {
            return;
        };
        let Ok(city) = reader.lookup::<geoip2::City>(ip_address) else {
            return;
        };

        if let Some(country) = city.country {
            enrichment.country = country.iso_code.map(ToOwned::to_owned);
        }
        if let Some(city) = city.city {
            enrichment.city = english_name(city.names);
        }
        if let Some(subdivisions) = city.subdivisions {
            enrichment.region = subdivisions.into_iter().next().and_then(|subdivision| {
                english_name(subdivision.names)
                    .or_else(|| subdivision.iso_code.map(|iso_code| iso_code.to_string()))
            });
        }
        if let Some(location) = city.location {
            enrichment.timezone = location.time_zone.map(ToOwned::to_owned);
            enrichment.metro_code = location.metro_code.map(|metro_code| metro_code.to_string());
        }
        if let Some(postal) = city.postal {
            enrichment.postal_code = postal.code.map(ToOwned::to_owned);
        }
    }

    fn apply_country(&self, ip_address: IpAddr, enrichment: &mut VisitEnrichment) {
        if enrichment.country.is_some() {
            return;
        }
        let Some(reader) = self.country.reader.as_ref() else {
            return;
        };
        let Ok(country) = reader.lookup::<geoip2::Country>(ip_address) else {
            return;
        };
        if let Some(country) = country.country {
            enrichment.country = country.iso_code.map(ToOwned::to_owned);
        }
    }

    fn apply_asn(&self, ip_address: IpAddr, enrichment: &mut VisitEnrichment) {
        let Some(reader) = self.asn.reader.as_ref() else {
            return;
        };
        let Ok(asn) = reader.lookup::<geoip2::Asn>(ip_address) else {
            return;
        };
        enrichment.asn = asn
            .autonomous_system_number
            .map(|number| format!("AS{number}"));
        enrichment.asn_organization = asn.autonomous_system_organization.map(ToOwned::to_owned);
    }
}

#[derive(Debug)]
struct LoadedDatabase {
    reader: Option<Reader<Vec<u8>>>,
    status: GeolocationDatabaseStatus,
}

impl LoadedDatabase {
    fn load(configured_path: &str, loaded_at_millis: i64) -> Self {
        let path = Path::new(configured_path);
        if !path.exists() {
            return Self {
                reader: None,
                status: GeolocationDatabaseStatus {
                    configured_path: configured_path.to_string(),
                    exists: false,
                    database_type: None,
                    build_epoch: None,
                    last_loaded_at_millis: None,
                    error: None,
                },
            };
        }

        match Reader::open_readfile(path) {
            Ok(reader) => {
                let status = GeolocationDatabaseStatus {
                    configured_path: configured_path.to_string(),
                    exists: true,
                    database_type: Some(reader.metadata.database_type.clone()),
                    build_epoch: Some(reader.metadata.build_epoch),
                    last_loaded_at_millis: Some(loaded_at_millis),
                    error: None,
                };
                Self {
                    reader: Some(reader),
                    status,
                }
            }
            Err(error) => Self {
                reader: None,
                status: GeolocationDatabaseStatus {
                    configured_path: configured_path.to_string(),
                    exists: true,
                    database_type: None,
                    build_epoch: None,
                    last_loaded_at_millis: None,
                    error: Some(error.to_string()),
                },
            },
        }
    }
}

fn english_name(names: Option<std::collections::BTreeMap<&str, &str>>) -> Option<String> {
    names
        .as_ref()
        .and_then(|names| names.get("en"))
        .or_else(|| names.as_ref().and_then(|names| names.values().next()))
        .map(|name| (*name).to_string())
}
