use crate::agents::tick_tock::{TickTock, TickTockRequest};
use crate::appstate::app_state::{AppState, STATE};
use crate::appstate::lists::PrimeElement;
use crate::notify_primary;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_material::MatCheckbox;

pub enum Msg {
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub selected: bool,
}

pub struct SelectFieldBody {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for SelectFieldBody {
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
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
              <td>
                 <div class="uk-grid-small uk-grid" uk-grid="">

                    <div class="uk-width-auto">
                       <p class="uk-text-center" style="background-color: green;color: white;">{"+"}</p>
                    </div>

                    <div class="uk-width-auto"><input class="uk-checkbox" type="checkbox" checked=self.props.selected /></div>
                 </div>

              </td>
        }
    }
}
