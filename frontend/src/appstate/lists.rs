use crate::utils::routes::AppRoute;
use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;

// pub type SelectableElement = ReportableElement;
//
// pub enum ReportableElement {
//     OfferSource,
//     Offer,
//     TrafficSource,
//     LandingPage,
//     Funnel,
//     Campaign,
// }

#[derive(Deserialize, Serialize, Clone, EnumString, ToString, PartialEq)]
pub enum ReportDateRange {
    #[strum(serialize = "Today")]
    Today,
    #[strum(serialize = "Yesterday")]
    Yesterday,
    #[strum(serialize = "Past 3 Days")]
    ThreeDays,
    #[strum(serialize = "Past 7 Days")]
    SevenDays,
    #[strum(serialize = "Past 14 Days")]
    FourteenDays,
    #[strum(serialize = "Past 30 Days")]
    ThirtyDays,
    #[strum(serialize = "Past 6 Months")]
    SixMonths,
    #[strum(serialize = "Custom Range")]
    CustomRange,
    #[strum(serialize = "All of Time")]
    All,
}

impl Default for ReportDateRange {
    fn default() -> Self {
        ReportDateRange::Today
    }
}

#[derive(Deserialize, Serialize, Clone, EnumString, ToString, PartialEq)]
pub enum RowLimitOptions {
    #[strum(serialize = "50")]
    Fifty,
    #[strum(serialize = "100")]
    Hundred,
    #[strum(serialize = "200")]
    TwoHundred,
    #[strum(serialize = "500")]
    FiveHundred,
    #[strum(serialize = "1000")]
    OneThousand,
}

impl Default for RowLimitOptions {
    fn default() -> Self {
        RowLimitOptions::Fifty
    }
}

#[derive(Deserialize, Serialize, Clone, EnumString, ToString, PartialEq)]
pub enum FilterElementOptions {
    #[strum(serialize = "All")]
    All,
    #[strum(serialize = "Archived")]
    Archived,
    #[strum(serialize = "Has Traffic")]
    HasTraffic,
    #[strum(serialize = "Active")]
    Active,
}

impl Default for FilterElementOptions {
    fn default() -> Self {
        FilterElementOptions::All
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, Eq, PartialEq, Debug)]
pub enum PrimeElement {
    #[strum(serialize = "Drill Down")]
    Nothing,
    #[strum(serialize = "Traffic Sources")]
    TrafficSources,
    #[strum(serialize = "Offer Sources")]
    OfferSources,
    Brands,
    #[strum(serialize = "Browser Versions")]
    BrowserVersions,
    Browsers,
    Campaigns,
    #[strum(serialize = "Connection Types")]
    ConnectionTypes,
    Conversions,
    Countries,
    Day,
    #[strum(serialize = "Day of Week")]
    DayOfWeek,
    #[strum(serialize = "Hour of Day")]
    HourOfDay,
    #[strum(serialize = "Device Types")]
    DeviceTypes,
    Sequences,
    Funnels,
    #[strum(serialize = "ISP / Carriers")]
    ISPCarrier,
    Landers,
    #[strum(serialize = "Mobile Carriers")]
    MobileCarrier,
    Models,
    Month,
    OS,
    #[strum(serialize = "OS Versions")]
    OSVersions,
    Offers,
    Proxies,
}

impl Default for PrimeElement {
    fn default() -> Self {
        PrimeElement::Nothing
    }
}

impl From<AppRoute> for PrimeElement {
    fn from(route: AppRoute) -> Self {
        match route {
            AppRoute::Campaign => PrimeElement::Campaigns,
            AppRoute::Offers => PrimeElement::Offers,
            AppRoute::Sequences => PrimeElement::Sequences,
            AppRoute::Funnels => PrimeElement::Funnels,
            AppRoute::Traffic => PrimeElement::TrafficSources,
            AppRoute::OfferSources => PrimeElement::OfferSources,
            AppRoute::Landers => PrimeElement::Landers,
            AppRoute::OSVersion => PrimeElement::OSVersions,
            AppRoute::OS => PrimeElement::OS,
            AppRoute::DayOfWeek => PrimeElement::DayOfWeek,
            AppRoute::HourOfDay => PrimeElement::HourOfDay,
            AppRoute::DateMonth => PrimeElement::Month,
            AppRoute::DateDay => PrimeElement::Day,
            AppRoute::BrowserVersion => PrimeElement::BrowserVersions,
            AppRoute::Browser => PrimeElement::Browsers,
            AppRoute::Model => PrimeElement::Models,
            AppRoute::Brand => PrimeElement::Brands,
            AppRoute::Devices => PrimeElement::DeviceTypes,
            AppRoute::Proxy => PrimeElement::Proxies,
            AppRoute::MobileCarrier => PrimeElement::MobileCarrier,
            AppRoute::ISPCarrier => PrimeElement::ISPCarrier,
            AppRoute::Connection => PrimeElement::ConnectionTypes,
            AppRoute::Conversions => PrimeElement::Conversions,
            _ => PrimeElement::Nothing,
        }
    }
}
