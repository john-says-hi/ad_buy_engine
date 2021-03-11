use crate::appstate::app_state::AppState;
use crate::appstate::lists::RowLimitOptions;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Click(RowLimitOptions),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct RowLimit {
    link: ComponentLink<Self>,
    text: String,
    props: Props,
    node_ref: NodeRef,
}

impl Component for RowLimit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let text = props.state.borrow().return_row_limit_text();

        Self {
            link,
            text,
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let element = self.node_ref.cast::<Element>().expect("getr");
        toggle_uk_dropdown(element);

        match msg {
            Msg::Click(option) => {
                self.text = option.to_string();

                self.props.state.borrow_mut().set_row_limit(option)
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <div>
            <ul class="uk-subnav uk-subnav-pill" uk-margin="">
                <li>
                    <a class="uk-margin-small-right">{&self.text} <span uk-icon="icon:  triangle-down"></span></a>
                    <div ref=self.node_ref.clone() uk-dropdown="mode: click;">
                        <ul class="uk-nav uk-dropdown-nav">
                            <li><a onclick=self.link.callback(|_| Msg::Click(RowLimitOptions::Fifty))>{"50"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(RowLimitOptions::Hundred))>{"100"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(RowLimitOptions::TwoHundred))>{"200"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(RowLimitOptions::FiveHundred))>{"500"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(RowLimitOptions::OneThousand))>{"1000"}</a></li>
                        </ul>
                    </div>
                </li>
            </ul>
        </div>
                }
    }
}
