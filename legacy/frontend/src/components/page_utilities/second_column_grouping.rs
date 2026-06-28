use crate::appstate::app_state::AppState;
use crate::appstate::lists::PrimeElement;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use crate::utils::routes::AppRoute;
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Click(PrimeElement),
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
    pub onclick: Callback<()>,
}

pub struct SecondColumnGrouping {
    link: ComponentLink<Self>,
    props: Props,
    node_ref: NodeRef,
}

impl Component for SecondColumnGrouping {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let element = self.node_ref.cast::<Element>().expect("getr");
        toggle_uk_dropdown(element);

        match msg {
            Msg::Click(grouping_column) => {
                self.props
                    .state
                    .borrow_mut()
                    .set_second_prime_column(grouping_column.into());
                self.props.onclick.emit(())
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.link.send_message(Msg::Ignore);
        true
    }

    fn view(&self) -> Html {
        let mut column_groups = VList::new();

        for group in PrimeElement::iter() {
            let text = group.to_string();

            column_groups.push(VNode::from(html! {
                <li><a onclick=self.link.callback(move |_| Msg::Click(group.clone()))>{text}</a></li>
            }))
        }

        html! {
            <div>
                <ul class="uk-subnav uk-subnav-pill" uk-margin="">
                    <li>
                        <a href="#">{self.props.state.borrow().return_second_column_text_value()} <span uk-icon="icon:  triangle-down"></span></a>
                        <div ref=self.node_ref.clone() uk-dropdown="mode: click;">
                            <ul class="uk-nav uk-dropdown-nav">
                                {column_groups}
                            </ul>
                        </div>
                    </li>
                </ul>
            </div>
        }
    }
}
