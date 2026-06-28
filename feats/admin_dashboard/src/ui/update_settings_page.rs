use ad_buy_engine_domain::{UpdatePhase, UpdateStatusResponse};
use gloo_timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::client;

const POLL_MILLIS: u32 = 3000;

#[function_component(UpdateSettingsPage)]
pub fn update_settings_page() -> Html {
    let status = use_state(|| None::<UpdateStatusResponse>);
    let loading = use_state(|| true);
    let busy = use_state(|| false);
    let message = use_state(|| None::<String>);
    let password = use_state(String::new);
    let install_confirmation = use_state(String::new);
    let rollback_confirmation = use_state(String::new);
    let refresh_version = use_state(|| 0_u64);

    {
        let status = status.clone();
        let loading = loading.clone();
        let message = message.clone();
        let refresh_version = *refresh_version;
        use_effect_with(refresh_version, move |_| {
            spawn_local(async move {
                match client::get_update_status().await {
                    Ok(response) => status.set(Some(response)),
                    Err(error) => message.set(Some(error)),
                }
                loading.set(false);
            });
            || ()
        });
    }

    {
        let refresh_version = refresh_version.clone();
        let running = status
            .as_ref()
            .map(|current| current.phase.is_running())
            .unwrap_or(false);
        use_effect_with(running, move |running| {
            let interval = if *running {
                let refresh_version = refresh_version.clone();
                Some(Interval::new(POLL_MILLIS, move || {
                    refresh_version.set(*refresh_version + 1);
                }))
            } else {
                None
            };
            move || drop(interval)
        });
    }

    let on_check = {
        let status = status.clone();
        let busy = busy.clone();
        let message = message.clone();
        Callback::from(move |_| {
            busy.set(true);
            message.set(None);
            let status = status.clone();
            let busy = busy.clone();
            let message = message.clone();
            spawn_local(async move {
                match client::check_updates().await {
                    Ok(response) => status.set(Some(response)),
                    Err(error) => message.set(Some(error)),
                }
                busy.set(false);
            });
        })
    };

    let on_install = {
        let status = status.clone();
        let busy = busy.clone();
        let message = message.clone();
        let password = password.clone();
        let install_confirmation = install_confirmation.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if install_confirmation.as_str() != "INSTALL" {
                message.set(Some("Type INSTALL to confirm".to_string()));
                return;
            }
            busy.set(true);
            message.set(None);
            let current_password = (*password).clone();
            let requested_version = status
                .as_ref()
                .and_then(|current| current.latest_version.clone());
            let status = status.clone();
            let busy = busy.clone();
            let message = message.clone();
            spawn_local(async move {
                match client::start_update(current_password, requested_version).await {
                    Ok(response) => status.set(Some(response)),
                    Err(error) => message.set(Some(error)),
                }
                busy.set(false);
            });
        })
    };

    let on_rollback = {
        let status = status.clone();
        let busy = busy.clone();
        let message = message.clone();
        let password = password.clone();
        let rollback_confirmation = rollback_confirmation.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if rollback_confirmation.as_str() != "ROLLBACK" {
                message.set(Some("Type ROLLBACK to confirm".to_string()));
                return;
            }
            busy.set(true);
            message.set(None);
            let current_password = (*password).clone();
            let status = status.clone();
            let busy = busy.clone();
            let message = message.clone();
            spawn_local(async move {
                match client::rollback_update(current_password).await {
                    Ok(response) => status.set(Some(response)),
                    Err(error) => message.set(Some(error)),
                }
                busy.set(false);
            });
        })
    };

    let install_disabled = status
        .as_ref()
        .map(|current| !can_install(current) || *busy || install_confirmation.as_str() != "INSTALL")
        .unwrap_or(true);
    let rollback_disabled = status
        .as_ref()
        .map(|current| {
            !can_rollback(current) || *busy || rollback_confirmation.as_str() != "ROLLBACK"
        })
        .unwrap_or(true);

    html! {
        <main class="abe-report">
            <section class="abe-settings-panel">
                <h1>{ "Updates" }</h1>
                { render_message(&message) }
                {
                    if *loading {
                        html! { <p>{ "Loading..." }</p> }
                    } else if let Some(current) = status.as_ref() {
                        html! {
                            <>
                                { update_status_table(current) }
                                <div class="abe-settings-actions">
                                    <button class="uk-button uk-button-default" type="button" disabled={!current.enabled || current.phase.is_running() || *busy} onclick={on_check}>
                                        { if *busy { "Working..." } else { "Check" } }
                                    </button>
                                </div>
                                <form class="abe-settings-form" onsubmit={on_install}>
                                    { password_input("Operator Password", &password) }
                                    { text_input("Install Confirmation", &install_confirmation, "INSTALL") }
                                    <div class="abe-settings-actions">
                                        <button class="uk-button uk-button-primary" type="submit" disabled={install_disabled}>{ "Install" }</button>
                                    </div>
                                </form>
                                <form class="abe-settings-form" onsubmit={on_rollback}>
                                    { text_input("Rollback Confirmation", &rollback_confirmation, "ROLLBACK") }
                                    <div class="abe-settings-actions">
                                        <button class="uk-button uk-button-danger" type="submit" disabled={rollback_disabled}>{ "Rollback" }</button>
                                    </div>
                                </form>
                            </>
                        }
                    } else {
                        html! { <p class="abe-inline-error">{ "Update status is unavailable" }</p> }
                    }
                }
            </section>
        </main>
    }
}

pub fn phase_label(phase: UpdatePhase) -> &'static str {
    match phase {
        UpdatePhase::Disabled => "Disabled",
        UpdatePhase::Idle => "Idle",
        UpdatePhase::Checking => "Checking",
        UpdatePhase::UpdateRequested => "Queued",
        UpdatePhase::Downloading => "Downloading",
        UpdatePhase::Verifying => "Verifying",
        UpdatePhase::Staging => "Staging",
        UpdatePhase::StartingCandidate => "Starting",
        UpdatePhase::CandidateHealthCheck => "Candidate Health",
        UpdatePhase::SwitchingTraffic => "Switching Traffic",
        UpdatePhase::PublicHealthCheck => "Public Health",
        UpdatePhase::DrainingOldSlot => "Draining",
        UpdatePhase::Succeeded => "Succeeded",
        UpdatePhase::Failed => "Failed",
        UpdatePhase::RollbackRequested => "Rollback Queued",
        UpdatePhase::RollingBack => "Rolling Back",
        UpdatePhase::RolledBack => "Rolled Back",
        UpdatePhase::RollbackFailed => "Rollback Failed",
    }
}

pub fn can_install(status: &UpdateStatusResponse) -> bool {
    status.enabled && !status.phase.is_running()
}

pub fn can_rollback(status: &UpdateStatusResponse) -> bool {
    status.enabled && !status.phase.is_running() && status.rollback.eligible
}

fn update_status_table(status: &UpdateStatusResponse) -> Html {
    let rollback = rollback_text(status);
    html! {
        <table class="uk-table uk-table-divider uk-table-small abe-table abe-settings-status">
            <tbody>
                { status_row("Current Version", &status.current_version) }
                { status_row("Latest Version", status.latest_version.as_deref().unwrap_or("-")) }
                { status_row("Active Slot", status.active_slot.map(|slot| slot.as_str()).unwrap_or("-")) }
                { status_row("Phase", phase_label(status.phase)) }
                { status_row("Rollback", &rollback) }
                { status.last_result.as_ref().map(|result| status_row("Last Result", &result.message)).unwrap_or_default() }
            </tbody>
        </table>
    }
}

fn rollback_text(status: &UpdateStatusResponse) -> String {
    if status.rollback.eligible {
        status
            .rollback
            .target_version
            .clone()
            .unwrap_or_else(|| "Available".to_string())
    } else {
        status
            .rollback
            .reason
            .clone()
            .unwrap_or_else(|| "Unavailable".to_string())
    }
}

fn status_row(label: &'static str, value: &str) -> Html {
    html! {
        <tr>
            <td>{ label }</td>
            <td>{ value }</td>
        </tr>
    }
}

fn password_input(label: &'static str, value: &UseStateHandle<String>) -> Html {
    html! {
        <label class="abe-settings-field">
            <span>{ label }</span>
            <input class="uk-input" type="password" value={(**value).clone()} oninput={input_callback(value)} />
        </label>
    }
}

fn text_input(
    label: &'static str,
    value: &UseStateHandle<String>,
    placeholder: &'static str,
) -> Html {
    html! {
        <label class="abe-settings-field">
            <span>{ label }</span>
            <input class="uk-input" type="text" placeholder={placeholder} value={(**value).clone()} oninput={input_callback(value)} />
        </label>
    }
}

fn input_callback(value: &UseStateHandle<String>) -> Callback<InputEvent> {
    let value = value.clone();
    Callback::from(move |event: InputEvent| {
        let input: HtmlInputElement = event.target_unchecked_into();
        value.set(input.value());
    })
}

fn render_message(message: &UseStateHandle<Option<String>>) -> Html {
    message
        .as_ref()
        .map(|message| html! { <p class="abe-inline-error">{ message }</p> })
        .unwrap_or_default()
}
