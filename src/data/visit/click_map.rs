use either::Either;
use url::Url;
use uuid::Uuid;
use weighted_rs::{SmoothWeight, Weight};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferClickMap {
    pub offer_id: Uuid,
    pub offer_url: Url,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LandingPageClickMap {
    pub landing_page_id: Uuid,
    pub landing_page_url: Url,
    pub offers: Vec<OfferClickMap>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PreLandingPageClickMap {
    pub landing_page_id: Uuid,
    pub pre_landing_page_url: Url,
    pub landing_pages: Vec<LandingPageClickMap>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RedirectInstructions {
    pub live_campaign_id: Uuid,
    pub map: ClickMap,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClickMap {
    Offer(OfferClickMap),
    LandingPage(LandingPageClickMap),
    PreLandingPage(PreLandingPageClickMap),
}

// impl ClickMap {
//     pub fn new(live_campaign_mirror: &LiveCampaign) -> Result<ClickMapResult, anyhow::Error> {
//         let mut sequence_id = None;
//
//         match &live_campaign_mirror.path {
//             Either::Left(a) => {
//                 let mut sw: SmoothWeight<&SlimSequenceMirror> = SmoothWeight::new();
//                 for i in a {
//                     sw.add(&i.sequence, i.weight as isize)
//                 }
//
//                 let select_sequence = &sw.next().unwrap();
//                 sequence_id = Some(select_sequence.sid.clone());
//
//                 match &select_sequence.path_info.0 {
//                     Either::Left(a) => {
//                         if a.landing_pages.is_empty() {
//                             // DL
//                             let mut sw: SmoothWeight<&OfferMirror> = SmoothWeight::new();
//
//                             for i in &a.offer_groups.get(0).expect("WEGRjuj").offers {
//                                 sw.add(&i.mirror, i.weight as isize)
//                             }
//
//                             let selected = sw.next().expect("G#szzf");
//
//                             let clickmap = ClickMap::DL(OfferClickMap {
//                                 offer_id: selected.oid,
//                                 offer_url: selected.url.clone(),
//                             });
//
//                             Ok(ClickMapResult {
//                                 click_map: clickmap,
//                                 sequence_id,
//                             })
//                         } else {
//                             let mut sw: SmoothWeight<&LandingPageMirror> = SmoothWeight::new();
//
//                             for i in a.landing_pages.iter() {
//                                 sw.add(&i.mirror, i.weight as isize)
//                             }
//
//                             let selected_landing_page = sw.next().expect("HG4wx");
//
//                             // Select Offer
//                             if a.offer_groups.len() == 1 {
//                                 let mut sw: SmoothWeight<&OfferMirror> = SmoothWeight::new();
//
//                                 for i in a.offer_groups.get(0).expect("G#$w").offers.iter() {
//                                     sw.add(&i.mirror, i.weight as isize)
//                                 }
//
//                                 let selected_offer = sw.next().expect("GF#szcc");
//
//                                 let offer_s = vec![OfferClickMap {
//                                     offer_id: selected_offer.oid,
//                                     offer_url: selected_offer.url.clone(),
//                                 }];
//
//                                 let click_map = ClickMap::LP(LandingPageClickMap {
//                                     landing_page_id: selected_landing_page.lpid,
//                                     landing_page_url: selected_landing_page.url.clone(),
//                                     offer_groups: offer_s,
//                                 });
//
//                                 Ok(ClickMapResult {
//                                     click_map,
//                                     sequence_id,
//                                 })
//                             } else {
//                                 // Many Offer Group
//                                 // LP
//                                 let mut sw: SmoothWeight<&LandingPageMirror> = SmoothWeight::new();
//
//                                 for i in a.landing_pages.iter() {
//                                     sw.add(&i.mirror, i.weight as isize)
//                                 }
//
//                                 let selected_landing_page = sw.next().expect("ejtj");
//
//                                 // Offer Groups
//                                 let mut offer_s: Vec<OfferClickMap> = vec![];
//
//                                 for group in a.offer_groups.iter() {
//                                     let mut sw: SmoothWeight<&OfferMirror> = SmoothWeight::new();
//
//                                     for i in group.offers.iter() {
//                                         sw.add(&i.mirror, i.weight as isize)
//                                     }
//
//                                     let selected_offer = sw.next().expect("G#s788");
//
//                                     offer_s.push(OfferClickMap {
//                                         offer_id: selected_offer.oid,
//                                         offer_url: selected_offer.url.clone(),
//                                     })
//                                 }
//
//                                 let click_map = ClickMap::LP(LandingPageClickMap {
//                                     landing_page_id: selected_landing_page.lpid,
//                                     landing_page_url: selected_landing_page.url.clone(),
//                                     offer_groups: offer_s,
//                                 });
//
//                                 Ok(ClickMapResult {
//                                     click_map,
//                                     sequence_id,
//                                 })
//                             }
//                         }
//                     }
//                     Either::Right(b) => {
//                         //MV
//                         // let mut sw:SmoothWeight<&MultiVectorPath>
//                         Err(anyhow::Error::msg("mv not setup"))
//                     }
//                 }
//                 // Ok()
//             }
//             Either::Right(b) => Err(anyhow::Error::msg("core not setup")),
//         }
//     }
// }
