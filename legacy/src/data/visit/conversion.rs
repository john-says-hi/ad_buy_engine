use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversion {
    pub postback_url_parameters: HashMap<String, String>,
    pub ip: IpAddr,
    pub user_agent: String,
    pub referrer: String,
    pub postback_timestamp: NaiveDateTime,
}

impl Conversion {
    pub fn return_payout_value(&self) -> Option<Decimal> {
        if let Some(payout) = self.postback_url_parameters.get("payout") {
            if let Ok(payout) = Decimal::from_str(payout) {
                Some(payout)
            } else {
                None
            }
        } else {
            None
        }
    }
}
