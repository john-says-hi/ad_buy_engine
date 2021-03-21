
use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use crate::appstate::app_state::STATE;

pub enum Msg {
    Click,
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
}

pub struct Search {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Search {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
    
    fn view(&self) -> Html {
        html! {
<div>
                     <form class="uk-search uk-search-small">
                        <span uk-search-icon=""></span>
                        <input class="uk-search-input" type="search" placeholder="     Search..." />
                     </form>
</div>
        }
    }
}
