pub mod account_depricating;
pub mod crud;
pub mod invitation_depricating;
pub mod user_depricating;


use crate::utils::database::{get_conn,PgPool};

database_functions!(
	account, AccountModel
	user, UserModel
	offer_source, OfferSourceModel
	offer, OfferModel
	landing_page, LandingPageModel
	funnel, FunnelModel
	campaign, CampaignModel
	traffic_source, TrafficSourceModel
);

pub mod visits {
	use crate::utils::errors::ApiError;
	use crate::utils::database::{get_conn, PgPool};
	use std::ops::Deref;
	use ad_buy_engine::data::backend_models::visit::VisitModel;
	
	pub fn all(pool: &PgPool) -> Result<Vec<VisitModel>, ApiError> {
Ok(<VisitModel>::all(get_conn(pool)?.deref())?)
}
	
	pub fn new(new: VisitModel, pool: &PgPool) -> Result<usize, ApiError> {
		Ok(<VisitModel>::new(new, get_conn(pool)?.deref())?)
	}
	
	pub fn delete(model_id: i64, pool: &PgPool) -> Result<usize, ApiError> {
		Ok(<VisitModel>::delete(model_id, get_conn(pool)?.deref())?)
	}
	
	pub fn update(model_id: i64, new: VisitModel, pool: &PgPool) -> Result<usize, ApiError> {
		Ok(<VisitModel>::update(model_id, new, get_conn(pool)?.deref())?)
	}
	
	pub fn get(model_id: i64, pool: &PgPool) -> Result<VisitModel, ApiError> {
Ok(<VisitModel>::get(model_id, get_conn(pool)?.deref())?)
}
	
	pub fn update_and_get(model_id: i64, new: VisitModel, pool: &PgPool) -> Result<VisitModel, ApiError> {
Ok(<VisitModel>::update_and_get(model_id, new, get_conn(pool)?.deref())?)
}
	
	pub fn delete_all(pool: &PgPool) -> Result<usize, ApiError> {
		Ok(<VisitModel>::delete_all(get_conn(pool)?.deref())?)
	}
	
	// update postback url conversion
	// update link click, offer click, lp click
	// get latest 1000
}