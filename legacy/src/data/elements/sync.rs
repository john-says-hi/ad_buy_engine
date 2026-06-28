use crate::data::elements::campaign::Campaign;
use crate::data::elements::funnel::Funnel;
use crate::data::elements::landing_page::LandingPage;
use crate::data::elements::offer::Offer;
use crate::data::elements::offer_source::OfferSource;
use crate::data::elements::traffic_source::TrafficSource;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct  SyncElementUnit {
    pub id:Uuid,
    pub last_updated:DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct  SyncElementRequest {
    pub offer_sources:Vec<SyncElementUnit>,
    pub offers:Vec<SyncElementUnit>,
    pub landers:Vec<SyncElementUnit>,
    pub funnels:Vec<SyncElementUnit>,
    pub traffic_sources:Vec<SyncElementUnit>,
    pub campaigns:Vec<SyncElementUnit>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncElementResponse {
    pub offer_sources:Vec<SyncFlagData<OfferSource>>,
    pub offers:Vec<SyncFlagData<Offer>>,
    pub landers:Vec<SyncFlagData<LandingPage>>,
    pub traffic_sources:Vec<SyncFlagData<TrafficSource>>,
    pub funnels:Vec<SyncFlagData<Funnel>>,
    pub campaigns:Vec<SyncFlagData<Campaign>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SyncFlagData<T> {
    Remove(Uuid),
    Update(T),
    Insert(T),
    Nothing,
}

// impl From<OfferSource> for PrimeElementBuild {
//     fn from(data: OfferSource) -> Self {
//         Self::OfferSource(data)
//     }
// }
// impl From<Offer> for PrimeElementBuild {
//     fn from(data: Offer) -> Self {
//         Self::Offer(data)
//     }
// }
// impl From<TrafficSource> for PrimeElementBuild {
//     fn from(data: TrafficSource) -> Self {
//         Self::TrafficSource(data)
//     }
// }
// impl From<LandingPage> for PrimeElementBuild {
//     fn from(data: LandingPage) -> Self {
//         Self::LandingPage(data)
//     }
// }
// impl From<Funnel> for PrimeElementBuild {
//     fn from(data: Funnel) -> Self {
//         Self::Funnel(data)
//     }
// }
// impl From<Campaign> for PrimeElementBuild {
//     fn from(data: Campaign) -> Self {
//         Self::Campaign(data)
//     }
// }
//
// #[derive(Deserialize, Serialize, Clone, EnumString, ToString)]
// pub enum CreatableElement {
//     Campaign,
//     Offer,
//     Lander,
//     Funnel,
//     #[strum(serialize = "Traffic Source")]
//     TrafficSource,
//     #[strum(serialize = "Offer Source")]
//     OfferSource,
// }
