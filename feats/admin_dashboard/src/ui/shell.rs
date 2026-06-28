use ad_buy_engine_domain::EntityRow;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::client;
use crate::route::Route;
use crate::state::entity_form::EntityKind;
use crate::state::report::{
    ReportDateRange, ReportState, default_grouping_for_route, filter_rows_by_search,
};
use crate::ui::create_modal::CreateModal;
use crate::ui::navigation_bar::NavigationBar;
use crate::ui::report_table::ReportTable;
use crate::ui::report_toolbar::ReportToolbar;
use crate::ui::settings_page::GeolocationSettingsPage;
use crate::ui::top_bar::TopBar;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShellProps {
    pub route: Route,
    pub on_logout: Callback<()>,
}

#[function_component(Shell)]
pub fn shell(props: &ShellProps) -> Html {
    let route = props.route.render_route();
    let navigator = use_navigator();
    let opened_editor = use_state(|| None::<EditorState>);
    let rows = use_state(Vec::<EntityRow>::new);
    let loading_rows = use_state(|| false);
    let row_error = use_state(|| None::<String>);
    let refresh_version = use_state(|| 0_u64);
    let date_range = use_state(|| ReportDateRange::Today);
    let search_query = use_state(String::new);
    let first_grouping = use_state(|| default_grouping_for_route(route));

    {
        let search_query = search_query.clone();
        let first_grouping = first_grouping.clone();
        use_effect_with(route, move |_| {
            search_query.set(String::new());
            first_grouping.set(default_grouping_for_route(route));
            || ()
        });
    }

    {
        let rows = rows.clone();
        let loading_rows = loading_rows.clone();
        let row_error = row_error.clone();
        let refresh_version = *refresh_version;
        let selected_date_range = *date_range;
        let selected_first_grouping = *first_grouping;
        use_effect_with(
            (
                route,
                refresh_version,
                selected_date_range,
                selected_first_grouping,
            ),
            move |(route, _, selected_date_range, selected_first_grouping)| {
                let route = *route;
                let selected_first_grouping = *selected_first_grouping;
                if selected_first_grouping.route_path().is_none() {
                    loading_rows.set(true);
                    row_error.set(None);
                    let selected_date_range = *selected_date_range;
                    spawn_local(async move {
                        match client::list_dimension_rows(
                            selected_first_grouping,
                            selected_date_range,
                        )
                        .await
                        {
                            Ok(loaded_rows) => rows.set(loaded_rows),
                            Err(message) => row_error.set(Some(message)),
                        }
                        loading_rows.set(false);
                    });
                } else if let Some(kind) = EntityKind::from_route(route) {
                    loading_rows.set(true);
                    row_error.set(None);
                    let selected_date_range = *selected_date_range;
                    spawn_local(async move {
                        match client::list_rows(kind, selected_date_range).await {
                            Ok(loaded_rows) => rows.set(loaded_rows),
                            Err(message) => row_error.set(Some(message)),
                        }
                        loading_rows.set(false);
                    });
                } else if route.report_rows_endpoint().is_some() {
                    loading_rows.set(true);
                    row_error.set(None);
                    let selected_date_range = *selected_date_range;
                    spawn_local(async move {
                        match client::list_report_rows(route, selected_date_range).await {
                            Ok(loaded_rows) => rows.set(loaded_rows),
                            Err(message) => row_error.set(Some(message)),
                        }
                        loading_rows.set(false);
                    });
                } else {
                    rows.set(Vec::new());
                    loading_rows.set(false);
                    row_error.set(None);
                }
                || ()
            },
        );
    }

    let open_create_modal = {
        let opened_editor = opened_editor.clone();
        Callback::from(move |route| {
            opened_editor.set(Some(EditorState {
                route,
                edit_id: None,
            }))
        })
    };
    let close_create_modal = {
        let opened_editor = opened_editor.clone();
        Callback::from(move |_| opened_editor.set(None))
    };
    let on_saved = {
        let opened_editor = opened_editor.clone();
        let refresh_version = refresh_version.clone();
        Callback::from(move |_| {
            opened_editor.set(None);
            refresh_version.set(*refresh_version + 1);
        })
    };
    let on_edit = {
        let opened_editor = opened_editor.clone();
        Callback::from(move |id| {
            opened_editor.set(Some(EditorState {
                route,
                edit_id: Some(id),
            }))
        })
    };
    let on_archive = {
        let refresh_version = refresh_version.clone();
        let row_error = row_error.clone();
        Callback::from(move |id: String| {
            let Some(kind) = EntityKind::from_route(route) else {
                return;
            };
            let refresh_version = refresh_version.clone();
            let row_error = row_error.clone();
            spawn_local(async move {
                match client::archive_entity(kind, id).await {
                    Ok(()) => refresh_version.set(*refresh_version + 1),
                    Err(message) => row_error.set(Some(message)),
                }
            });
        })
    };
    let on_refresh = {
        let refresh_version = refresh_version.clone();
        Callback::from(move |_| refresh_version.set(*refresh_version + 1))
    };
    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |query| search_query.set(query))
    };
    let on_date_range_change = {
        let date_range = date_range.clone();
        Callback::from(move |selected_date_range| date_range.set(selected_date_range))
    };
    let on_first_grouping_change = {
        let first_grouping = first_grouping.clone();
        let navigator = navigator.clone();
        Callback::from(move |dimension| {
            if let Some(target_route) = Route::from_report_dimension(dimension) {
                if target_route != route {
                    if let Some(navigator) = navigator.as_ref() {
                        navigator.push(&target_route);
                    }
                    return;
                }
            }
            first_grouping.set(dimension);
        })
    };
    let visible_rows = filter_rows_by_search((*rows).as_slice(), search_query.as_str());

    html! {
        <div class="abe-app">
            <TopBar on_logout={props.on_logout.clone()} />
            <NavigationBar active_route={route} />
            {
                if route.is_dashboard() {
                    html! { <DashboardPage /> }
                } else if route == Route::GeolocationSettings {
                    html! { <GeolocationSettingsPage /> }
                } else {
                    let mut report = ReportState::for_route(route, *first_grouping);
                    report.date_range = *date_range;
                    let actions_enabled = EntityKind::from_route(route).is_some()
                        && *first_grouping == default_grouping_for_route(route);
                    html! {
                        <main class="abe-report">
                            <ReportToolbar
                                route={route}
                                report={report}
                                search_query={(*search_query).clone()}
                                loading={*loading_rows}
                                on_create={open_create_modal}
                                on_search={on_search}
                                on_date_range_change={on_date_range_change}
                                on_first_grouping_change={on_first_grouping_change}
                                on_refresh={on_refresh}
                            />
                            {
                                row_error.as_ref().map(|message| html! {
                                    <p class="abe-inline-error">{ message }</p>
                                }).unwrap_or_default()
                            }
                            <ReportTable
                                report={report}
                                rows={visible_rows}
                                loading={*loading_rows}
                                actions_enabled={actions_enabled}
                                on_edit={on_edit}
                                on_archive={on_archive}
                            />
                        </main>
                    }
                }
            }
            {
                if let Some(editor) = (*opened_editor).clone() {
                    html! {
                        <CreateModal
                            route={editor.route}
                            edit_id={editor.edit_id}
                            on_close={close_create_modal}
                            on_saved={on_saved}
                        />
                    }
                } else {
                    Html::default()
                }
            }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EditorState {
    route: Route,
    edit_id: Option<String>,
}

#[function_component(DashboardPage)]
fn dashboard_page() -> Html {
    html! {
        <main class="abe-dashboard">
            <h1>{ "Dashboard" }</h1>
        </main>
    }
}
