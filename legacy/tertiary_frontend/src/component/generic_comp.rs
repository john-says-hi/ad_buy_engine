use crate::{app::routes::AppRoute, prelude::*};
use ad_buy_engine::{
    prelude::*,
    protocol::{model, request, response},
};
use yew::{
    format::Json, html, prelude::*, services::fetch, virtual_dom::VNode, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*, Switch};

pub struct GenericComponent {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
}

pub enum Message {
    Example1,
    Example2,
    Ignore,
}

impl Component for GenericComponent {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Message::Ignore);
        let router = RouteAgent::bridge(callback);

        GenericComponent { link, router }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Example1 => {}
            Message::Example2 => {}
            Message::Ignore => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
                        <div class="  uk-preserve-color ">
                            <div class="uk-container">
                                <div class="uk-child-width-expand uk-text-center" uk-grid="">
                                    <div class=""></div>
                                        <div class="uk-width-1-2 uk-section">
                                            <div class=" uk-card uk-card-default uk-card-body   uk-border-rounded",>
                                            <h2>{"Check Your Email and Get Ready To Lock 'n Load, it's campaign time..."}</h2>
                                            </div>
                                        </div>
                                    <div class=""></div>
                                </div>
                            </div>
                        </div>
        }
    }
}
