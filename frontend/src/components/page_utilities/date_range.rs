use crate::appstate::app_state::AppState;
use crate::appstate::lists::ReportDateRange;
use crate::utils::javascript::js_bindings::toggle_uk_dropdown;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum Msg {
    Click(ReportDateRange),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct DateRange {
    link: ComponentLink<Self>,
    text: String,
    props: Props,
    node_ref: NodeRef,
}

impl Component for DateRange {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let text = props.state.borrow().return_date_range_text();

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
            Msg::Click(report) => {
                self.text = report.to_string();
                self.props.state.borrow_mut().set_date_range(report)
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
                    <a class="uk-margin-small-right"><span class="fas fa-calendar-alt uk-margin-small-right"></span>{&self.text} <span uk-icon="icon:  triangle-down"></span></a>
                    <div ref=self.node_ref.clone() uk-dropdown="mode: click;">
                        <ul class="uk-nav uk-dropdown-nav">
                            <li><a onclick=self.link.callback(|_| Msg::Click(ReportDateRange::Today))>{"Today"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(ReportDateRange::Yesterday))>{"Yesterday"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(ReportDateRange::ThreeDays))>{"Last 3 Days"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(ReportDateRange::SevenDays))>{"Last 7 Days"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(ReportDateRange::FourteenDays))>{"Last 14 Days"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(ReportDateRange::ThirtyDays))>{"Last 30 Days"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(ReportDateRange::SixMonths))>{"Last 6 Months"}</a></li>
                            <li uk-tooltip="title:Not built yet "><a>{"Custom Range"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::Click(ReportDateRange::All))>{"All of Time"}</a></li>
                        </ul>
                    </div>
                </li>
            </ul>
        </div>
                }
    }
}
