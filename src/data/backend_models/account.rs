use crate::data::account::{Account, ConversionRegistrationTimeReporting, DefaultHomeScreen, TwoFactorAuthentication};
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
        to_json_string!(
            id; account.account_id
            report_time_zone; account.report_time_zone
            billing_currency; account.billing_currency
            sys_language; account.sys_language
            // domains_configuration; account.domains_configuration
            work_spaces; account.work_spaces
            conversion_registration_time_reporting; account.conversion_registration_time_reporting
            default_home_screen; account.default_home_screen
            default_way_to_open_report; account.default_way_to_open_report
            default_reporting_currency; account.default_reporting_currency
            profile_first_name; account.profile_first_name
            profile_last_name; account.profile_last_name
            primary_user; account.primary_user
            additional_users; account.additional_users
            skype; account.skype
            phone_number; account.phone_number
            two_factor_authentication; account.two_factor_authentication
            api_access_keys; account.api_access_keys
            billing_information; account.billing_information
            custom_conversions; account.custom_conversions
            referrer_handling_list; account.referrer_handling_list
            fuel; account.fuel
        );
        
        Self {
            id,
            report_time_zone,
            billing_currency,
            sys_language,
            domains_configuration:serde_json::to_string(&account.domains_configuration).expect("V54sfg"),
            work_spaces,
            fuel,
            conversion_registration_time_reporting,
            default_home_screen,
            default_way_to_open_report,
            ip_anonymization: account.ip_anonymization,
            default_reporting_currency,
            profile_first_name,
            profile_last_name,
            primary_user,
            additional_users,
            skype,
            phone_number,
            two_factor_authentication,
            api_access_keys,
            billing_information,
            custom_conversions,
            referrer_handling_list,
            last_updated: account.last_updated.timestamp(),
        }
    }
}

impl From<AccountModel> for Account {
    fn from(account_model: AccountModel) -> Self {
        from_json_string!(
           account_id; account_model.id => Uuid
           report_time_zone; account_model.report_time_zone => TimeZone
           billing_currency; account_model.billing_currency => Currency
           sys_language; account_model.sys_language => Language
           // domains_configuration; account_model.domains_configuration => DomainsConfiguration
           work_spaces; account_model.domains_configuration => Vec<WorkSpace>
           conversion_registration_time_reporting; account_model.conversion_registration_time_reporting => ConversionRegistrationTimeReporting
           default_home_screen; account_model.default_home_screen => DefaultHomeScreen
           default_way_to_open_report; account_model.default_way_to_open_report => DefaultHomeScreen
           default_reporting_currency; account_model.default_reporting_currency => Currency
           default_home_screen; account_model.default_home_screen => DefaultHomeScreen
           primary_user; account_model.primary_user => SlimUser
           additional_users; account_model.additional_users => Vec<AdditionalUser>
           two_factor_authentication; account_model.two_factor_authentication => Option<TwoFactorAuthentication>
           api_access_keys; account_model.api_access_keys => Vec<String>
           billing_information; account_model.billing_information => Option<BillingInformation>
           custom_conversions; account_model.custom_conversions => Vec<CustomConversionEvent>
           referrer_handling_list; account_model.referrer_handling_list => Vec<ReplaceReferrerList>
           fuel; account_model.fuel => u64
        );
        
        Self {
            account_id,
            report_time_zone,
            billing_currency,
            sys_language,
            domains_configuration:serde_json::from_str(&account_model.domains_configuration).expect("G%^$xs"),
            work_spaces,
            fuel,
            conversion_registration_time_reporting,
            default_home_screen,
            default_way_to_open_report,
            ip_anonymization: account_model.ip_anonymization,
            default_reporting_currency,
            profile_first_name:account_model.profile_first_name,
            profile_last_name:account_model.profile_last_name,
            primary_user,
            additional_users,
            skype:account_model.skype,
            phone_number:account_model.phone_number,
            two_factor_authentication,
            api_access_keys,
            billing_information,
            custom_conversions,
            referrer_handling_list,
            last_updated: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(account_model.last_updated, 0),Utc),
        }
        
    }
}
