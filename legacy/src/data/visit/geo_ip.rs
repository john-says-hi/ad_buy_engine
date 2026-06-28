use std::net::IpAddr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeoIPData {
    pub ip: IpAddr,
    pub city: String,
    pub continent: String,
    pub country_iso_code: String,
    pub subdivision_iso_code: String,
    pub time_zone: String,
    pub latitude: f64,
    pub longitude: f64,
    pub metro_code: u16,
    pub postal_code: String,
    pub asn: String,
    pub isp: String,
    pub connection_type: String,
    pub is_anonymous_proxy: bool,
    pub is_anonymous: bool,
    pub is_anonymous_vpn: bool,
    pub is_hosting_provider: bool,
    pub is_public_proxy: bool,
    pub is_satellite_provider: bool,
    pub is_tor_exit_node: bool,
    pub average_income: u32,
    pub population_density: u32,
}
