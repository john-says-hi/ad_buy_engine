use yew::{
    html, prelude::*, virtual_dom::VNode, Component, ComponentLink, Html, Properties, ShouldRender,
};

pub struct PublicFooter {
    link: ComponentLink<Self>,
    counter: usize,
}

pub enum Msg {
    More,
    Less,
}

impl Component for PublicFooter {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        PublicFooter { link, counter: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::More => {
                self.counter = self.counter + 1;
            }
            Msg::Less => {
                if self.counter > 0 {
                    self.counter = self.counter - 1;
                }
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
                            <div class=" ">
                              <div class="uk-container uk-preserve-color">
                                  <div class="uk-border-rounded uk-card uk-card-body uk-card-default uk-margin-left uk-margin-bottom" uk-grid="">
                                    <div class="uk-width-1-1 uk-margin-top"><h5 class="">{ "This Project Needs You" }</h5></div>

                                    <div class=" uk-width-1-2"><p>{ "Imagine a world where your ad campaigns can run themself. They can use your ad supply material to create new campaigns and even auto scale according to the laws you set." }</p><div><h5 class="">{ "Build The Engine" }</h5></div><p style="color:#15E900">{ "v.01" }</p><p>{ "...as fast as it gets. I'll make sure of it." }</p></div>

                                    <div class="uk-width-1-2"><ul class="uk-list">
                                      <li><a class="grey-text text-lighten-3" href="#!">{ "Contact" }</a></li>
                                      <li><a class="grey-text text-lighten-3" href="#!">{ "Project's Vision Forward" }</a></li>
                                      <li><a class="grey-text text-lighten-3" href="#!">{ "Historical Conception" }</a></li>
                                      <li><a class="grey-text text-lighten-3" href="#!">{ "Abo ut the Engine" }</a></li>
                                      <li><a class="grey-text text-lighten-3" href="#!">{ "Join Now" }</a></li>
                                    </ul><h6 style="color: #15E900;">{ "Powered By Web Assembly & Rust" }</h6></div>

                                    <p class="uk-width-1-1 uk-text-center">{ "© 2020 ValknutEngine.com JPClickMedia LLC | 30 N 2nd St, Ashland, OR 97520 Apt 3" }</p>

                                    </div>
                                </div>
                            </div>
            </div>
        }
    }
}
