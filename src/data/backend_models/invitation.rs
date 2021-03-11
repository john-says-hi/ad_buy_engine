#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "invitation_table",
    primary_key("invitation_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Invitation {
    pub invitation_id: String,
    pub email: String,
    pub email_confirmed: bool,
    pub expires_at: chrono::NaiveDateTime,
}
