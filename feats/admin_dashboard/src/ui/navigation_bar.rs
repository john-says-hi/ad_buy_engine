use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::{NAVIGATION_ITEMS, Route};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Properties)]
pub struct NavigationBarProps {
    pub active_route: Route,
}

#[function_component(NavigationBar)]
pub fn navigation_bar(props: &NavigationBarProps) -> Html {
    html! {
        <div class="uk-grid-collapse uk-background-default" uk-grid="">
            <nav class="uk-navbar-container abe-main-nav" uk-navbar="mode: click">
                <div class="uk-navbar-left">
                    <ul class="uk-navbar-nav uk-flex-wrap uk-flex-center">
                        { for NAVIGATION_ITEMS.iter().map(|item| html! {
                            <li class={classes!((item.route == props.active_route).then_some("uk-active"))}>
                                <Link<Route> to={item.route}>
                                    <span uk-icon={format!("icon: {}", item.icon)}></span>
                                    <span>{ item.label }</span>
                                </Link<Route>>
                            </li>
                        }) }
                    </ul>
                </div>
            </nav>
        </div>
    }
}
