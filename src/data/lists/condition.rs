use crate::data::lists::country::ISOCountry;
use crate::data::lists::time_zone::TimeZone;
use crate::data::lists::DeviceType;
use crate::ISOLanguage;
use std::net::IpAddr;
use std::str::FromStr;
use strum::IntoEnumIterator;
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Condition {
    pub condition_data_type: ConditionDataType,
    pub include: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, EnumString, ToString, EnumIter)]
pub enum ConditionDataType {
    #[strum(serialize = "Brand")]
    Brand(Vec<String>),
    #[strum(serialize = "Browser")]
    Browser(Vec<String>),
    #[strum(serialize = "City")]
    City(Vec<String>),
    #[strum(serialize = "Connection Type")]
    ConnectionType(Vec<ConnectionType>),
    #[strum(serialize = "Country")]
    Country(Vec<ISOCountry>),
    #[strum(serialize = "Variable 1")]
    Variable1(Vec<String>),
    #[strum(serialize = "Variable 2")]
    Variable2(Vec<String>),
    #[strum(serialize = "Variable 3")]
    Variable3(Vec<String>),
    #[strum(serialize = "Variable 4")]
    Variable4(Vec<String>),
    #[strum(serialize = "Variable 5")]
    Variable5(Vec<String>),
    #[strum(serialize = "Variable 6")]
    Variable6(Vec<String>),
    #[strum(serialize = "Variable 7")]
    Variable7(Vec<String>),
    #[strum(serialize = "Variable 8")]
    Variable8(Vec<String>),
    #[strum(serialize = "Variable 9")]
    Variable9(Vec<String>),
    #[strum(serialize = "Variable 10")]
    Variable10(Vec<String>),
    #[strum(serialize = "Device Type")]
    DeviceType(Vec<DeviceType>),
    #[strum(serialize = "IP Range")]
    IP(Vec<(IpAddr, IpAddr)>),
    #[strum(serialize = "ISP")]
    ISP(Vec<String>),
    #[strum(serialize = "Language")]
    Language(Vec<ISOLanguage>),
    #[strum(serialize = "Mobile Carrier")]
    MobileCarrier(Vec<String>),
    #[strum(serialize = "Operating System")]
    OperatingSystem(Vec<String>),
    #[strum(serialize = "Proxy")]
    Proxy(Vec<String>),
    #[strum(serialize = "Referrer")]
    Referrer(Vec<Url>),
    #[strum(serialize = "Referrer Domain")]
    ReferrerDomain(Vec<Url>),
    #[strum(serialize = "Region")]
    Region(Vec<String>),
    #[strum(serialize = "Time of Day")]
    TimeOfDay((String, String, TimeZone)),
    #[strum(serialize = "Unique Visit")]
    UniqueVisit,
    #[strum(serialize = "Weekday")]
    Weekday((Vec<Weekdays>, TimeZone)),
}

#[derive(
    Serialize, Deserialize, Copy, Clone, Debug, EnumString, ToString, EnumIter, Eq, PartialEq,
)]
pub enum ConnectionType {
    Broadband,
    Cable,
    Dialup,
    Mobile,
    Satellite,
    Unknown,
    Wirelress,
    XDSL,
}

#[derive(
    Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, EnumString, ToString, EnumIter,
)]
pub enum Weekdays {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
