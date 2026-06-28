use crate::constant::server_info::{CLICK_SERVER_IP_PORT_TERSE, HOST_DOMAIN, ROOT_DOMAIN};
use crate::data::account::generate_subdomain;
use crate::data::work_space::Clearance;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DomainsConfiguration {
    pub subdomain: String,
    pub main_domain: Url,
    pub dedicated_domains: Vec<DedicatedDomainName>,
    pub custom_domain_names: Vec<CustomDomainName>,
    pub root_domain_configuration: RootDomainConfiguration,
}

impl DomainsConfiguration {
    pub fn return_all_tracking_urls_no_filter(&self) -> Vec<Url> {
        let mut list = vec![];
        // list.push(self.subdomain.clone());
        list.push(self.main_domain.clone());
        for i in self.custom_domain_names.iter() {
            list.push(i.domain.clone())
        }
        for i in self.dedicated_domains.iter() {
            list.push(i.domain.clone())
        }
        list
    }
}

impl DomainsConfiguration {
    pub fn create() -> Self {
        let subdomain = generate_subdomain();
        let main_domain = format!("https://{}.{}", &subdomain, HOST_DOMAIN)
            .parse::<Url>()
            .expect("f3ewfasdf");
        Self {
            subdomain,
            main_domain,
            dedicated_domains: vec![],
            custom_domain_names: vec![],
            root_domain_configuration: RootDomainConfiguration::FourZeroFour,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomDomainName {
    pub domain: Url,
    pub clearance: Clearance,
    pub dns_status: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DedicatedDomainName {
    pub domain: Url,
    pub clearance: Clearance,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RootDomainConfiguration {
    FourZeroFour,
    RedirectTo(Url),
}

// impl
