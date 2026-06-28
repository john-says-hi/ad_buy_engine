use crate::javascript::js_bindings::send_to_secure;
use yew::{
    html, prelude::*, virtual_dom::VNode, Component, ComponentLink, Html, Properties, ShouldRender,
};

pub struct PublicHomePage {
    link: ComponentLink<Self>,
    counter: usize,
}

pub enum Msg {
    More,
    Less,
    SendToSecure,
}

impl Component for PublicHomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::SendToSecure);

        PublicHomePage { link, counter: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => {
                // send_to_secure();
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                        <div class="  uk-preserve-color ">
                            <div class="uk-container">
                                <div class="uk-child-width-expand uk-text-center" uk-grid="">
                                    <div class=""></div>
                                        <div class="uk-width-1-2 uk-section">
                                            <div class=" uk-card uk-card-default uk-card-body   uk-border-rounded",>
                                                <p>{"PUBLIC HOMEPAGE WECOME!!"}</p>
                                            </div>
                                        </div>
                                    <div class=""></div>
                                </div>
                            </div>
                        </div>
            </div>
        }
    }
}
