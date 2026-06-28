use crate::data::elements::campaign::Campaign;
use crate::data::elements::funnel::Funnel;
use crate::data::elements::landing_page::LandingPage;
use crate::data::elements::offer::Offer;
use crate::data::elements::offer_source::OfferSource;
use crate::data::elements::traffic_source::TrafficSource;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CRUDElementRequest {
    Create(PrimeElementBuild),
    Update(Vec<PrimeElementBuild>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CRUDElementResponse {
    pub list_of_return_elements: Vec<PrimeElementBuild>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PrimeElementBuild {
    OfferSource(OfferSource),
    Offer(Offer),
    LandingPage(LandingPage),
    Funnel(Funnel),
    TrafficSource(TrafficSource),
    Campaign(Campaign),
}

impl From<OfferSource> for PrimeElementBuild {
    fn from(data: OfferSource) -> Self {
        Self::OfferSource(data)
    }
}
impl From<Offer> for PrimeElementBuild {
    fn from(data: Offer) -> Self {
        Self::Offer(data)
    }
}
impl From<TrafficSource> for PrimeElementBuild {
    fn from(data: TrafficSource) -> Self {
        Self::TrafficSource(data)
    }
}
impl From<LandingPage> for PrimeElementBuild {
    fn from(data: LandingPage) -> Self {
        Self::LandingPage(data)
    }
}
impl From<Funnel> for PrimeElementBuild {
    fn from(data: Funnel) -> Self {
        Self::Funnel(data)
    }
}
impl From<Campaign> for PrimeElementBuild {
    fn from(data: Campaign) -> Self {
        Self::Campaign(data)
    }
}

#[derive(Deserialize, Serialize, Clone, EnumString, ToString)]
pub enum CreatableElement {
    Campaign,
    Offer,
    Lander,
    Funnel,
    #[strum(serialize = "Traffic Source")]
    TrafficSource,
    #[strum(serialize = "Offer Source")]
    OfferSource,
}
