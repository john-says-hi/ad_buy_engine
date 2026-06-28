
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

pub struct Pagination {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Pagination {
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
<ul class="uk-pagination" uk-margin="">
    <li><a ><span uk-pagination-previous=""></span></a></li>
    <li><a >{"1"}</a></li>
    <li class="uk-disabled"><span>{"..."}</span></li>
    <li class="uk-active"><span>{"5"}</span></li>
    <li class="uk-disabled"><span>{"..."}</span></li>
    <li><a >{"10"}</a></li>
    <li><a ><span uk-pagination-next=""></span></a></li>
</ul>
        }
    }
}
