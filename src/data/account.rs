use uuid::Uuid;

use billing::BillingInformation;

use crate::constant::utility::UUID_PLACEHOLDER;
use crate::data::account::domains_configuration::{DomainsConfiguration, RootDomainConfiguration};
use crate::data::custom_events::CustomConversionEvent;
use crate::data::lists::referrer_handling::{ReferrerHandling, ReplaceReferrerList};
use crate::data::lists::time_zone::TimeZone;
use crate::data::lists::{Currency, Language};
use crate::data::user::{SlimUser, User};
use crate::data::work_space::{AdditionalUser, WorkSpace};
use crate::AError;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use std::str::FromStr;
use url::Url;

pub mod billing;
pub mod domains_configuration;

pub fn generate_subdomain() -> String {
    use rand::Rng;
    use rnglib::{Language, RNG};
    let num = rand::thread_rng().gen_range(0, 99);

    let rng = RNG::new(&Language::Elven).unwrap();
    let first_name = rng.generate_name();
    let last_name = rng.generate_name();
    format!("{}{}{}", first_name, num, last_name)
}

impl From<User> for Account {
    fn from(user: User) -> Account {
        Self {
            account_id: user.account_id,
            report_time_zone: TimeZone::UTC,
            billing_currency: Currency::USD,
            sys_language: Language::English,
            domains_configuration: DomainsConfiguration::create(),
            work_spaces: vec![],
            fuel: 0,
            conversion_registration_time_reporting:
                ConversionRegistrationTimeReporting::VisitTimestamp,
            default_home_screen: DefaultHomeScreen::Dashboard,
            default_way_to_open_report: DefaultWayToOpenReport::GoToReport,
            ip_anonymization: false,
            default_reporting_currency: Currency::USD,
            profile_first_name: "".to_string(),
            profile_last_name: "".to_string(),
            primary_user: SlimUser {
                user_id: user.user_id,
                email: user.email,
            },
            additional_users: vec![],
            skype: "".to_string(),
            phone_number: "".to_string(),
            two_factor_authentication: None,
            api_access_keys: vec![],
            billing_information: None,
            custom_conversions: vec![],
            referrer_handling_list: vec![],
            last_updated: Utc::now(),
        }
    }
}

impl ToString for Account {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
impl FromStr for Account {
    type Err = AError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(res) = serde_json::from_str(&string) {
            Ok(res)
        } else {
            Err(AError::msg("fatrda"))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub account_id: Uuid,
    pub report_time_zone: TimeZone,
    pub billing_currency: Currency,
    pub sys_language: Language,
    pub domains_configuration: DomainsConfiguration,
    pub work_spaces: Vec<WorkSpace>,
    pub fuel: u64,
    pub conversion_registration_time_reporting: ConversionRegistrationTimeReporting,
    pub default_home_screen: DefaultHomeScreen,
    pub default_way_to_open_report: DefaultWayToOpenReport,
    pub ip_anonymization: bool,
    pub default_reporting_currency: Currency,
    pub profile_first_name: String,
    pub profile_last_name: String,
    pub primary_user: SlimUser,
    pub additional_users: Vec<AdditionalUser>,
    pub skype: String,
    pub phone_number: String,
    pub two_factor_authentication: Option<TwoFactorAuthentication>,
    pub api_access_keys: Vec<String>,
    pub billing_information: Option<BillingInformation>,
    pub custom_conversions: Vec<CustomConversionEvent>,
    pub referrer_handling_list: Vec<ReplaceReferrerList>,
    pub last_updated: DateTime<Utc>,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            account_id: Uuid::parse_str(UUID_PLACEHOLDER).expect("54g"),
            report_time_zone: TimeZone::UTC,
            billing_currency: Currency::USD,
            sys_language: Language::Any,
            domains_configuration: DomainsConfiguration {
                subdomain: "https://google.com".to_string(),
                main_domain: Url::parse("https://google.com").expect("gasd"),
                dedicated_domains: vec![],
                custom_domain_names: vec![],
                root_domain_configuration: RootDomainConfiguration::FourZeroFour,
            },
            work_spaces: vec![],
            fuel: 0,
            conversion_registration_time_reporting:
                ConversionRegistrationTimeReporting::VisitTimestamp,
            default_home_screen: DefaultHomeScreen::Dashboard,
            default_way_to_open_report: DefaultWayToOpenReport::GoToReport,
            ip_anonymization: false,
            default_reporting_currency: Currency::USD,
            profile_first_name: "".to_string(),
            profile_last_name: "".to_string(),
            primary_user: SlimUser {
                user_id: Uuid::parse_str(UUID_PLACEHOLDER).expect("54g"),
                email: "".to_string(),
            },
            additional_users: vec![],
            skype: "".to_string(),
            phone_number: "".to_string(),
            two_factor_authentication: None,
            api_access_keys: vec![],
            billing_information: None,
            custom_conversions: vec![],
            referrer_handling_list: vec![],
            last_updated: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DefaultWayToOpenReport {
    GoToReport,
    OpenInBackground,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DefaultHomeScreen {
    Dashboard,
    CampaignList,
    LastOpenedGlobalReport,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConversionRegistrationTimeReporting {
    VisitTimestamp,
    PostbackTimestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TwoFactorAuthentication {
    data: String,
}

impl Account {
    pub fn update_change_date(&mut self) {
        self.last_updated = Utc::now()
    }
}
