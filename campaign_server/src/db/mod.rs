pub mod account_depricated;
pub mod crud;
pub mod invitation_depricated;
pub mod user_depricated;


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
