use yew::prelude::*;

use crate::route::Route;
use crate::state::report::ReportState;
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
                            <ReportToolbar route={route} report={report} />
                            <ReportTable report={report} />
                        </main>
                    }
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
