// use crate::agents::fetch_agent::Msg;
// use crate::agents::FetchAgent;
// use crate::alert;
// use crate::utils::uikit::NotificationStatus;
// use ad_buy_engine::constant::{API_URL_CREATE_OFFER_SOURCE, API_URL_TEAM_GET_ID, ACCOUNT_ID_KEY};
// use ad_buy_engine::data_state_logic_models::element::NewCampaignElement;
// use ad_buy_engine::data_state_logic_models::models::EitherCampaignElement;
// use ad_buy_engine::AError;
// use uuid::Uuid;
// use yew::format::{Json, Nothing};
// use yew_services::fetch::{FetchTask, Request, Response};
// use yew_services::storage::Area;
// use yew_services::{FetchService, StorageService};
//
// impl FetchAgent {
//     pub fn get_team_info(&self) -> Option<FetchTask> {
//         let request = Request::get(API_URL_TEAM_GET_ID).body(Nothing).unwrap();
//
//         let callback = self
//             .link
//             .callback(move |res: Response<Json<Result<Uuid, AError>>>| {
//                 if let (meta, Json(Ok(data))) = res.into_parts() {
//                     if !meta.status.is_success() {
//                         if let Ok(mut area) = StorageService::new(Area::Local) {
//                             area.store(ACCOUNT_ID_KEY, Json(&data));
//                         } else {
//                             alert("Storage service error", NotificationStatus::Danger)
//                         }
//                     } else {
//                         alert("Fetch failed to get team id", NotificationStatus::Danger);
//                     }
//                 } else {
//                     alert("Corrupt response data_types", NotificationStatus::Danger);
//                 }
//                 Msg::Ignore
//             });
//
//         Some(FetchService::fetch(request, callback).expect("G#sd"))
//     }
//
//     pub fn create(&mut self, new_campaign_element: NewCampaignElement) -> Option<FetchTask> {
//         match new_campaign_element {
//             NewCampaignElement::OfferSource(request_obj) => {
//                 let request = Request::post(API_URL_CREATE_OFFER_SOURCE)
//                     .header("Content-Type", "application/json")
//                     .body(Json(&request_obj))
//                     .unwrap();
//
//                 let callback = self.link.callback(
//                     move |res: Response<Json<Result<EitherCampaignElement, AError>>>| {
//                         let (meta, Json(data)) = res.into_parts();
//
//                         if meta.status.is_success() {
//                             if let Ok(element) = data {
//                                 Msg::SyncFetchedElement(element)
//                             } else {
//                                 Msg::FetchError(vec!["Json deserialize failed".to_string()])
//                             }
//                         } else {
//                             Msg::FetchError(vec!["meta status failed".to_string()])
//                         }
//                     },
//                 );
//
//                 Some(FetchService::fetch(request, callback).expect("G#sd"))
//             }
//             _ => None,
//         }
//     }
// }
