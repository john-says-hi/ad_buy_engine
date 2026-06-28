use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug, EnumString, ToString, EnumIter)]
pub enum RedirectOption {
    Redirect,
    #[strum(serialize = "No Redirect")]
    NoRedirect,
}
