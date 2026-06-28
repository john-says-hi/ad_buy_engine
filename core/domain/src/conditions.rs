use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    Country,
    Region,
    City,
    Isp,
    ConnectionType,
    ProxyType,
    Carrier,
    Browser,
    OperatingSystem,
    DeviceType,
    DeviceBrand,
    Language,
    QueryParameter,
    Referrer,
    ReferrerDomain,
    IpRange,
    Weekday,
    TimeWindow,
    UniqueVisit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Include,
    Exclude,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionRule {
    pub id: String,
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    pub key: Option<String>,
    pub values: Vec<String>,
    pub timezone: Option<String>,
    pub start_minute_of_day: Option<u16>,
    pub end_minute_of_day: Option<u16>,
    pub active: bool,
}

impl ConditionRule {
    pub fn query_parameter(
        id: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            condition_type: ConditionType::QueryParameter,
            operator: ConditionOperator::Include,
            key: Some(key.into()),
            values: vec![value.into()],
            timezone: None,
            start_minute_of_day: None,
            end_minute_of_day: None,
            active: true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClickContext {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub referrer_domain: Option<String>,
    pub query: Vec<TokenValue>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub isp: Option<String>,
    pub connection_type: Option<String>,
    pub proxy_type: Option<String>,
    pub carrier: Option<String>,
    pub browser: Option<String>,
    pub operating_system: Option<String>,
    pub device_type: Option<String>,
    pub device_brand: Option<String>,
    pub language: Option<String>,
    pub weekday: Option<String>,
    pub minute_of_day: Option<u16>,
    pub is_unique_visit: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenValue {
    pub key: String,
    pub value: String,
}
