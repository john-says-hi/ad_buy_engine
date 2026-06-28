use yew_router::switch::AllowMissing;
use yew_router::{prelude::*, Switch};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    // #[to = "/tertiary"]
    // Home,
    #[to = "/tertiary/#login"]
    Login,
    #[to = "/tertiary/#invitation"]
    Invitation,
    #[to = "/tertiary/#register"]
    Register,
    #[to = "/tertiary/#check_your_email/{email_service}"]
    CheckYourEmail(String),
    #[to = "/tertiary/#join_the_team"]
    RegisterAnotherUser,
}
