use ad_buy_engine_domain::SessionResponse;
use yew::TargetCast;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::client;
use crate::route::Route;
use crate::ui::shell::Shell;

#[function_component(App)]
pub fn app() -> Html {
    let session = use_state(|| None::<SessionResponse>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let session = session.clone();
        let loading = loading.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match client::get_session().await {
                    Ok(response) => session.set(Some(response)),
                    Err(message) => error.set(Some(message)),
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_session = {
        let session = session.clone();
        Callback::from(move |response| session.set(Some(response)))
    };
    let on_logout = {
        let session = session.clone();
        Callback::from(move |_| {
            let session = session.clone();
            spawn_local(async move {
                let _ = client::logout().await;
                session.set(Some(SessionResponse {
                    authenticated: false,
                    username: None,
                    must_change_credentials: false,
                }));
            });
        })
    };

    if *loading {
        return html! { <FullPageStatus message="Loading" /> };
    }

    if let Some(message) = error.as_ref() {
        return html! { <FullPageStatus message={message.clone()} /> };
    }

    let current_session = (*session).clone().unwrap_or(SessionResponse {
        authenticated: false,
        username: None,
        must_change_credentials: false,
    });

    if !current_session.authenticated {
        return html! { <LoginPage on_session={on_session} /> };
    }

    if current_session.must_change_credentials {
        return html! { <CredentialChangePage on_session={on_session} /> };
    }

    html! {
        <BrowserRouter>
            <Switch<Route> render={move |route| switch_with_logout(route, on_logout.clone())} />
        </BrowserRouter>
    }
}

fn switch_with_logout(route: Route, on_logout: Callback<()>) -> Html {
    html! { <Shell route={route.render_route()} on_logout={on_logout} /> }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct LoginPageProps {
    on_session: Callback<SessionResponse>,
}

#[function_component(LoginPage)]
fn login_page(props: &LoginPageProps) -> Html {
    let username = use_state(|| "admin".to_string());
    let password = use_state(|| "admin".to_string());
    let error = use_state(|| None::<String>);
    let submitting = use_state(|| false);
    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let submitting = submitting.clone();
        let on_session = props.on_session.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let username = (*username).clone();
            let password = (*password).clone();
            let error = error.clone();
            let submitting = submitting.clone();
            let on_session = on_session.clone();
            submitting.set(true);
            error.set(None);
            spawn_local(async move {
                match client::login(username, password).await {
                    Ok(response) => on_session.emit(response),
                    Err(message) => error.set(Some(message)),
                }
                submitting.set(false);
            });
        })
    };

    html! {
        <AuthFrame title="Operator Login">
            <form class="abe-auth-form" {onsubmit}>
                <AuthInput label="Username" value={(*username).clone()} state={username.clone()} input_type="text" />
                <AuthInput label="Password" value={(*password).clone()} state={password.clone()} input_type="password" />
                { render_auth_error(&error) }
                <button class="uk-button uk-button-primary" type="submit" disabled={*submitting}>
                    { if *submitting { "Signing in..." } else { "Sign In" } }
                </button>
            </form>
        </AuthFrame>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct CredentialChangeProps {
    on_session: Callback<SessionResponse>,
}

#[function_component(CredentialChangePage)]
fn credential_change_page(props: &CredentialChangeProps) -> Html {
    let current_password = use_state(|| "admin".to_string());
    let new_username = use_state(|| "admin".to_string());
    let new_password = use_state(String::new);
    let error = use_state(|| None::<String>);
    let submitting = use_state(|| false);
    let onsubmit = {
        let current_password = current_password.clone();
        let new_username = new_username.clone();
        let new_password = new_password.clone();
        let error = error.clone();
        let submitting = submitting.clone();
        let on_session = props.on_session.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let current_password = (*current_password).clone();
            let new_username = (*new_username).clone();
            let new_password = (*new_password).clone();
            let error = error.clone();
            let submitting = submitting.clone();
            let on_session = on_session.clone();
            submitting.set(true);
            error.set(None);
            spawn_local(async move {
                match client::update_credentials(current_password, new_username, new_password).await
                {
                    Ok(response) => on_session.emit(response),
                    Err(message) => error.set(Some(message)),
                }
                submitting.set(false);
            });
        })
    };

    html! {
        <AuthFrame title="Change Credentials">
            <form class="abe-auth-form" {onsubmit}>
                <AuthInput label="Current Password" value={(*current_password).clone()} state={current_password.clone()} input_type="password" />
                <AuthInput label="New Username" value={(*new_username).clone()} state={new_username.clone()} input_type="text" />
                <AuthInput label="New Password" value={(*new_password).clone()} state={new_password.clone()} input_type="password" />
                { render_auth_error(&error) }
                <button class="uk-button uk-button-primary" type="submit" disabled={*submitting}>
                    { if *submitting { "Saving..." } else { "Save Credentials" } }
                </button>
            </form>
        </AuthFrame>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct AuthInputProps {
    label: &'static str,
    value: String,
    state: UseStateHandle<String>,
    input_type: &'static str,
}

#[function_component(AuthInput)]
fn auth_input(props: &AuthInputProps) -> Html {
    let state = props.state.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let input: web_sys::HtmlInputElement = event.target_unchecked_into();
        state.set(input.value());
    });
    html! {
        <label class="abe-auth-field">
            <span>{ props.label }</span>
            <input class="uk-input" type={props.input_type} value={props.value.clone()} {oninput} />
        </label>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct AuthFrameProps {
    title: &'static str,
    children: Children,
}

#[function_component(AuthFrame)]
fn auth_frame(props: &AuthFrameProps) -> Html {
    html! {
        <main class="abe-auth-page">
            <section class="abe-auth-panel">
                <img class="abe-auth-logo" src="/assets/logo.svg" alt="Ad Buy Engine logo" />
                <h1>{ props.title }</h1>
                { for props.children.iter() }
            </section>
        </main>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct FullPageStatusProps {
    message: String,
}

#[function_component(FullPageStatus)]
fn full_page_status(props: &FullPageStatusProps) -> Html {
    html! {
        <main class="abe-auth-page">
            <section class="abe-auth-panel">
                <p>{ props.message.clone() }</p>
            </section>
        </main>
    }
}

fn render_auth_error(error: &UseStateHandle<Option<String>>) -> Html {
    error
        .as_ref()
        .map(|message| html! { <p class="abe-form-error">{ message }</p> })
        .unwrap_or_default()
}
