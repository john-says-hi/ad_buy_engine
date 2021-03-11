use crate::data::account::Account;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "account_table",
    primary_key("account_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountModel {
    pub account_id: String,
    pub account_data: String,
    pub last_updated: i64,
}

impl From<Account> for AccountModel {
    fn from(account: Account) -> Self {
        Self {
            account_id: account.account_id.to_string(),
            account_data: serde_json::to_string(&account).expect("2g34S"),
            last_updated: account.last_updated.timestamp(),
        }
    }
}

impl From<AccountModel> for Account {
    fn from(account_model: AccountModel) -> Self {
        serde_json::from_str(&account_model.account_data).expect("$%TG@S")
    }
}
