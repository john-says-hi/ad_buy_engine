use crate::appstate::app_state::AppState;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

pub enum Msg {
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct NameFieldHead {
    link: ComponentLink<Self>,
}

impl Component for NameFieldHead {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
                <th class="uk-table-shrink">{"Name"}</th>
        }
    }
}
