use crate::data::account::{Account, ConversionRegistrationTimeReporting, DefaultHomeScreen, TwoFactorAuthentication, DefaultWayToOpenReport};
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::{NaiveDateTime, DateTime, Utc};
#[cfg(feature = "backend")]
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use uuid::Uuid;
use crate::data::lists::time_zone::TimeZone;
use crate::data::lists::{Currency, Language};
use crate::data::account::domains_configuration::DomainsConfiguration;
use crate::data::work_space::{WorkSpace, AdditionalUser};
use crate::data::user::SlimUser;
use crate::data::account::billing::BillingInformation;
use crate::data::custom_events::CustomConversionEvent;
use crate::data::lists::referrer_handling::ReplaceReferrerList;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "accounts",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountModel {
    pub id: String,
    pub report_time_zone: String,
    pub billing_currency: String,
    pub sys_language: String,
    pub domains_configuration: String,
    pub work_spaces: String,
    pub fuel: String,
    pub conversion_registration_time_reporting: String,
    pub default_home_screen: String,
    pub default_way_to_open_report: String,
    pub ip_anonymization: bool,
    pub default_reporting_currency: String,
    pub profile_first_name: String,
    pub profile_last_name: String,
    pub primary_user: String,
    pub additional_users: String,
    pub skype: String,
    pub phone_number: String,
    pub two_factor_authentication: String,
    pub api_access_keys: String,
    pub billing_information: String,
    pub custom_conversions: String,
    pub referrer_handling_list: String,
    pub last_updated: i64,
}

#[cfg(feature = "backend")]
impl AccountModel {
    pub fn delete_all(conn:&PgConnection)->QueryResult<usize> {
        diesel::delete(accounts::dsl::accounts).execute(conn)
    }
}

impl From<Account> for AccountModel {
    fn from(account: Account) -> Self {
    
        Self {
            id : account.account_id.to_string(),
            report_time_zone:serde_json::to_string(&account.report_time_zone).expect("rerRg"),
            billing_currency:serde_json::to_string(&account.default_reporting_currency).expect("rerRg"),
            sys_language:serde_json::to_string(&account.sys_language).expect("rerRg"),
            domains_configuration:serde_json::to_string(&account.domains_configuration).expect("rerRg"),
            work_spaces:serde_json::to_string(&account.work_spaces).expect("rerRg"),
            fuel:serde_json::to_string(&account.fuel).expect("rerRg"),
            conversion_registration_time_reporting:serde_json::to_string(&account.conversion_registration_time_reporting).expect("rerRg"),
            default_home_screen:serde_json::to_string(&account.default_home_screen).expect("rerRg"),
            default_way_to_open_report:serde_json::to_string(&account.default_way_to_open_report).expect("rerRg"),
            ip_anonymization: account.ip_anonymization,
            default_reporting_currency:serde_json::to_string(&account.default_reporting_currency).expect("rerRg"),
            profile_first_name:account.profile_first_name,
            profile_last_name:account.profile_last_name,
            primary_user:serde_json::to_string(&account.primary_user).expect("rerRg"),
            additional_users:serde_json::to_string(&account.additional_users).expect("rerRg"),
            skype:account.skype,
            phone_number:account.phone_number,
            two_factor_authentication:serde_json::to_string(&account.two_factor_authentication).expect("rerRg"),
            api_access_keys:serde_json::to_string(&account.api_access_keys).expect("rerRg"),
            billing_information:serde_json::to_string(&account.billing_information).expect("rerRg"),
            custom_conversions:serde_json::to_string(&account.custom_conversions).expect("rerRg"),
            referrer_handling_list:serde_json::to_string(&account.referrer_handling_list).expect("rerRg"),
            last_updated: account.last_updated.timestamp(),
        }

    }
}

impl From<AccountModel> for Account {
    fn from(account_model: AccountModel) -> Self {
    
        Self {
            account_id: Uuid::parse_str(&account_model.id).expect("g534rsd"),
            report_time_zone:serde_json::from_str(&account_model.report_time_zone).expect("GV545r3"),
            billing_currency:serde_json::from_str(&account_model.billing_currency).expect("GV54fg3"),
            sys_language:serde_json::from_str(&account_model.sys_language).expect("G4V54fg3"),
            domains_configuration:serde_json::from_str(&account_model.domains_configuration).expect("GV54fg4re3"),
            work_spaces:serde_json::from_str(&account_model.work_spaces).expect("GV54fg344"),
            fuel:serde_json::from_str(&account_model.fuel).expect("G545V54fg3"),
            conversion_registration_time_reporting:serde_json::from_str(&account_model.conversion_registration_time_reporting).expect("G4V54fg35"),
            default_home_screen:serde_json::from_str(&account_model.default_home_screen).expect("GV54fgfd3"),
            default_way_to_open_report:serde_json::from_str(&account_model.default_way_to_open_report).expect("GV5455fg3"),
            ip_anonymization: account_model.ip_anonymization,
            default_reporting_currency:serde_json::from_str(&account_model.default_reporting_currency).expect("GV54fggfg3"),
            profile_first_name:account_model.profile_first_name,
            profile_last_name:account_model.profile_last_name,
            primary_user:serde_json::from_str(&account_model.primary_user).expect("GV54ff4fg3"),
            additional_users:serde_json::from_str(&account_model.additional_users).expect("GV54f4efg3"),
            skype:account_model.skype,
            phone_number:account_model.phone_number,
            two_factor_authentication:serde_json::from_str(&account_model.two_factor_authentication).expect("GV54fdfdfg3"),
            api_access_keys:serde_json::from_str(&account_model.api_access_keys).expect("df4GV54fg3"),
            billing_information:serde_json::from_str(&account_model.billing_information).expect("GV54fggf45g53"),
            custom_conversions:serde_json::from_str(&account_model.custom_conversions).expect("GV54fgvf3"),
            referrer_handling_list:serde_json::from_str(&account_model.referrer_handling_list).expect("GV54fgfgfg3"),
            last_updated: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(account_model.last_updated, 0),Utc),
        }
    }
}
