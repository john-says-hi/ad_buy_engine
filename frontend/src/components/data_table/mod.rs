pub mod data_state_logic_models;
pub mod data_table_body;
pub mod data_table_footer;
pub mod data_table_header;

use crate::appstate::app_state::AppState;
use data_table_body::DataTableBody;
use data_table_footer::DataTableFoot;
use data_table_header::DataTableHead;
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

pub struct DataTable {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for DataTable {
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
        true
    }

    fn view(&self) -> Html {
        html! {
        <table class="uk-table uk-table-responsive uk-table-divider uk-table-striped uk-table-hover uk-table-small">

        <DataTableHead state=Rc::clone(&self.props.state) />

        <DataTableFoot state=Rc::clone(&self.props.state) />

        <DataTableBody state=Rc::clone(&self.props.state) />

        </table>
                }
    }
}
