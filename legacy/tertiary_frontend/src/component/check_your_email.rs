use crate::alert;
use crate::routes::AppRoute;
use mailchecker::{blacklist, is_valid};
#[cfg(feasture = "backend_models")]
use yew::services::reader::ReaderTask;
use yew::{
    format::Json, html, prelude::*, services::fetch, virtual_dom::VNode, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*, Switch};

pub struct CheckYourEmail;

impl Component for CheckYourEmail {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CheckYourEmail
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <>
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
            </>
        }
    }
}
