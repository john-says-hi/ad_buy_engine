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
use super::backend_models::{
		account::AccountModel,
		user::UserModel,
		campaign::CampaignModel,
		funnel::FunnelModel,
	landing_page::LandingPageModel,
	traffic_source::TrafficSourceModel,
	offer_source::OfferSourceModel,
	offer::OfferModel,
	visit::VisitModel,
	invitation::Invitation
	};

#[cfg(feature = "backend")]
use crate::schema::emails;
#[cfg(feature = "backend")]
use diesel::{PgConnection, QueryResult, RunQueryDsl, prelude::*};
use uuid::Uuid;

#[cfg_attr(
feature = "backend",
derive(Queryable, Insertable, AsChangeset, Identifiable),
table_name = "emails",
primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailModel {
	pub id: String,
}

#[cfg(feature = "backend")]
impl EmailModel {
	pub fn all(conn:&PgConnection)->QueryResult<Vec<Self>> {
		emails::dsl::emails.load(conn)
	}
	
	pub fn delete_all(conn:&PgConnection)->QueryResult<usize> {
		diesel::delete(emails::dsl::emails).execute(conn)
	}
}

pub trait Accountable {}

#[cfg(feature = "backend")]
pub trait AccountableDBComm<T> {
	fn all_by_last_updated(acc_id: String, conn:&PgConnection)->QueryResult<Vec<T>>;
	fn all_for_account(acc_id: String, conn:&PgConnection)->QueryResult<Vec<T>>;
	fn delete_all_for_account(acc_id: String, conn:&PgConnection)->QueryResult<usize>;
}

#[cfg(feature = "backend")]
pub trait DatabaseCommunication<T> {
	fn new(new: T, conn:&PgConnection)->QueryResult<usize>;
	fn delete(model_id: String, conn:&PgConnection)->QueryResult<usize>;
	fn update(model_id: String, new:T, conn:&PgConnection)->QueryResult<usize>;
	fn get(model_id: String, conn:&PgConnection)->QueryResult<T>;
	fn update_and_get(model_id: String, new: T, conn:&PgConnection)->QueryResult<T>;
	fn delete_all(conn:&PgConnection)->QueryResult<usize>;
	fn all(conn:&PgConnection)->QueryResult<Vec<T>>;
}

#[cfg(feature = "backend")]
impl_database_communication!(
	AccountModel, accounts
	UserModel, users
	CampaignModel, campaigns
	FunnelModel, funnels
	TrafficSourceModel, traffic_sources
	LandingPageModel, landing_pages
	OfferModel, offers
	OfferSourceModel, offer_sources
);

#[cfg(feature = "backend")]
impl_accountable_database_communication!(
	UserModel, users
	CampaignModel, campaigns
	FunnelModel, funnels
	TrafficSourceModel, traffic_sources
	LandingPageModel, landing_pages
	OfferModel, offers
	OfferSourceModel, offer_sources
);

impl_accountable!(
	UserModel
	CampaignModel
	FunnelModel
	TrafficSourceModel
	LandingPageModel
	OfferModel
	OfferSourceModel
);