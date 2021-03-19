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
