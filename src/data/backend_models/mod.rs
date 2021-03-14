pub mod account;
pub mod campaign;
pub mod funnel;
pub mod invitation;
pub mod landing_page;
pub mod live_campaign_table;
pub mod offer;
pub mod offer_source;
pub mod traffic_source;
pub mod user;
pub mod visit;
pub mod visit_identity;
pub mod visit_ledger;

#[cfg(feature = "backend")]
use crate::schema::*;
use diesel::{PgConnection, QueryResult, RunQueryDsl};

#[cfg_attr(
feature = "backend",
derive(Queryable, Insertable, AsChangeset, Identifiable),
table_name = "email_list_table",
primary_key("email")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailModel {
	pub email: String,
}

impl EmailModel {
	pub fn all(conn:&PgConnection)->QueryResult<Vec<Self>> {
		email_list_table::dsl::email_list_table.load(conn)
	}
}