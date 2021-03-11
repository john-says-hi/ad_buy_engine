use crate::appstate::app_state::{AppState, STATE};
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

pub enum Msg {
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub name: String,
}

pub struct NameFieldBody {
    link: ComponentLink<Self>,
    props: Props,
    name: String,
}

impl Component for NameFieldBody {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let name = props.name.clone();
        Self { link, props, name }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.name = props.name.clone();
        true
    }

    fn view(&self) -> Html {
        html! {
        <td>{&self.name}</td>
        }
    }
}
