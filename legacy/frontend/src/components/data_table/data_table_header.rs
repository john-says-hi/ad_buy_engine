pub mod name_field;

use crate::appstate::app_state::AppState;
use name_field::NameFieldHead;
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

pub struct DataTableHead {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for DataTableHead {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
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
        <thead>
            <tr>
                <th class="uk-table-shrink" uk-tooltip="title: Select All"><input class="uk-checkbox uk-disabled uk-margin-left" type="checkbox" /></th>
                <NameFieldHead state=Rc::clone(&self.props.state) />
                <th class="uk-table-shrink">{"Visits"}</th>
                <th class="uk-table-shrink">{"Unique"}</th>
            </tr>
        </thead>
            }
    }
}
