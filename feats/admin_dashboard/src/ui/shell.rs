use ad_buy_engine_domain::EntityRow;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::client;
use crate::route::Route;
use crate::state::entity_form::EntityKind;
use crate::state::report::ReportState;
use crate::ui::create_modal::CreateModal;
use crate::ui::navigation_bar::NavigationBar;
use crate::ui::report_table::ReportTable;
use crate::ui::report_toolbar::ReportToolbar;
use crate::ui::top_bar::TopBar;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShellProps {
    pub route: Route,
    pub on_logout: Callback<()>,
}

#[function_component(Shell)]
pub fn shell(props: &ShellProps) -> Html {
    let route = props.route.render_route();
    let opened_editor = use_state(|| None::<EditorState>);
    let rows = use_state(Vec::<EntityRow>::new);
    let loading_rows = use_state(|| false);
    let row_error = use_state(|| None::<String>);
    let refresh_version = use_state(|| 0_u64);

    {
        let rows = rows.clone();
        let loading_rows = loading_rows.clone();
        let row_error = row_error.clone();
        let refresh_version = *refresh_version;
        use_effect_with((route, refresh_version), move |(route, _)| {
            let route = *route;
            if let Some(kind) = EntityKind::from_route(route) {
                loading_rows.set(true);
                row_error.set(None);
                spawn_local(async move {
                    match client::list_rows(kind).await {
                        Ok(loaded_rows) => rows.set(loaded_rows),
                        Err(message) => row_error.set(Some(message)),
                    }
                    loading_rows.set(false);
                });
            } else {
                rows.set(Vec::new());
            }
            || ()
        });
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

    html! {
        <div class="abe-app">
            <TopBar on_logout={props.on_logout.clone()} />
            <NavigationBar active_route={route} />
            {
                if route.is_dashboard() {
                    html! { <DashboardPage /> }
                } else {
                    let report = ReportState::for_route(route);
                    html! {
                        <main class="abe-report">
                            <ReportToolbar route={route} report={report} on_create={open_create_modal} />
                            {
                                row_error.as_ref().map(|message| html! {
                                    <p class="abe-inline-error">{ message }</p>
                                }).unwrap_or_default()
                            }
                            <ReportTable
                                report={report}
                                rows={(*rows).clone()}
                                loading={*loading_rows}
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
