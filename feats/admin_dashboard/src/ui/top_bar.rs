use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TopBarProps {
    pub on_logout: Callback<()>,
}

#[function_component(TopBar)]
pub fn top_bar(props: &TopBarProps) -> Html {
    let navigator = use_navigator();
    let on_logout = props.on_logout.clone();
    let logout = Callback::from(move |_| on_logout.emit(()));
    let open_settings = Callback::from(move |_| {
        if let Some(navigator) = navigator.as_ref() {
            navigator.push(&Route::Settings);
        }
    });

    html! {
        <nav class="uk-navbar-container abe-top-bar" uk-navbar="">
            <div class="uk-navbar-left">
                <Link<Route> classes="uk-navbar-item uk-logo abe-brand-link" to={Route::Dashboard}>
                    <img class="logo" src="/assets/logo.svg" alt="Ad Buy Engine logo" />
                    <span class="abe-brand-title">{ "Ad Buy Engine" }</span>
                </Link<Route>>
            </div>

            <div class="uk-navbar-right">
                <ul class="uk-navbar-nav abe-top-status">
                    <li class="uk-navbar-item">
                        <div class="abe-status-pill" uk-tooltip="title: This Feature is Not Built Yet">
                            <span uk-icon="icon: bolt"></span>
                            <span>{ "Fuel: " }</span>
                            <a>{ "9321351" }</a>
                        </div>
                    </li>
                    <li class="uk-navbar-item">
                        <div class="abe-status-pill" uk-tooltip="title: This Feature is Not Built Yet">
                            <span uk-icon="icon: bell"></span>
                        </div>
                    </li>
                    <li class="uk-navbar-item">
                        <div class="abe-status-pill" uk-tooltip="title: This Feature is Not Built Yet">
                            <span uk-icon="icon: info"></span>
                        </div>
                    </li>
                    <li class="uk-navbar-item">
                        <button
                            class="abe-status-pill abe-status-button"
                            type="button"
                            aria-label="Settings"
                            uk-tooltip="title: Settings"
                            onclick={open_settings}
                        >
                            <span uk-icon="icon: cog"></span>
                            <span>{ "Settings" }</span>
                        </button>
                    </li>
                    <li class="uk-navbar-item">
                        <button class="uk-button uk-button-default uk-button-small abe-button" type="button" onclick={logout}>
                            <span uk-icon="icon: sign-out"></span>
                            { "Logout" }
                        </button>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
