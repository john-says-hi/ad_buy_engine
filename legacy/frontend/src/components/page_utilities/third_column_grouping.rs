use crate::appstate::app_state::AppState;
use crate::appstate::lists::PrimeElement;
use crate::notify_primary;
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

pub struct ThirdColumnGrouping {
    link: ComponentLink<Self>,
    props: Props,
    node_ref: NodeRef,
    disabled: bool,
    text: String,
}

impl Component for ThirdColumnGrouping {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let disabled = props.state.borrow().should_render_third_column();
        let text = props.state.borrow().return_third_column_text_value();

        Self {
            link,
            props,
            node_ref: NodeRef::default(),
            disabled,
            text,
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
                    .set_third_prime_column(grouping_column.into());
                self.props.onclick.emit(());
            }
            Msg::Ignore => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.disabled = props.state.borrow().should_render_third_column();
        self.text = self.props.state.borrow().return_third_column_text_value();
        true
    }

    fn view(&self) -> Html {
        let mut column_groups = VList::new();

        if self.disabled {
            for group in PrimeElement::iter() {
                let text = group.to_string();

                column_groups.push(VNode::from(html! {
                <li><a onclick=self.link.callback(move |_| Msg::Click(group.clone()))>{text}</a></li>
            }))
            }
        };

        html! {
            <div>
                <ul class="uk-subnav uk-subnav-pill" uk-margin="">
                    <li>
                        <a href="#">{&self.text} <span uk-icon="icon:  triangle-down"></span></a>
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
