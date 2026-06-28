use ad_buy_engine_domain::{
    GeolocationDatabaseStatus, GeolocationSettingsResponse, GeolocationSettingsUpdate,
};
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::client;

#[function_component(GeolocationSettingsPage)]
pub fn geolocation_settings_page() -> Html {
    let settings = use_state(|| None::<GeolocationSettingsResponse>);
    let account_id = use_state(String::new);
    let license_key = use_state(String::new);
    let city_path = use_state(String::new);
    let country_path = use_state(String::new);
    let asn_path = use_state(String::new);
    let loading = use_state(|| true);
    let saving = use_state(|| false);
    let message = use_state(|| None::<String>);

    {
        let settings = settings.clone();
        let account_id = account_id.clone();
        let city_path = city_path.clone();
        let country_path = country_path.clone();
        let asn_path = asn_path.clone();
        let loading = loading.clone();
        let message = message.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match client::get_geolocation_settings().await {
                    Ok(response) => {
                        account_id.set(response.account_id.clone());
                        city_path.set(response.city_database_path.clone());
                        country_path.set(response.country_database_path.clone());
                        asn_path.set(response.asn_database_path.clone());
                        settings.set(Some(response));
                    }
                    Err(error) => message.set(Some(error)),
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_save = {
        let account_id = account_id.clone();
        let license_key = license_key.clone();
        let city_path = city_path.clone();
        let country_path = country_path.clone();
        let asn_path = asn_path.clone();
        let settings = settings.clone();
        let saving = saving.clone();
        let message = message.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            saving.set(true);
            message.set(None);
            let update = GeolocationSettingsUpdate {
                account_id: (*account_id).clone(),
                license_key: if license_key.trim().is_empty() {
                    None
                } else {
                    Some((*license_key).clone())
                },
                city_database_path: (*city_path).clone(),
                country_database_path: (*country_path).clone(),
                asn_database_path: (*asn_path).clone(),
            };
            let settings = settings.clone();
            let saving = saving.clone();
            let message = message.clone();
            let license_key = license_key.clone();
            spawn_local(async move {
                match client::save_geolocation_settings(update).await {
                    Ok(response) => {
                        license_key.set(String::new());
                        settings.set(Some(response));
                        message.set(Some("Saved".to_string()));
                    }
                    Err(error) => message.set(Some(error)),
                }
                saving.set(false);
            });
        })
    };

    let on_download = {
        let settings = settings.clone();
        let saving = saving.clone();
        let message = message.clone();
        Callback::from(move |_| {
            saving.set(true);
            message.set(None);
            let settings = settings.clone();
            let saving = saving.clone();
            let message = message.clone();
            spawn_local(async move {
                match client::download_geolite_databases().await {
                    Ok(response) => {
                        message.set(Some(response.message));
                        match client::get_geolocation_settings().await {
                            Ok(response) => settings.set(Some(response)),
                            Err(error) => message.set(Some(error)),
                        }
                    }
                    Err(error) => message.set(Some(error)),
                }
                saving.set(false);
            });
        })
    };

    html! {
        <main class="abe-report">
            <section class="abe-settings-panel">
                <h1>{ "Geo Settings" }</h1>
                {
                    if *loading {
                        html! { <p>{ "Loading..." }</p> }
                    } else {
                        html! {
                            <form class="abe-settings-form" onsubmit={on_save}>
                                { settings_message(&message) }
                                { text_input("MaxMind Account ID", &account_id, false) }
                                { text_input("MaxMind License Key", &license_key, true) }
                                { license_key_status(settings.as_ref()) }
                                { text_input("City Database", &city_path, false) }
                                { text_input("Country Database", &country_path, false) }
                                { text_input("ASN Database", &asn_path, false) }
                                <div class="abe-settings-actions">
                                    <button class="uk-button uk-button-primary" type="submit" disabled={*saving}>{ "Save" }</button>
                                    <button class="uk-button uk-button-default" type="button" disabled={*saving} onclick={on_download}>{ "Download" }</button>
                                </div>
                            </form>
                        }
                    }
                }
                { settings.as_ref().map(status_table).unwrap_or_default() }
            </section>
        </main>
    }
}

fn text_input(label: &'static str, value: &UseStateHandle<String>, password: bool) -> Html {
    let state = value.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let input: HtmlInputElement = event.target_unchecked_into();
        state.set(input.value());
    });
    html! {
        <label class="abe-settings-field">
            <span>{ label }</span>
            <input
                class="uk-input"
                type={if password { "password" } else { "text" }}
                value={(**value).clone()}
                oninput={oninput}
            />
        </label>
    }
}

fn settings_message(message: &UseStateHandle<Option<String>>) -> Html {
    message
        .as_ref()
        .map(|message| html! { <p class="abe-inline-error">{ message }</p> })
        .unwrap_or_default()
}

fn license_key_status(settings: Option<&GeolocationSettingsResponse>) -> Html {
    let Some(settings) = settings else {
        return Html::default();
    };
    if settings.license_key_configured {
        let preview = settings
            .license_key_preview
            .clone()
            .unwrap_or_else(|| "configured".to_string());
        html! { <p class="abe-settings-hint">{ format!("Saved license key: {preview}") }</p> }
    } else {
        html! { <p class="abe-settings-hint">{ "No saved license key" }</p> }
    }
}

fn status_table(settings: &GeolocationSettingsResponse) -> Html {
    html! {
        <table class="uk-table uk-table-divider uk-table-small abe-table abe-settings-status">
            <thead>
                <tr>
                    <th>{ "Database" }</th>
                    <th>{ "Status" }</th>
                    <th>{ "Build" }</th>
                    <th>{ "Path" }</th>
                </tr>
            </thead>
            <tbody>
                { status_row("City", &settings.city_database) }
                { status_row("Country", &settings.country_database) }
                { status_row("ASN", &settings.asn_database) }
            </tbody>
        </table>
    }
}

fn status_row(label: &'static str, status: &GeolocationDatabaseStatus) -> Html {
    let state = if let Some(error) = status.error.as_ref() {
        error.clone()
    } else if status.exists {
        status
            .database_type
            .clone()
            .unwrap_or_else(|| "Loaded".to_string())
    } else {
        "Missing".to_string()
    };
    let build_epoch = status
        .build_epoch
        .map(|build_epoch| build_epoch.to_string())
        .unwrap_or_else(|| "-".to_string());
    html! {
        <tr>
            <td>{ label }</td>
            <td>{ state }</td>
            <td>{ build_epoch }</td>
            <td>{ status.configured_path.clone() }</td>
        </tr>
    }
}
