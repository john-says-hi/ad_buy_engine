use crate::components::main_component::MainComponent;
pub mod route_helpers;

use crate::appstate::lists::PrimeElement;
use ad_buy_engine::data::elements::crud::CreatableElement;
use std::borrow::BorrowMut;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::AllowMissing;
use yew_router::{prelude::*, Switch};
use yew_services::storage::Area;
use yew_services::StorageService;

#[derive(Debug, Switch, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppRoute {
    #[to = "/secure/#fuel"]
    Fuel,
    #[to = "/secure/#engine"]
    Engine,
    #[to = "/secure/#dashboard"]
    Dashboard,
    #[to = "/secure/#offer_sources"]
    OfferSources,
    #[to = "/secure/#offers"]
    Offers,
    #[to = "/secure/#traffic_sources"]
    Traffic,
    #[to = "/secure/#landing_pages"]
    Landers,
    #[to = "/secure/#sequence"]
    Sequences,
    #[to = "/secure/#cores"]
    Core,
    #[to = "/secure/#funnels"]
    Funnels,
    #[to = "/secure/#campaigns"]
    Campaign,
    #[to = "/secure/#404"]
    FourZeroFour,
    #[to = "/secure/#connection"]
    Connection,
    #[to = "/secure/#isp_carrier"]
    ISPCarrier,
    #[to = "/secure/#mobile_carrier"]
    MobileCarrier,
    #[to = "/secure/#proxy"]
    Proxy,
    #[to = "/secure/#devices"]
    Devices,
    #[to = "/secure/#brand"]
    Brand,
    #[to = "/secure/#db"]
    Model,
    #[to = "/secure/#os"]
    OS,
    #[to = "/secure/#os_version"]
    OSVersion,
    #[to = "/secure/#browser"]
    Browser,
    #[to = "/secure/#browser_version"]
    BrowserVersion,
    #[to = "/secure/#date_day"]
    DateDay,
    #[to = "/secure/#date_month"]
    DateMonth,
    #[to = "/secure/#hour_of_day"]
    HourOfDay,
    #[to = "/secure/#day_of_week"]
    DayOfWeek,
    #[to = "/secure/#account"]
    Account,
    #[to = "/secure/#conversions"]
    Conversions,
    #[to = "/secure/#custom_conversions"]
    CustomConversions,
    #[to = "/secure/#referrer_handling"]
    ReferrerHandling,
}

impl Default for AppRoute {
    fn default() -> Self {
        AppRoute::Campaign
    }
}

impl From<PrimeElement> for AppRoute {
    fn from(first_grouping_column: PrimeElement) -> Self {
        match first_grouping_column {
            PrimeElement::Campaigns => AppRoute::Campaign,
            PrimeElement::Offers => AppRoute::Offers,
            PrimeElement::Funnels => AppRoute::Funnels,
            PrimeElement::TrafficSources => AppRoute::Traffic,
            PrimeElement::OfferSources => AppRoute::OfferSources,
            PrimeElement::Landers => AppRoute::Landers,
            PrimeElement::OSVersions => AppRoute::OSVersion,
            PrimeElement::OS => AppRoute::OS,
            PrimeElement::DayOfWeek => AppRoute::DayOfWeek,
            PrimeElement::HourOfDay => AppRoute::HourOfDay,
            PrimeElement::Month => AppRoute::DateMonth,
            PrimeElement::Day => AppRoute::DateDay,
            PrimeElement::BrowserVersions => AppRoute::BrowserVersion,
            PrimeElement::Browsers => AppRoute::Browser,
            PrimeElement::Models => AppRoute::Model,
            PrimeElement::Brands => AppRoute::Brand,
            PrimeElement::DeviceTypes => AppRoute::Devices,
            PrimeElement::Proxies => AppRoute::Proxy,
            PrimeElement::MobileCarrier => AppRoute::MobileCarrier,
            PrimeElement::ISPCarrier => AppRoute::ISPCarrier,
            PrimeElement::ConnectionTypes => AppRoute::Connection,
            PrimeElement::Conversions => AppRoute::Conversions,
            _ => AppRoute::FourZeroFour,
        }
    }
}

impl From<AppRoute> for CreatableElement {
    fn from(app_route: AppRoute) -> Self {
        match app_route {
            AppRoute::Campaign => CreatableElement::Campaign,
            AppRoute::Offers => CreatableElement::Offer,
            AppRoute::Landers => CreatableElement::Lander,
            AppRoute::Funnels => CreatableElement::Funnel,
            AppRoute::Traffic => CreatableElement::TrafficSource,
            AppRoute::OfferSources => CreatableElement::OfferSource,
            _ => CreatableElement::Campaign,
        }
    }
}
