pub mod traffic_source_postback_url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomConversionEventToken {
    pub event: CustomConversionEvent,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CustomConversionEvent {
    pub include_in_conversions_column: bool,
    pub include_in_revenue_column: bool,
    pub send_postback_to_traffic_source: bool,
    pub include_in_cost_column: bool,
    pub name: String,
    pub parameter: String,
}
//
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct CustomConversionEvent {
//     pub name: String,
//     pub parameter_values: String,
// }
