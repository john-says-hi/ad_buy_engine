use yew::prelude::*;

use crate::route::Route;
use crate::state::report::ReportState;
use crate::ui::create_modal::CreateModal;
use crate::ui::navigation_bar::NavigationBar;
use crate::ui::report_table::ReportTable;
use crate::ui::report_toolbar::ReportToolbar;
use crate::ui::top_bar::TopBar;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Properties)]
pub struct ShellProps {
    pub route: Route,
}

#[function_component(Shell)]
pub fn shell(props: &ShellProps) -> Html {
    let route = props.route.render_route();
    let opened_create_route = use_state(|| None::<Route>);
    let open_create_modal = {
        let opened_create_route = opened_create_route.clone();
        Callback::from(move |route| opened_create_route.set(Some(route)))
    };
    let close_create_modal = {
        let opened_create_route = opened_create_route.clone();
        Callback::from(move |_| opened_create_route.set(None))
    };

    html! {
        <div class="abe-app">
            <TopBar />
            <NavigationBar active_route={route} />
            {
                if route.is_dashboard() {
                    html! { <DashboardPage /> }
                } else {
                    let report = ReportState::for_route(route);
                    html! {
                        <main class="abe-report">
                            <ReportToolbar route={route} report={report} on_create={open_create_modal} />
                            <ReportTable report={report} />
                        </main>
                    }
                }
            }
            {
                if let Some(create_route) = *opened_create_route {
                    html! { <CreateModal route={create_route} on_close={close_create_modal} /> }
                } else {
                    Html::default()
                }
            }
        </div>
    }
}

#[function_component(DashboardPage)]
fn dashboard_page() -> Html {
    html! {
        <main class="abe-dashboard">
            <h1>{ "Dashboard" }</h1>
        </main>
    }
}
