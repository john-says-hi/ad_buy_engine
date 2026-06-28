use std::borrow::Borrow;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

use maxminddb::geoip2::model::Traits;
use maxminddb::geoip2::*;
use uuid::Uuid;

use geo_ip::GeoIPData;
use user_agent::UserAgentData;

use crate::data::custom_events::CustomConversionEvent;
use crate::data::live_campaign::LiveCampaign;
use crate::data::visit::click_event::ClickEvent;
use crate::data::visit::click_map::ClickMap;
use crate::data::visit::conversion::Conversion;
use crate::AError;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::time::Duration;
use url::Url;

pub mod click_event;
pub mod click_map;
pub mod conversion;
pub mod geo_ip;
pub mod user_agent;
pub mod visit_identity;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VisitClick {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Visit {
    pub id: i64,
    pub account_id: Uuid,
    pub campaign_id: Uuid,
    pub traffic_source_id: Uuid,
    pub funnel_id: Option<Uuid>,
    pub pre_sell_landing_page_id: Option<Uuid>,
    pub landing_page_ids: Vec<Uuid>,
    pub offer_ids: Vec<Uuid>,
    pub impressions_from_traffic_source: u64,
    pub tracking_link_clicks: u32,
    pub pre_landing_page_clicks: Vec<ClickEvent>,
    pub landing_page_clicks: Vec<ClickEvent>,
    pub offer_clicks: Vec<ClickEvent>,
    pub referrer: Url,
    pub traffic_source_parameters: HashMap<String, String>,
    pub redirection_time: Duration,
    pub click_map: ClickMap,
    pub user_agent_data: UserAgentData,
    pub geo_ip_data: GeoIPData,
    pub conversions: Vec<Conversion>,
    pub custom_conversions: Vec<CustomConversionEvent>,
    pub click_is_suspicious: bool,
    pub last_updated: DateTime<Utc>,
}

//
// impl Visit {
//     #[cfg(feature = "use-ua-parser")]
//     pub fn create(
//         campaign: &LiveCampaign,
//         ip: IpAddr,
//         ua: String,
//         referrer: String,
//         tokens: HashMap<String, String>,
//     ) -> Result<(Visit, VisitIdentity), AError> {
//         let mut meta = MetaData {
//             click_id: Uuid::new_v4(),
//             timestamp: chrono::Local::now().timestamp(),
//             campaign: campaign.campaign_id.clone(),
//             sequence: None,
//             account_id: campaign.account_id.clone(),
//             referrer,
//             tokens,
//             redirection_time: 0,
//             traffic_source: Default::default(),
//         };
//
//         let user_agent_data = UserAgentData::new(ua.clone());
//
//         let city_reader = maxminddb::Reader::open_readfile("GeoLite2-City.mmdb").expect("Fd32");
//         let asn_reader = maxminddb::Reader::open_readfile("GeoLite2-ASN.mmdb").expect("F3h2");
//
//         if let Err(_) = &city_reader.lookup::<City>(ip) {
//             // Err(AError::msg("No IP Found"))
//             // todo deployment, remove this ip for testing
//             println!("Debug IP is: {}", &ip.to_string());
//             let ip = IpAddr::from_str("24.245.77.178")?;
//
//             let city: City = city_reader.lookup(ip)?;
//
//             let isp: Isp = asn_reader.lookup(ip).expect("fdds");
//             let ct: ConnectionType = asn_reader.lookup(ip).expect("fdds");
//             let traits: Traits = city_reader.lookup(ip).expect("fdds");
//             let anonymous_ip: AnonymousIp = city_reader.lookup(ip).expect("fdds");
//             let density: DensityIncome = city_reader.lookup(ip).expect("fdds");
//
//             let city_name = if let Some(c) = city.city {
//                 if let Some(n) = c.names {
//                     n.get("en").expect("Gg3").to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let continent = if let Some(c) = city.continent {
//                 if let Some(n) = c.names {
//                     n.get("en").expect("Gg3").to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let country_iso_code = if let Some(c) = city.country {
//                 if let Some(n) = c.iso_code {
//                     n.to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let subdivision_iso_code = if let Some(c) = city.subdivisions {
//                 if let Some(n) = c.get(0) {
//                     n.iso_code.unwrap_or("").to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let time_zone = if let Some(c) = &city.location {
//                 if let Some(n) = c.time_zone {
//                     n.to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let latitude = if let Some(c) = &city.location {
//                 if let Some(n) = c.latitude {
//                     n
//                 } else {
//                     0.0
//                 }
//             } else {
//                 0.0
//             };
//
//             let longitude = if let Some(c) = &city.location {
//                 if let Some(n) = c.longitude {
//                     n
//                 } else {
//                     0.0
//                 }
//             } else {
//                 0.0
//             };
//
//             let metro_code = if let Some(c) = &city.location {
//                 if let Some(n) = c.metro_code {
//                     n
//                 } else {
//                     0
//                 }
//             } else {
//                 0
//             };
//
//             let postal_code = if let Some(c) = city.postal {
//                 if let Some(n) = c.code {
//                     n.to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let asn = if let Some(c) = isp.autonomous_system_organization {
//                 c.to_string()
//             } else {
//                 "".to_string()
//             };
//
//             let isp = if let Some(c) = isp.isp {
//                 c.to_string()
//             } else {
//                 "".to_string()
//             };
//
//             let connection_type = if let Some(c) = ct.connection_type {
//                 c.to_string()
//             } else {
//                 "".to_string()
//             };
//
//             let is_anonymous_proxy = if let Some(c) = traits.is_anonymous_proxy {
//                 c
//             } else {
//                 false
//             };
//
//             let is_anonymous = if let Some(c) = anonymous_ip.is_anonymous {
//                 c
//             } else {
//                 false
//             };
//
//             let is_anonymous_vpn = if let Some(c) = anonymous_ip.is_anonymous_vpn {
//                 c
//             } else {
//                 false
//             };
//
//             let is_hosting_provider = if let Some(c) = anonymous_ip.is_hosting_provider {
//                 c
//             } else {
//                 false
//             };
//
//             let is_public_proxy = if let Some(c) = anonymous_ip.is_public_proxy {
//                 c
//             } else {
//                 false
//             };
//
//             let is_satellite_provider = if let Some(c) = traits.is_satellite_provider {
//                 c
//             } else {
//                 false
//             };
//
//             let is_tor_exit_node = if let Some(c) = anonymous_ip.is_tor_exit_node {
//                 c
//             } else {
//                 false
//             };
//
//             let average_income = if let Some(c) = density.average_income {
//                 c
//             } else {
//                 0
//             };
//
//             let population_density = if let Some(c) = density.population_density {
//                 c
//             } else {
//                 0
//             };
//
//             let geo_ip_data = GeoIPData {
//                 ip,
//                 city: city_name,
//                 continent,
//                 country_iso_code,
//                 subdivision_iso_code,
//                 time_zone,
//                 latitude,
//                 longitude,
//                 metro_code,
//                 postal_code,
//                 asn,
//                 isp,
//                 connection_type,
//                 is_anonymous_proxy,
//                 is_anonymous,
//                 is_anonymous_vpn,
//                 is_hosting_provider,
//                 is_public_proxy,
//                 is_satellite_provider,
//                 is_tor_exit_node,
//                 average_income,
//                 population_density,
//             };
//
//             let click_map = ClickMap::new(campaign).expect("G333d");
//             meta.sequence = click_map.sequence_id.clone();
//
//             let visit = Visit {
//                 meta,
//                 user_agent_data,
//                 geo_ip_data,
//                 click_data: ClickData::first(&click_map.click_map),
//             };
//
//             let visit_identity = VisitIdentity::new(visit.clone(), click_map.click_map.clone());
//
//             Ok((visit, visit_identity))
//         } else {
//             let city: City = city_reader.lookup(ip)?;
//
//             let isp: Isp = asn_reader.lookup(ip).expect("fdds");
//             let ct: ConnectionType = asn_reader.lookup(ip).expect("fdds");
//             let traits: Traits = city_reader.lookup(ip).expect("fdds");
//             let anonymous_ip: AnonymousIp = city_reader.lookup(ip).expect("fdds");
//             let density: DensityIncome = city_reader.lookup(ip).expect("fdds");
//
//             let city_name = if let Some(c) = city.city {
//                 if let Some(n) = c.names {
//                     n.get("en").expect("Gg3").to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let continent = if let Some(c) = city.continent {
//                 if let Some(n) = c.names {
//                     n.get("en").expect("Gg3").to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let country_iso_code = if let Some(c) = city.country {
//                 if let Some(n) = c.iso_code {
//                     n.to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let subdivision_iso_code = if let Some(c) = city.subdivisions {
//                 if let Some(n) = c.get(0) {
//                     n.iso_code.unwrap_or("").to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let time_zone = if let Some(c) = &city.location {
//                 if let Some(n) = c.time_zone {
//                     n.to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let latitude = if let Some(c) = &city.location {
//                 if let Some(n) = c.latitude {
//                     n
//                 } else {
//                     0.0
//                 }
//             } else {
//                 0.0
//             };
//
//             let longitude = if let Some(c) = &city.location {
//                 if let Some(n) = c.longitude {
//                     n
//                 } else {
//                     0.0
//                 }
//             } else {
//                 0.0
//             };
//
//             let metro_code = if let Some(c) = &city.location {
//                 if let Some(n) = c.metro_code {
//                     n
//                 } else {
//                     0
//                 }
//             } else {
//                 0
//             };
//
//             let postal_code = if let Some(c) = city.postal {
//                 if let Some(n) = c.code {
//                     n.to_string()
//                 } else {
//                     "".to_string()
//                 }
//             } else {
//                 "".to_string()
//             };
//
//             let asn = if let Some(c) = isp.autonomous_system_organization {
//                 c.to_string()
//             } else {
//                 "".to_string()
//             };
//
//             let isp = if let Some(c) = isp.isp {
//                 c.to_string()
//             } else {
//                 "".to_string()
//             };
//
//             let connection_type = if let Some(c) = ct.connection_type {
//                 c.to_string()
//             } else {
//                 "".to_string()
//             };
//
//             let is_anonymous_proxy = if let Some(c) = traits.is_anonymous_proxy {
//                 c
//             } else {
//                 false
//             };
//
//             let is_anonymous = if let Some(c) = anonymous_ip.is_anonymous {
//                 c
//             } else {
//                 false
//             };
//
//             let is_anonymous_vpn = if let Some(c) = anonymous_ip.is_anonymous_vpn {
//                 c
//             } else {
//                 false
//             };
//
//             let is_hosting_provider = if let Some(c) = anonymous_ip.is_hosting_provider {
//                 c
//             } else {
//                 false
//             };
//
//             let is_public_proxy = if let Some(c) = anonymous_ip.is_public_proxy {
//                 c
//             } else {
//                 false
//             };
//
//             let is_satellite_provider = if let Some(c) = traits.is_satellite_provider {
//                 c
//             } else {
//                 false
//             };
//
//             let is_tor_exit_node = if let Some(c) = anonymous_ip.is_tor_exit_node {
//                 c
//             } else {
//                 false
//             };
//
//             let average_income = if let Some(c) = density.average_income {
//                 c
//             } else {
//                 0
//             };
//
//             let population_density = if let Some(c) = density.population_density {
//                 c
//             } else {
//                 0
//             };
//
//             let geo_ip_data = GeoIPData {
//                 ip,
//                 city: city_name,
//                 continent,
//                 country_iso_code,
//                 subdivision_iso_code,
//                 time_zone,
//                 latitude,
//                 longitude,
//                 metro_code,
//                 postal_code,
//                 asn,
//                 isp,
//                 connection_type,
//                 is_anonymous_proxy,
//                 is_anonymous,
//                 is_anonymous_vpn,
//                 is_hosting_provider,
//                 is_public_proxy,
//                 is_satellite_provider,
//                 is_tor_exit_node,
//                 average_income,
//                 population_density,
//             };
//
//             let click_map = ClickMap::new(campaign).expect("G333d");
//
//             let visit = Visit {
//                 meta,
//
//                 user_agent_data,
//                 geo_ip_data,
//                 click_data: ClickData::first(&click_map.click_map), //todo impl initial click data_types
//             };
//
//             let visit_identity = VisitIdentity::new(visit.clone(), click_map.click_map.clone());
//
//             Ok((visit, visit_identity))
//         }
//     }
// }
//
// impl Visit {
//     // pub fn parse_ua(&mut self, ua: &str) {
//     //     let user_agent_data =
//     //         UserAgentData::new(ua.to_string());
//     //
//     //     self.user_agent_data = user_agent_data;
//     // }
//
//     pub fn parse_geoip(&mut self, ip: std::net::IpAddr) {
//         let city_reader = maxminddb::Reader::open_readfile("GeoLite2-City.mmdb").expect("F32");
//         let asn_reader = maxminddb::Reader::open_readfile("GeoLite2-ASN.mmdb").expect("F32");
//
//         let city: City = city_reader.lookup(ip).expect("fds");
//         let isp: Isp = asn_reader.lookup(ip).expect("fdds");
//         let ct: ConnectionType = asn_reader.lookup(ip).expect("fdds");
//         let traits: Traits = city_reader.lookup(ip).expect("fdds");
//         let anonymous_ip: AnonymousIp = city_reader.lookup(ip).expect("fdds");
//         let density: DensityIncome = city_reader.lookup(ip).expect("fdds");
//
//         let city_name = if let Some(c) = city.city {
//             if let Some(n) = c.names {
//                 n.get("en").expect("Gg3")
//             } else {
//                 ""
//             }
//         } else {
//             ""
//         };
//
//         let continent = if let Some(c) = city.continent {
//             if let Some(n) = c.names {
//                 n.get("en").expect("Gg3")
//             } else {
//                 ""
//             }
//         } else {
//             ""
//         };
//
//         let country_iso_code = if let Some(c) = city.country {
//             if let Some(n) = c.iso_code {
//                 n
//             } else {
//                 ""
//             }
//         } else {
//             ""
//         };
//
//         let subdivision_iso_code = if let Some(c) = city.subdivisions {
//             if let Some(n) = c.get(0) {
//                 n.iso_code.unwrap_or("")
//             } else {
//                 ""
//             }
//         } else {
//             ""
//         };
//
//         let time_zone = if let Some(c) = &city.location {
//             if let Some(n) = c.time_zone {
//                 n
//             } else {
//                 ""
//             }
//         } else {
//             ""
//         };
//
//         let latitude = if let Some(c) = &city.location {
//             if let Some(n) = c.latitude {
//                 n
//             } else {
//                 0.0
//             }
//         } else {
//             0.0
//         };
//
//         let longitude = if let Some(c) = &city.location {
//             if let Some(n) = c.longitude {
//                 n
//             } else {
//                 0.0
//             }
//         } else {
//             0.0
//         };
//
//         let metro_code = if let Some(c) = &city.location {
//             if let Some(n) = c.metro_code {
//                 n
//             } else {
//                 0
//             }
//         } else {
//             0
//         };
//
//         let postal_code = if let Some(c) = city.postal {
//             if let Some(n) = c.code {
//                 n
//             } else {
//                 ""
//             }
//         } else {
//             ""
//         };
//
//         let asn = if let Some(c) = isp.autonomous_system_organization {
//             c
//         } else {
//             ""
//         };
//
//         let isp = if let Some(c) = isp.isp { c } else { "" };
//
//         let connection_type = if let Some(c) = ct.connection_type {
//             c
//         } else {
//             ""
//         };
//
//         let is_anonymous_proxy = if let Some(c) = traits.is_anonymous_proxy {
//             c
//         } else {
//             false
//         };
//
//         let is_anonymous = if let Some(c) = anonymous_ip.is_anonymous {
//             c
//         } else {
//             false
//         };
//
//         let is_anonymous_vpn = if let Some(c) = anonymous_ip.is_anonymous_vpn {
//             c
//         } else {
//             false
//         };
//
//         let is_hosting_provider = if let Some(c) = anonymous_ip.is_hosting_provider {
//             c
//         } else {
//             false
//         };
//
//         let is_public_proxy = if let Some(c) = anonymous_ip.is_public_proxy {
//             c
//         } else {
//             false
//         };
//
//         let is_satellite_provider = if let Some(c) = traits.is_satellite_provider {
//             c
//         } else {
//             false
//         };
//
//         let is_tor_exit_node = if let Some(c) = anonymous_ip.is_tor_exit_node {
//             c
//         } else {
//             false
//         };
//
//         let average_income = if let Some(c) = density.average_income {
//             c
//         } else {
//             0
//         };
//
//         let population_density = if let Some(c) = density.population_density {
//             c
//         } else {
//             0
//         };
//
//         self.geo_ip_data.ip = ip;
//         self.geo_ip_data.city = city_name.to_string();
//         self.geo_ip_data.continent = continent.to_string();
//         self.geo_ip_data.country_iso_code = country_iso_code.to_string();
//         self.geo_ip_data.subdivision_iso_code = subdivision_iso_code.to_string();
//         self.geo_ip_data.time_zone = time_zone.to_string();
//         self.geo_ip_data.latitude = latitude;
//         self.geo_ip_data.longitude = longitude;
//         self.geo_ip_data.metro_code = metro_code;
//         self.geo_ip_data.postal_code = postal_code.to_string();
//         self.geo_ip_data.asn = asn.to_string();
//         self.geo_ip_data.isp = isp.to_string();
//         self.geo_ip_data.connection_type = connection_type.to_string();
//         self.geo_ip_data.is_anonymous_proxy = is_anonymous_proxy;
//         self.geo_ip_data.is_anonymous = is_anonymous;
//         self.geo_ip_data.is_anonymous_vpn = is_anonymous_vpn;
//         self.geo_ip_data.is_hosting_provider = is_hosting_provider;
//         self.geo_ip_data.is_public_proxy = is_public_proxy;
//         self.geo_ip_data.is_satellite_provider = is_satellite_provider;
//         self.geo_ip_data.is_tor_exit_node = is_tor_exit_node;
//         self.geo_ip_data.average_income = average_income;
//         self.geo_ip_data.population_density = population_density;
//     }
// }
