// use crate::prelude::*;
// use crate::appstate::app_state::AppState;
// use ad_buy_engine::data_state_logic_models::element::ElementType;
// use ad_buy_engine::AError;
// use boyer_moore_magiclen::BMByte;
// use std::rc::Rc;
// use yew::agent::*;
//
// #[derive(Serialize, Deserialize)]
// pub enum Request {
//     Filter(FilterElementsRequest),
// }
//
// pub struct FilterAgent {
//     link: AgentLink<FilterAgent>,
// }
//
// impl Agent for FilterAgent {
//     type Reach = Context<Self>;
//     type Message = ();
//     type Input = Request;
//     type Output = FilterAgentResponse;
//
//     fn create(link: AgentLink<Self>) -> Self {
//         Self { link }
//     }
//
//     fn update(&mut self, _msg: Self::Message) {}
//
//     fn handle_input(&mut self, msg: Self::Input, hid: HandlerId) {
//         match msg {
//             Request::Filter(req) => match req.elem_type {
//                 ElementType::OfferSource => match req.filter_by {
//                     FilterBy::Active => {
//                         // let list = req
//                         //     .app_state
//                         //     .offer_sources
//                         //     .borrow()
//                         //     .iter()
//                         //     .enumerate()
//                         //     .filter(|(idx, s)| s.archived == false)
//                         //     .map(|(idx, s)| idx)
//                         //     .collect::<Vec<usize>>();
//
//                         // self.link.respond(
//                         //     hid,
//                         //     FilterAgentResponse {
//                         //         items: Rc::new(list),
//                         //     },
//                         // );
//                     }
//                     FilterBy::Archived => {
//                         // let list = req
//                         //     .app_state
//                         //     .offer_sources
//                         //     .borrow()
//                         //     .iter()
//                         //     .enumerate()
//                         //     .filter(|(i, s)| s.archived == true)
//                         //     .map(|(i, s)| i)
//                         //     .collect::<Vec<usize>>();
//
//                         // self.link.respond(
//                         //     hid,
//                         //     FilterAgentResponse {
//                         //         items: Rc::new(list),
//                         //     },
//                         // )
//                     }
//                     FilterBy::Name(name) => {
//                         // let items = req
//                         //     .app_state
//                         //     .offer_sources
//                         //     .borrow()
//                         //     .iter()
//                         //     .enumerate()
//                         //     .filter(|(i, s)| {
//                         //         search_find_match(&name, s.name.as_str()).expect("G3xz")
//                         //     })
//                         //     .map(|(i, s)| i)
//                         //     .collect::<Vec<usize>>();
//
//                         // self.link.respond(
//                         //     hid,
//                         //     FilterAgentResponse {
//                         //         items: Rc::new(items),
//                         //     },
//                         // )
//                     }
//                     _ => {}
//                 },
//                 _ => {}
//             },
//         }
//     }
// }
//
// pub fn search_find_match(pattern_to_match: &str, text: &str) -> Result<bool, AError> {
//     let ptm = BMByte::from(pattern_to_match);
//
//     if ptm.is_none() {
//         return Err(AError::msg("Invalid Pattern"));
//     };
//
//     if ptm.unwrap().find_first_in(text).is_some() {
//         Ok(true)
//     } else {
//         Ok(false)
//     }
// }
//
// #[derive(Deserialize, Serialize, Clone, PartialEq)]
// pub enum FilterBy {
//     Active,
//     Archived,
//     Name(String),
// }
//
// #[derive(Deserialize, Serialize, Clone, PartialEq)]
// pub struct FilterAgentResponse {
//     pub items: Rc<Vec<usize>>,
// }
//
// #[derive(Deserialize, Serialize, Clone)]
// pub struct FilterElementsRequest {
//     pub filter_by: FilterBy,
//     pub elem_type: ElementType,
//     pub app_state: Rc<AppState>,
// }
