use yew::prelude::*;

use crate::route::Route;
use crate::state::report::{DATE_RANGE_OPTIONS, ReportDateRange, ReportState};
use web_sys::HtmlInputElement;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ReportToolbarProps {
    pub route: Route,
    pub report: ReportState,
    pub search_query: String,
    pub loading: bool,
    pub on_create: Callback<Route>,
    pub on_search: Callback<String>,
    pub on_date_range_change: Callback<ReportDateRange>,
    pub on_refresh: Callback<()>,
}

#[function_component(ReportToolbar)]
pub fn report_toolbar(props: &ReportToolbarProps) -> Html {
    let on_refresh = {
        let on_refresh = props.on_refresh.clone();
        Callback::from(move |_| on_refresh.emit(()))
    };

    html! {
        <section class="abe-toolbar">
            <div class="abe-toolbar-row">
                <div class="abe-toolbar-left">
                    <ToolbarDropdown label={props.report.first_grouping} options={&GROUPING_OPTIONS} />
                    <ToolbarDropdown label={props.report.second_grouping} options={&GROUPING_OPTIONS_WITH_EMPTY} />
                    <ToolbarDropdown label={props.report.third_grouping} options={&GROUPING_OPTIONS_WITH_EMPTY} />
                </div>

                <div class="abe-toolbar-right">
                    <SearchControl value={props.search_query.clone()} on_search={props.on_search.clone()} />
                    <DateRangeDropdown
                        selected={props.report.date_range}
                        on_change={props.on_date_range_change.clone()}
                    />
                    <ToolbarButton
                        label="Refresh"
                        icon="refresh"
                        disabled={props.loading}
                        onclick={on_refresh}
                    />
                    <ToolbarButton label="Graph" icon="database" disabled={true} />
                </div>
            </div>

            <div class="abe-toolbar-row">
                <div class="abe-toolbar-left">
                    <Pagination />
                </div>

                <div class="abe-toolbar-right">
                    <CreateElementButton
                        route={props.route}
                        label={props.route.create_button_label()}
                        on_create={props.on_create.clone()}
                    />
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

#[derive(Clone, Debug, PartialEq, Properties)]
struct DateRangeDropdownProps {
    selected: ReportDateRange,
    on_change: Callback<ReportDateRange>,
}

#[function_component(DateRangeDropdown)]
fn date_range_dropdown(props: &DateRangeDropdownProps) -> Html {
    html! {
        <div>
            <ul class="uk-subnav uk-subnav-pill" uk-margin="">
                <li>
                    <a class="abe-dropdown-label" href="#">
                        { render_icon("calendar") }
                        { props.selected.label() }
                        <span uk-icon="icon: triangle-down"></span>
                    </a>
                    <div uk-dropdown="mode: click;">
                        <ul class="uk-nav uk-dropdown-nav">
                            { for DATE_RANGE_OPTIONS.iter().map(|option| {
                                let option = *option;
                                let on_change = props.on_change.clone();
                                let selected = props.selected == option;
                                let onclick = Callback::from(move |event: MouseEvent| {
                                    event.prevent_default();
                                    on_change.emit(option);
                                });
                                html! {
                                    <li class={classes!(selected.then_some("uk-active"))}>
                                        <a href="#" onclick={onclick}>{ option.label() }</a>
                                    </li>
                                }
                            }) }
                        </ul>
                    </div>
                </li>
            </ul>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct SearchControlProps {
    value: String,
    on_search: Callback<String>,
}

#[function_component(SearchControl)]
fn search_control(props: &SearchControlProps) -> Html {
    let on_search = props.on_search.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let input: HtmlInputElement = event.target_unchecked_into();
        on_search.emit(input.value());
    });

    html! {
        <div class="uk-search uk-search-small abe-search">
            <span uk-search-icon=""></span>
            <input
                class="uk-search-input"
                type="search"
                placeholder="     Search..."
                value={props.value.clone()}
                oninput={oninput}
            />
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct ToolbarButtonProps {
    label: &'static str,
    icon: &'static str,
    #[prop_or(false)]
    disabled: bool,
    #[prop_or_default]
    onclick: Callback<MouseEvent>,
}

#[function_component(ToolbarButton)]
fn toolbar_button(props: &ToolbarButtonProps) -> Html {
    let disabled_class = props.disabled.then_some("uk-disabled");

    html! {
        <button
            class={classes!("uk-button", "uk-button-default", "uk-button-small", "uk-background-primary", "uk-light", "abe-button", disabled_class)}
            disabled={props.disabled}
            uk-tooltip={props.disabled.then_some("title: Not built yet")}
            onclick={props.onclick.clone()}
        >
            { render_icon(props.icon) }
            { props.label }
        </button>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct CreateElementButtonProps {
    route: Route,
    label: Option<&'static str>,
    on_create: Callback<Route>,
}

#[function_component(CreateElementButton)]
fn create_element_button(props: &CreateElementButtonProps) -> Html {
    match props.label {
        Some(label) => {
            let route = props.route;
            let on_create = props.on_create.clone();
            let onclick = Callback::from(move |_| on_create.emit(route));

            html! { <ToolbarButton label={label} icon="plus" onclick={onclick} /> }
        }
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
