use yew::prelude::*;
use yew_router::prelude::*;

use crate::ui::shell::Shell;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Routable)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Dashboard,
    #[at("/campaigns")]
    Campaigns,
    #[at("/offers")]
    Offers,
    #[at("/landers")]
    Landers,
    #[at("/conversions")]
    Conversions,
    #[at("/funnels")]
    Funnels,
    #[at("/traffic-sources")]
    TrafficSources,
    #[at("/offer-sources")]
    OfferSources,
    #[at("/connection")]
    Connection,
    #[at("/browsers")]
    Browsers,
    #[at("/device")]
    Device,
    #[at("/os")]
    Os,
    #[at("/date")]
    Date,
    #[at("/day-parting")]
    DayParting,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationItem {
    pub route: Route,
    pub label: &'static str,
    pub icon: &'static str,
}

pub const NAVIGATION_ITEMS: [NavigationItem; 14] = [
    NavigationItem::new(Route::Dashboard, "Dashboard", "home"),
    NavigationItem::new(Route::Campaigns, "Campaigns", "world"),
    NavigationItem::new(Route::Offers, "Offers", "tag"),
    NavigationItem::new(Route::Landers, "Landers", "file-text"),
    NavigationItem::new(Route::Conversions, "Conversions", "check"),
    NavigationItem::new(Route::Funnels, "Funnels", "database"),
    NavigationItem::new(Route::TrafficSources, "Traffic Sources", "users"),
    NavigationItem::new(Route::OfferSources, "Offer Sources", "cart"),
    NavigationItem::new(Route::Connection, "Connection", "link"),
    NavigationItem::new(Route::Browsers, "Browsers", "desktop"),
    NavigationItem::new(Route::Device, "Device", "tablet"),
    NavigationItem::new(Route::Os, "OS", "cog"),
    NavigationItem::new(Route::Date, "Date", "calendar"),
    NavigationItem::new(Route::DayParting, "Day Parting", "clock"),
];

impl NavigationItem {
    pub const fn new(route: Route, label: &'static str, icon: &'static str) -> Self {
        Self { route, label, icon }
    }
}

impl Route {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Campaigns => "Campaigns",
            Self::Offers => "Offers",
            Self::Landers => "Landers",
            Self::Conversions => "Conversions",
            Self::Funnels => "Funnels",
            Self::TrafficSources => "Traffic Sources",
            Self::OfferSources => "Offer Sources",
            Self::Connection => "Connection",
            Self::Browsers => "Browsers",
            Self::Device => "Device",
            Self::Os => "OS",
            Self::Date => "Date",
            Self::DayParting => "Day Parting",
        }
    }

    pub const fn path(self) -> &'static str {
        match self {
            Self::Dashboard => "/",
            Self::Campaigns => "/campaigns",
            Self::Offers => "/offers",
            Self::Landers => "/landers",
            Self::Conversions => "/conversions",
            Self::Funnels => "/funnels",
            Self::TrafficSources => "/traffic-sources",
            Self::OfferSources => "/offer-sources",
            Self::Connection => "/connection",
            Self::Browsers => "/browsers",
            Self::Device => "/device",
            Self::Os => "/os",
            Self::Date => "/date",
            Self::DayParting => "/day-parting",
        }
    }

    pub const fn render_route(self) -> Self {
        self
    }

    pub const fn is_dashboard(self) -> bool {
        matches!(self.render_route(), Self::Dashboard)
    }

    pub const fn is_report(self) -> bool {
        !self.is_dashboard()
    }

    pub const fn create_button_label(self) -> Option<&'static str> {
        match self.render_route() {
            Self::Campaigns => Some("New Campaign"),
            Self::Offers => Some("New Offer"),
            Self::Landers => Some("New Lander"),
            Self::Funnels => Some("New Funnel"),
            Self::TrafficSources => Some("New Traffic Source"),
            Self::OfferSources => Some("New Offer Source"),
            _ => None,
        }
    }
}

pub fn switch(route: Route) -> Html {
    html! { <Shell route={route.render_route()} on_logout={Callback::from(|_| ())} /> }
}
