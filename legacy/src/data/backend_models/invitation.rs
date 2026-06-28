#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "invitation",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Invitation {
    pub id: String,
    pub email: String,
    pub email_confirmed: bool,
    pub expires_at: chrono::NaiveDateTime,
}
