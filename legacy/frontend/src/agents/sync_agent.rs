// use crate::alert;
// use crate::prelude::*;
// use crate::appstate::app_state::AppState;
// use ad_buy_engine::data_state_logic_models::element::{CampaignElement, CampaignElements};
// use ad_buy_engine::data_state_logic_models::models::EitherCampaignElement;
// use either::Either;
// use std::rc::Rc;
// use yew::agent::*;
//
// pub mod many_elements;
// pub mod single_element;
//
// #[derive(Deserialize, Serialize, Clone, PartialEq)]
// pub enum SyncAction {
//     GetAppState,
//     SyncClickData,
//     SyncCampaignElement(EitherCampaignElement),
// }
//
// #[derive(Serialize, Deserialize)]
// pub enum SyncRequest {
//     Sync(SyncAction),
// }
//
// pub struct SyncAgent {
//     pub link: AgentLink<SyncAgent>,
//     pub app_state: Rc<AppState>,
// }
//
// impl Agent for SyncAgent {
//     type Reach = Context<Self>;
//     type Message = ();
//     type Input = SyncRequest;
//     type Output = Rc<AppState>;
//
//     fn create(link: AgentLink<Self>) -> Self {
//         Self {
//             link,
//             app_state: Rc::new(AppState::init()),
//         }
//     }
//
//     fn update(&mut self, _msg: Self::Message) {}
//
//     fn handle_input(&mut self, msg: Self::Input, hid: HandlerId) {
//         match msg {
//             SyncRequest::Sync(action) => match action {
//                 // SyncAction::SyncCampaignElement(either) => match either.campaign_element {
//                 //     Either::Left(element) => match element {
//                 //         CampaignElement::OfferSource(mirror) => {
//                 //             // if let Some(pos) = self
//                 //             //     .app_state
//                 //             //     .offer_sources
//                 //             //     .borrow()
//                 //             //     .iter()
//                 //             //     .position(|s| s.id == mirror.id)
//                 //             // {
//                 //             //     let mut handle = self.app_state.offer_sources.borrow_mut();
//                 //             //     handle.remove(pos);
//                 //             //     handle.insert(pos, mirror);
//                 //             // } else {
//                 //             //     self.app_state.offer_sources.borrow_mut().push(mirror)
//                 //             // }
//                 //         }
//                 //         _ => {}
//                 //     },
//                 //     Either::Right(elements) => match elements {
//                 //         CampaignElements::OfferSources(mirrors) => {
//                 //             // for mirror in mirrors {
//                 //             //     if let Some(pos) = self
//                 //             //         .app_state
//                 //             //         .offer_sources
//                 //             //         .borrow()
//                 //             //         .iter()
//                 //             //         .position(|s| s.id == mirror.id)
//                 //             //     {
//                 //             //         let mut handle = self.app_state.offer_sources.borrow_mut();
//                 //             //         handle.remove(pos);
//                 //             //         handle.insert(pos, mirror);
//                 //             //     } else {
//                 //             //         self.app_state.offer_sources.borrow_mut().push(mirror)
//                 //             //     }
//                 //             }
//                 //         }
//                 //         _ => {}
//                 //     },
//                 // },
//                 //
//                 // SyncAction::GetAppState => self.link.respond(hid, Rc::clone(&self.app_state)),
//                 _ => {}
//             },
//         }
//     }
// }
