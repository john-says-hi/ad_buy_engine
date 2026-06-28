use yew::prelude::*;

use crate::route::Route;
use crate::state::report::ReportState;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Properties)]
pub struct ReportToolbarProps {
    pub route: Route,
    pub report: ReportState,
}

#[function_component(ReportToolbar)]
pub fn report_toolbar(props: &ReportToolbarProps) -> Html {
    html! {
        <section class="abe-toolbar">
            <div class="abe-toolbar-row">
                <div class="abe-toolbar-left">
                    <ToolbarDropdown label={props.report.first_grouping} options={&GROUPING_OPTIONS} />
                    <ToolbarDropdown label={props.report.second_grouping} options={&GROUPING_OPTIONS_WITH_EMPTY} />
                    <ToolbarDropdown label={props.report.third_grouping} options={&GROUPING_OPTIONS_WITH_EMPTY} />
                </div>

                <div class="abe-toolbar-right">
                    <SearchControl />
                    <ToolbarDropdown label={props.report.date_range} options={&DATE_RANGE_OPTIONS} icon={Some("calendar")} />
                    <ToolbarButton label="Refresh" icon="refresh" />
                    <ToolbarButton label="Graph" icon="database" disabled={true} />
                </div>
            </div>

            <div class="abe-toolbar-row">
                <div class="abe-toolbar-left">
                    <Pagination />
                </div>

                <div class="abe-toolbar-right">
                    <CreateElementButton label={props.route.create_button_label()} />
                    <ToolbarButton label="Update Costs" icon="credit-card" disabled={true} />
                    <ToolbarButton label="Update" icon="file-edit" disabled={true} />
                    <ToolbarButton label="Clone" icon="copy" disabled={true} />
                    <ToolbarButton label="Law" icon="warning" disabled={true} />
                    <ToolbarButton label="Export" icon="download" disabled={true} />
                    <ToolbarDropdown label={props.report.row_limit} options={&ROW_LIMIT_OPTIONS} />
                    <ToolbarDropdown label={props.report.filter} options={&FILTER_OPTIONS} />
                </div>
            </div>
        </section>
    }
}

const GROUPING_OPTIONS: &[&str] = &[
    "Traffic Sources",
    "Offer Sources",
    "Brands",
    "Browser Versions",
    "Browsers",
    "Campaigns",
    "Connection Types",
    "Conversions",
    "Countries",
    "Day",
    "Day of Week",
    "Hour of Day",
    "Device Types",
    "Sequences",
    "Funnels",
    "ISP / Carriers",
    "Landers",
    "Mobile Carriers",
    "Models",
    "Month",
    "OS",
    "OS Versions",
    "Offers",
    "Proxies",
];

const GROUPING_OPTIONS_WITH_EMPTY: &[&str] = &[
    "Drill Down",
    "Traffic Sources",
    "Offer Sources",
    "Brands",
    "Browser Versions",
    "Browsers",
    "Campaigns",
    "Connection Types",
    "Conversions",
    "Countries",
    "Day",
    "Day of Week",
    "Hour of Day",
    "Device Types",
    "Sequences",
    "Funnels",
    "ISP / Carriers",
    "Landers",
    "Mobile Carriers",
    "Models",
    "Month",
    "OS",
    "OS Versions",
    "Offers",
    "Proxies",
];

const DATE_RANGE_OPTIONS: &[&str] = &[
    "Today",
    "Yesterday",
    "Last 3 Days",
    "Last 7 Days",
    "Last 14 Days",
    "Last 30 Days",
    "Last 6 Months",
    "Custom Range",
    "All of Time",
];

const ROW_LIMIT_OPTIONS: &[&str] = &["50", "100", "200", "500", "1000"];
const FILTER_OPTIONS: &[&str] = &["All", "Archived", "Has traffic", "Active"];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Properties)]
struct ToolbarDropdownProps {
    label: &'static str,
    options: &'static [&'static str],
    #[prop_or(None)]
    icon: Option<&'static str>,
}

#[function_component(ToolbarDropdown)]
fn toolbar_dropdown(props: &ToolbarDropdownProps) -> Html {
    html! {
        <div>
            <ul class="uk-subnav uk-subnav-pill" uk-margin="">
                <li>
                    <a class="abe-dropdown-label" href="#">
                        {
                            if let Some(icon) = props.icon {
                                render_icon(icon)
                            } else {
                                Html::default()
                            }
                        }
                        { props.label }
                        <span uk-icon="icon: triangle-down"></span>
                    </a>
                    <div uk-dropdown="mode: click;">
                        <ul class="uk-nav uk-dropdown-nav">
                            { for props.options.iter().map(|option| html! {
                                <li><a>{ option }</a></li>
                            }) }
                        </ul>
                    </div>
                </li>
            </ul>
        </div>
    }
}

#[function_component(SearchControl)]
fn search_control() -> Html {
    html! {
        <form class="uk-search uk-search-small abe-search">
            <span uk-search-icon=""></span>
            <input class="uk-search-input" type="search" placeholder="     Search..." disabled={true} />
        </form>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Properties)]
struct ToolbarButtonProps {
    label: &'static str,
    icon: &'static str,
    #[prop_or(false)]
    disabled: bool,
}

#[function_component(ToolbarButton)]
fn toolbar_button(props: &ToolbarButtonProps) -> Html {
    let disabled_class = props.disabled.then_some("uk-disabled");

    html! {
        <button
            class={classes!("uk-button", "uk-button-default", "uk-button-small", "uk-background-primary", "uk-light", "abe-button", disabled_class)}
            disabled={props.disabled}
            uk-tooltip={props.disabled.then_some("title: Not built yet")}
        >
            { render_icon(props.icon) }
            { props.label }
        </button>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Properties)]
struct CreateElementButtonProps {
    label: Option<&'static str>,
}

#[function_component(CreateElementButton)]
fn create_element_button(props: &CreateElementButtonProps) -> Html {
    match props.label {
        Some(label) => html! { <ToolbarButton label={label} icon="plus" /> },
        None => Html::default(),
    }
}

#[function_component(Pagination)]
fn pagination() -> Html {
    html! {
        <ul class="uk-pagination abe-pagination" uk-margin="">
            <li><a><span uk-pagination-previous=""></span></a></li>
            <li><a>{ "1" }</a></li>
            <li class="uk-disabled"><span>{ "..." }</span></li>
            <li class="uk-active"><span>{ "5" }</span></li>
            <li class="uk-disabled"><span>{ "..." }</span></li>
            <li><a>{ "10" }</a></li>
            <li><a><span uk-pagination-next=""></span></a></li>
        </ul>
    }
}

fn render_icon(icon: &'static str) -> Html {
    html! {
        <span class="uk-margin-small-right" uk-icon={format!("icon: {icon}")}></span>
    }
}
