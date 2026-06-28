use crate::data::visit::click_map::ClickMap;
use crate::data::visit::Visit;
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickIdentity {
    pub visit_record_id: Uuid,
    pub user_agent: String,
    pub ip: IpAddr,
    pub click_map: ClickMap,
}

// impl VisitIdentity {
//     pub fn new(visit: Visit, cm: ClickMap) -> Self {
//         Self {
//             visit_record_id: visit.meta.click_id,
//             date: chrono::Local::now().timestamp(),
//             ua: visit.user_agent_data.user_agent_string.clone(),
//             ip: visit.geo_ip_data.ip,
//             click_map: cm,
//         }
//     }
//
//     pub fn get_next_url(&self, referring_url: &str) {}
//
//     pub fn get_initial_url(&self) -> String {
//         match &self.click_map {
//             ClickMap::DL(a) => a.offer_url.to_string(),
//             ClickMap::LP(b) => b.landing_page_url.to_string(),
//             ClickMap::MV(c) => c.root_url.to_string(),
//         }
//     }
// }
