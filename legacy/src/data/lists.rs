use std::str::FromStr;
use std::string::ToString;

use strum::IntoEnumIterator;

use crate::AError;
use url::Url;

pub mod click_transition_method;
pub mod condition;
pub mod country;
pub mod referrer_handling;
pub mod time_zone;

#[derive(
    Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, Eq, PartialEq, Debug,
)]
pub enum DeviceType {
    Desktop,
    Laptop,
    Tablet,
    Phone,
    SmartTV,
    GameConsole,
}

#[derive(
    Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, Eq, PartialEq, Debug,
)]
pub enum Language {
    Any,
    English,
    German,
    French,
    Spanish,
}

#[derive(
    Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, Eq, PartialEq, Debug,
)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    CAD,
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl FromStr for TrafficSourceToken {
    type Err = AError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

impl ToString for TrafficSourceToken {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, EnumIter, PartialEq, Debug)]
pub enum TrafficSourceToken {
    Payout,
    ExternalID,
    PayoutCurrency,
    CampaignID,
    CampaignName,
    TrafficSourceID,
    LanderID,
    LanderName,
    OfferID,
    OfferName,
    Device,
    Brand,
    Model,
    Browser,
    BrowserVersion,
    OS,
    OSVersion,
    Country,
    Region,
    City,
    ISP,
    ConnectionType,
    Carrier,
    IP,
    CountryName,
    ReferrerDomain,
    UserAgent,
    TransactionID,
    ClickID,
    Var1,
    Var2,
    Var3,
    Var4,
    Var5,
    Var6,
    Var7,
    Var8,
    Var9,
    Var10,
    ConversionCost,
    EventType,
    ClearanceID,
    ClearanceName,
    PostBackTime,
}

// #[derive(Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, PartialEq, Debug)]
// pub enum DataURLToken {
//     CampaignID,
//     CampaignName,
//     TrafficSourceID,
//     TrafficSourceName,
//     OfferID,
//     OfferName,
//     LanderID,
//     LanderName,
//     Device,
//     Brand,
//     Model,
//     Browser,
//     BrowserVersion,
//     OS,
//     OSVersion,
//     Country,
//     CountryName,
//     City,
//     Region,
//     ISP,
//     UserAgent,
//     IP,
//     TrackingDomain,
//     ReferrerDomain,
//     Language,
//     ConnectionType,
//     Carrier,
// }

#[derive(Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, PartialEq, Debug)]
pub enum DataURLToken {
    #[strum(serialize = "{var1}")]
    Var1,
    #[strum(serialize = "{var2}")]
    Var2,
    #[strum(serialize = "{var3}")]
    Var3,
    #[strum(serialize = "{var4}")]
    Var4,
    #[strum(serialize = "{var5}")]
    Var5,
    #[strum(serialize = "{var6}")]
    Var6,
    #[strum(serialize = "{var7}")]
    Var7,
    #[strum(serialize = "{var8}")]
    Var8,
    #[strum(serialize = "{var9}")]
    Var9,
    #[strum(serialize = "{var10}")]
    Var10,
    #[strum(serialize = "{param5}")]
    Parameter5, //Traffic Source Only
    #[strum(serialize = "{param4}")]
    Parameter4, //Traffic Source Only
    #[strum(serialize = "{param3}")]
    Parameter3, //Traffic Source Only
    #[strum(serialize = "{param2}")]
    Parameter2, //Traffic Source Only
    #[strum(serialize = "{param1}")]
    Parameter1, //Traffic Source Only
    #[strum(serialize = "{transaction_id}")]
    TransactionID, //Traffic Source Only
    #[strum(serialize = "{time_of_postback}")]
    TimeOfPostback, //Traffic Source Only
    #[strum(serialize = "{conversion_cost}")]
    ConversionCost, //Traffic Source Only
    #[strum(serialize = "{custom_event}")]
    CustomEvent, //Traffic Source Only
    #[strum(serialize = "{payout_currency}")]
    PayoutCurrency, //Traffic Source Only
    #[strum(serialize = "{payout}")]
    Payout, //Traffic Source Only
    #[strum(serialize = "{external_id}")]
    ExternalID,
    #[strum(serialize = "{click_id}")]
    ClickID, //Offer Only
    #[strum(serialize = "{funnel_id}")]
    FunnelID, //Offer Only
    #[strum(serialize = "{cost}")]
    Cost, //Offer Only
    #[strum(serialize = "{campaign_id}")]
    CampaignID,
    #[strum(serialize = "{campaign_name}")]
    CampaignName,
    #[strum(serialize = "{traffic_source_id}")]
    TrafficSourceID,
    #[strum(serialize = "{traffic_source_name}")]
    TrafficSourceName,
    #[strum(serialize = "{offer_id}")]
    OfferID,
    #[strum(serialize = "{offer_name}")]
    OfferName,
    #[strum(serialize = "{lander_id}")]
    LanderID,
    #[strum(serialize = "{lander_name}")]
    LanderName,
    #[strum(serialize = "{device_type}")]
    Device,
    #[strum(serialize = "{device_brand}")]
    Brand,
    #[strum(serialize = "{device_model}")]
    Model,
    #[strum(serialize = "{browser}")]
    Browser,
    #[strum(serialize = "{browser_version}")]
    BrowserVersion,
    #[strum(serialize = "{os}")]
    OS,
    #[strum(serialize = "{os_version}")]
    OSVersion,
    #[strum(serialize = "{country}")]
    Country,
    #[strum(serialize = "{country_name}")]
    CountryName,
    #[strum(serialize = "{city}")]
    City,
    #[strum(serialize = "{region}")]
    Region,
    #[strum(serialize = "{isp}")]
    ISP,
    #[strum(serialize = "{user_agent}")]
    UserAgent,
    #[strum(serialize = "{ip}")]
    IP,
    #[strum(serialize = "tracking_domain")]
    TrackingDomain, // EXCLUDE TRAFFIC SOURCE
    #[strum(serialize = "referrer_domain")]
    ReferrerDomain,
    #[strum(serialize = "{language}")]
    Language,
    #[strum(serialize = "{connection_type}")]
    ConnectionType,
    #[strum(serialize = "{carrier}")]
    Carrier,
    #[strum(serialize = "{clearance_level}")]
    ClearanceLevel,
    #[strum(serialize = "{clearance_id}")]
    ClearanceID,
}

#[derive(Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, PartialEq, Debug)]
pub enum CostModel {
    NotTracked,
    CPC,
    CPM,
    CPA,
    RevShare,
    Auto,
}

#[derive(Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, PartialEq, Debug)]
pub enum Vertical {
    Many,
    Adult,
    Android,
    App,
    Apparel,
    AsSeenOnTV,
    Astrology,
    Auctions,
    Automotive,
    Baby,
    Beauty,
    Binary,
    BusinessOpportunity,
    CareersJobs,
    Casino,
    COD,
    Contextual,
    Coupon,
    CPC,
    Credit,
    Crypto,
    Dating,
    Debt,
    Diet,
    Display,
    DOI,
    Downloads,
    Ecom,
    Education,
    Email,
    Entertainment,
    Exclusive,
    Facebook,
    Family,
    Fashion,
    Finance,
    Fitness,
    Food,
    Freebie,
    FreeTrial,
    Gambling,
    Game,
    Gaming,
    Health,
    Holidays,
    Home,
    Incent,
    Install,
    Insurance,
    International,
    Internet,
    iOS,
    Job,
    KPI,
    LeadGen,
    Legal,
    Mobile,
    Mortgage,
    Movies,
    Music,
    Niche,
    Nutra,
    Path,
    Payday,
    PaydayLoan,
    PayPerCall,
    Pets,
    Pin,
    RealEstate,
    Search,
    Seasonal,
    SEO,
    Service,
    Shopping,
    SiteRegistration,
    SmartLink,
    SocialMedia,
    Software,
    SOI,
    Solar,
    Sports,
    Subscription,
    Survey,
    Sweepstake,
    Tech,
    Travel,
    Trial,
    Web,
    ZipSubmit,
}
