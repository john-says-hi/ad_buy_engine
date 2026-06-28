pub mod name_field;

use crate::appstate::app_state::AppState;
use name_field::NameFieldFoot;
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

pub struct DataTableFoot {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for DataTableFoot {
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
        <tfoot class="uk-margin-top-large">
            <tr>
                <td>{"Totals:"}</td>
                <NameFieldFoot state=Rc::clone(&self.props.state) />
                <td>{"0"}</td>
                <td>{"0"}</td>
            </tr>
        </tfoot>
            }
    }
}
