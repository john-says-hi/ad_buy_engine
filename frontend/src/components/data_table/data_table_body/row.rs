use super::name_field::NameFieldBody;
use crate::agents::tick_tock::{TickTock, TickTockRequest};
use crate::appstate::app_state::{AppState, STATE};
use crate::appstate::lists::PrimeElement;
use crate::appstate::selected::SelectedElement;
use crate::components::data_table::data_table_body::select_field::SelectFieldBody;
use crate::notify_primary;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

pub enum Msg {
    Ignore,
    Select,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
    pub name: String,
    pub prime_element: PrimeElement,
    pub index: usize,
}

pub struct Row {
    link: ComponentLink<Self>,
    props: Props,
    selected: bool,
    tt: Box<dyn Bridge<TickTock>>,
}

impl Component for Row {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tt = TickTock::bridge(link.callback(|_| Msg::Ignore));

        Self {
            link,
            props,
            selected: false,
            tt,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select => {
                let state = self.props.state.borrow();
                let selected_elements = &mut *state.selected_elements.borrow_mut();

                if let Some(pos) = selected_elements.iter().position(|s| {
                    s.element_type == self.props.prime_element && s.index == self.props.index
                }) {
                    selected_elements.remove(pos);
                    self.selected = false;
                    self.tt.send(TickTockRequest::Tick);
                } else {
                    selected_elements.push(SelectedElement {
                        element_type: self.props.prime_element,
                        index: self.props.index,
                    });
                    self.selected = true;
                    self.tt.send(TickTockRequest::Tick);
                }
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(selected_element) = props
            .state
            .borrow()
            .selected_elements
            .borrow()
            .iter()
            .find(|s| s.index == props.index && s.element_type == props.prime_element)
        {
            self.selected = true;
        } else {
            self.selected = false;
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let selected_border = if self.selected {
            "border:2px solid blue;"
        } else {
            ""
        };

        html! {
           <tr onclick=self.link.callback(|_| Msg::Select) style=selected_border>

                <SelectFieldBody selected=self.selected />

                <NameFieldBody state=Rc::clone(&self.props.state) name=&self.props.name />

              <td>{"0"}</td>
              <td>{"0"}</td>
           </tr>
        }
    }
}
