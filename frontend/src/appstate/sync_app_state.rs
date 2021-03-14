// use crate::appstate::app_state::AppState;
// use crate::notify_primary;
// use ad_buy_engine::constant::browser_storage_keys::{
//     CAMPAIGNS_KEY, FUNNELS_KEY, LANDING_PAGES_KEY, OFFERS_KEY, OFFER_SOURCES, TRAFFIC_SOURCES_KEY,
// };
// use ad_buy_engine::data::elements::crud::{
//     CRUDElementResponse, CreatableElement, PrimeElementBuild,
// };
// use yew::format::Json;
// use yew_services::storage::Area;
// use yew_services::StorageService;
//
// impl AppState {
//     pub fn crud_update(&self, crud_response: CRUDElementResponse) {
//         let mut list = crud_response.list_of_return_elements.iter();
//
//         while let Some(data) = list.next() {
//             match data {
//                 PrimeElementBuild::OfferSource(elem) => {
//                     let mut local_state = self.offer_sources.borrow_mut();
//
//                     if let Some(pos) = local_state
//                         .iter()
//                         .position(|s| s.offer_source_id == elem.offer_source_id)
//                     {
//                         local_state.remove(pos);
//                         local_state.insert(pos, elem.clone());
//                     } else {
//                         local_state.push(elem.clone());
//                     }
//                     StorageService::new(Area::Local)
//                         .expect("f43sa")
//                         .store(OFFER_SOURCES, Json(&*local_state))
//                 }
//
//                 PrimeElementBuild::Offer(elem) => {
//                     let mut local_state = self.offers.borrow_mut();
//                     if let Some(pos) = local_state.iter().position(|s| s.offer_id == elem.offer_id)
//                     {
//                         local_state.remove(pos);
//                         local_state.insert(pos, elem.clone());
//                     } else {
//                         local_state.push(elem.clone())
//                     }
//                     StorageService::new(Area::Local)
//                         .expect("f43sa")
//                         .store(OFFERS_KEY, Json(&*local_state))
//                 }
//                 PrimeElementBuild::LandingPage(elem) => {
//                     let mut local_state = self.landing_pages.borrow_mut();
//
//                     if let Some(pos) = local_state
//                         .iter()
//                         .position(|s| s.landing_page_id == elem.landing_page_id)
//                     {
//                         local_state.remove(pos);
//                         local_state.insert(pos, elem.clone());
//                     } else {
//                         local_state.push(elem.clone())
//                     }
//                     StorageService::new(Area::Local)
//                         .expect("f43sa")
//                         .store(LANDING_PAGES_KEY, Json(&*local_state))
//                 }
//                 PrimeElementBuild::TrafficSource(elem) => {
//                     let mut local_state = self.traffic_sources.borrow_mut();
//
//                     if let Some(pos) = local_state
//                         .iter()
//                         .position(|s| s.traffic_source_id == elem.traffic_source_id)
//                     {
//                         local_state.remove(pos);
//                         local_state.insert(pos, elem.clone());
//                     } else {
//                         local_state.push(elem.clone())
//                     }
//                     StorageService::new(Area::Local)
//                         .expect("f43sa")
//                         .store(TRAFFIC_SOURCES_KEY, Json(&*local_state))
//                 }
//                 PrimeElementBuild::Funnel(elem) => {
//                     let mut local_state = self.funnels.borrow_mut();
//
//                     if let Some(pos) = local_state
//                         .iter()
//                         .position(|s| s.funnel_id == elem.funnel_id)
//                     {
//                         local_state.remove(pos);
//                         local_state.insert(pos, elem.clone());
//                     } else {
//                         local_state.push(elem.clone())
//                     }
//                     StorageService::new(Area::Local)
//                         .expect("f43sa")
//                         .store(FUNNELS_KEY, Json(&*local_state))
//                 }
//                 PrimeElementBuild::Campaign(elem) => {
//                     let mut local_state = self.campaigns.borrow_mut();
//
//                     if let Some(pos) = local_state
//                         .iter()
//                         .position(|s| s.campaign_id == elem.campaign_id)
//                     {
//                         local_state.remove(pos);
//                         local_state.insert(pos, elem.clone());
//                     } else {
//                         local_state.push(elem.clone())
//                     }
//                     StorageService::new(Area::Local)
//                         .expect("f43sa")
//                         .store(CAMPAIGNS_KEY, Json(&*local_state))
//                 }
//             }
//         }
//     }
// }
