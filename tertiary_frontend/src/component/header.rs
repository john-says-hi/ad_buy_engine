use crate::routes::AppRoute;
use yew::{
    html, prelude::*, virtual_dom::VNode, Component, ComponentLink, Html, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*, Switch};

pub struct PublicHeader {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
}

pub enum Message {
    Ignore,
    Invitation,
    Login,
}

impl Component for PublicHeader {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Message::Ignore);
        let router = RouteAgent::bridge(callback);

        PublicHeader { link, router }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Ignore => {}
            Message::Invitation => self.router.send(ChangeRoute(AppRoute::Invitation.into())),
            Message::Login => self.router.send(ChangeRoute(AppRoute::Login.into())),
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
                    <div>
                        <div uk-sticky="sel-target: .uk-navbar-container; cls-active: uk-navbar-sticky uk-position-relative">
                            <div class="uk-position-relative">
                                <nav class="uk-navbar-container uk-margin" uk-navbar="dropbar: false">
                                    <div class="uk-navbar-left">
                                        // LOGO
        //                                <DebugComponent :/>
                                        <a class="uk-navbar-item uk-logo uk-margin-left" href="#">{"Ad Buy Engine"}</a>
                                        <img class="logo uk-navbar-item uk-logo" src="/assets/logo.svg"/>

                                        // THE SERVICES DROPDOWN
                                        <ul class="uk-navbar-nav">
                                            <li>
                                                <a class="uk-text-bold" href="#">{"TraHIHIcker"}</a>
                                                <div class="uk-navbar-dropdown">
                                                    <ul class="uk-nav uk-navbar-dropdown-nav">
                                                        <li class="uk-active"><a href="#">{"HOLDER"}</a></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li class="uk-nav-header">{"HOLDER"}</li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li class="uk-nav-divider"></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                    </ul>
                                                </div>
                                            </li>
                                        </ul>

                                        // PERKS DROPDOWN
                                        <ul class="uk-navbar-nav">
                                            <li>
                                                <a class="uk-text-bold" href="#">{"Campaign Tools"}</a>
                                                <div class="uk-navbar-dropdown">
                                                    <ul class="uk-nav uk-navbar-dropdown-nav">
                                                        <li class="uk-active"><a href="#">{"HOLDER"}</a></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li class="uk-nav-header">{"HOLDER"}</li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li class="uk-nav-divider"></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                    </ul>
                                                </div>
                                            </li>
                                        </ul>

                                        // Pricing
                                        <ul class="uk-navbar-nav">
                                            <li><a class="uk-text-bold" href="#">{"Pricing"}</a></li>
                                            <li><a class="uk-text-bold" href="#">{"System Operations"}</a></li>
                                        </ul>

                                        // TRAINING DROPDOWN
                                        <ul class="uk-navbar-nav">
                                            <li>
                                                <a class="uk-text-bold" href="#">{"Training"}</a>
                                                <div class="uk-navbar-dropdown">
                                                    <ul class="uk-nav uk-navbar-dropdown-nav">
                                                        <li class="uk-active"><a href="#">{"HOLDER"}</a></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li class="uk-nav-header">{"HOLDER"}</li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li class="uk-nav-divider"></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                    </ul>
                                                </div>
                                            </li>
                                        </ul>

                                        // PERKS ABOUT
                                        <ul class="uk-navbar-nav">
                                            <li>
                                                <a class="uk-text-bold" href="#">{"About"}</a>
                                                <div class="uk-navbar-dropdown">
                                                    <ul class="uk-nav uk-navbar-dropdown-nav">
                                                        <li class="uk-active"><a href="#">{"HOLDER"}</a></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li class="uk-nav-header">{"HOLDER"}</li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                        <li class="uk-nav-divider"></li>
                                                        <li><a href="#">{"HOLDER"}</a></li>
                                                    </ul>
                                                </div>
                                            </li>
                                        </ul>

                                    </div>

                                    // FAR RIGHT
                                    <div class="uk-navbar-right">

                                        // LOGIN
                                        <div class="uk-navbar-item">
                                            <div>{""}
                                            <a class="uk-button uk-button-primary uk-button-small" onclick= self.link.callback(|_| Message::Login)
                                            >{"Login"}</a>
                                            </div>
                                        </div>
                                        // SIGN UP
                                        <div class="uk-navbar-item">
                                            <button class="uk-button uk-button-default uk-text-bold button-light"
                                                                onclick=self.link.callback(|_| Message::Invitation)
                                            >{"GAIN ACCESS"}</button>
                                        </div>

                                        // END FAR RIGHT
                                    </div>

                                </nav>
                                <div class="uk-navbar-dropbar"></div>
                            </div>
                        </div>
                    </div>
                }
    }
}
