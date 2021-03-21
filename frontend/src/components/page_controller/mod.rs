mod browser;
mod campaigns;
mod connection;
mod conversions;
mod dashboard;
mod date;
mod day_parting;
mod device;
mod funnels;
mod landers;
mod offer_sources;
mod offers;
mod os;
mod traffic_sources;

use crate::appstate::app_state::{AppState, STATE};
use crate::utils::routes::AppRoute;
use browser::BrowserDrop;
use campaigns::CampaignBtn;
use connection::ConnectionDrop;
use conversions::ConversionsBtn;
use dashboard::DashboardBtn;
use date::DateDrop;
use day_parting::DayPartingDrop;
use device::DevicesDrop;
use funnels::FunnelBtn;
use landers::LanderBtn;
use offer_sources::OfferSourceBtn;
use offers::OfferBtn;
use os::OSDrop;
use traffic_sources::TrafficBtn;

use crate::{notify_primary, RootComponent};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Element;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_material::list::GraphicType;
use yew_material::{MatListItem, MatMenu, MatSelect, MatTab, MatTabBar};
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;

pub enum Msg {
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct PageController {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    props: Props,
}

impl Component for PageController {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));

        Self {
            link,
            router,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props=props;
        true
    }

    fn view(&self) -> Html {
        html! {
<div class="uk-margin uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1  uk-grid-row-collapse" uk-grid="">
        <nav class="uk-margin-top" uk-navbar="">
            <div class="uk-navbar-left">
                <ul class="uk-navbar-nav uk-flex-wrap uk-flex-center">
                    
                    <DashboardBtn state=Rc::clone(&self.props.state) />
                    <CampaignBtn state=Rc::clone(&self.props.state) />
                    <OfferBtn state=Rc::clone(&self.props.state) />
                    <LanderBtn state=Rc::clone(&self.props.state) />
                    <ConversionsBtn state=Rc::clone(&self.props.state) />
                    <FunnelBtn state=Rc::clone(&self.props.state) />
                    <TrafficBtn state=Rc::clone(&self.props.state) />
                    <OfferSourceBtn state=Rc::clone(&self.props.state) />
                    <ConnectionDrop state=Rc::clone(&self.props.state) />
                    <BrowserDrop state=Rc::clone(&self.props.state) />
                    <DevicesDrop state=Rc::clone(&self.props.state) />
                    <OSDrop state=Rc::clone(&self.props.state) />
                    <DateDrop state=Rc::clone(&self.props.state) />
                    <DayPartingDrop state=Rc::clone(&self.props.state) />
                  
                </ul>
            </div>
        </nav>
</div>

                                        }
    }
}