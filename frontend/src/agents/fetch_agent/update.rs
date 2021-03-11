// use crate::agents::fetch_agent::Msg;
// use crate::agents::FetchAgent;
// use crate::prelude::*;
// use ad_buy_engine::constant::API_URL_UPDATE_OFFER_SOURCE;
// use ad_buy_engine::data_state_logic_models::element::CampaignElement;
// use ad_buy_engine::data_state_logic_models::models::EitherCampaignElement;
// use ad_buy_engine::AError;
// use yew::format::Json;
// use yew_services::fetch::{FetchTask, Request, Response};
// use yew_services::FetchService;
//
// impl FetchAgent {
//     pub fn update(&mut self, new_campaign_element: CampaignElement) -> Option<FetchTask> {
//         match new_campaign_element {
//             CampaignElement::OfferSource(mirror) => {
//                 let request = Request::post(API_URL_UPDATE_OFFER_SOURCE)
//                     .header("Content-Type", "application/json")
//                     .body(Json(&mirror))
//                     .unwrap();
//
//                 let callback = self.link.callback(
//                     move |res: Response<Json<Result<EitherCampaignElement, AError>>>| {
//                         let (meta, Json(data)) = res.into_parts();
//
//                         if meta.status.is_success() {
//                             if let Ok(mirror) = data {
//                                 Msg::SyncFetchedElement(mirror)
//                             } else {
//                                 Msg::FetchError(vec!["json deserialize failed".to_string()])
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
