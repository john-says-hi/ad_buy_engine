use yew::TargetCast;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::client;
use crate::route::Route;
use crate::state::entity_form::{
    EntityKind, FieldType, FormOptionLists, FormValues, SelectSource, default_values,
    draft_from_values, form_fields, values_from_record,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateModalProps {
    pub route: Route,
    #[prop_or_default]
    pub edit_id: Option<String>,
    pub on_close: Callback<()>,
    pub on_saved: Callback<()>,
}

#[function_component(CreateModal)]
pub fn create_modal(props: &CreateModalProps) -> Html {
    let Some(kind) = EntityKind::from_route(props.route) else {
        return Html::default();
    };
    let values = use_state(|| default_values(kind));
    let options = use_state(FormOptionLists::default);
    let loading = use_state(|| true);
    let saving = use_state(|| false);
    let error = use_state(|| None::<String>);

    {
        let values = values.clone();
        let options = options.clone();
        let loading = loading.clone();
        let error = error.clone();
        let edit_id = props.edit_id.clone();
        use_effect_with((kind, edit_id), move |(kind, edit_id)| {
            let kind = *kind;
            let edit_id = edit_id.clone();
            values.set(default_values(kind));
            loading.set(true);
            error.set(None);
            spawn_local(async move {
                match client::load_options().await {
                    Ok(loaded_options) => options.set(loaded_options),
                    Err(message) => error.set(Some(message)),
                }
                if let Some(id) = edit_id {
                    match client::get_entity(kind, &id).await {
                        Ok(record) => values.set(values_from_record(record)),
                        Err(message) => error.set(Some(message)),
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };
    let save = {
        let values = values.clone();
        let saving = saving.clone();
        let error = error.clone();
        let edit_id = props.edit_id.clone();
        let on_saved = props.on_saved.clone();
        Callback::from(move |_| {
            let values = (*values).clone();
            let draft = draft_from_values(kind, &values);
            let saving = saving.clone();
            let error = error.clone();
            let edit_id = edit_id.clone();
            let on_saved = on_saved.clone();
            match draft {
                Ok(draft) => {
                    saving.set(true);
                    error.set(None);
                    spawn_local(async move {
                        match client::save_entity(kind, edit_id, draft).await {
                            Ok(_) => on_saved.emit(()),
                            Err(message) => error.set(Some(message)),
                        }
                        saving.set(false);
                    });
                }
                Err(message) => error.set(Some(message)),
            }
        })
    };
    let submit = {
        let save = save.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            save.emit(());
        })
    };
    let modal_id = format!("{}-modal", props.route.path().trim_start_matches('/'));
    let title = if props.edit_id.is_some() {
        format!("Edit {}", kind.title())
    } else {
        kind.title().to_string()
    };

    html! {
        <div class="abe-modal-backdrop" role="presentation">
            <section
                id={modal_id.clone()}
                class="abe-create-modal"
                role="dialog"
                aria-modal="true"
                aria-labelledby={format!("{modal_id}-title")}
            >
                <button
                    class={classes!("uk-modal-close-default", "abe-modal-close")}
                    type="button"
                    uk-close=""
                    aria-label="Close create form"
                    onclick={close.clone()}
                >
                </button>

                <header class="uk-modal-header">
                    <h2 id={format!("{modal_id}-title")} class="uk-modal-title uk-text-center">
                        { title }
                    </h2>
                </header>

                <form class="abe-create-form" onsubmit={submit}>
                    <div class="uk-modal-body abe-create-modal-body">
                        {
                            if *loading {
                                html! { <p class="abe-inline-status">{ "Loading..." }</p> }
                            } else {
                                html! {
                                    <section class="abe-create-section">
                                        <div class="abe-create-field-grid">
                                            { for form_fields(kind).iter().map(|field| {
                                                render_field(field, &values, &options)
                                            }) }
                                        </div>
                                    </section>
                                }
                            }
                        }
                        {
                            error.as_ref().map(|message| html! {
                                <p class="abe-form-error">{ message }</p>
                            }).unwrap_or_default()
                        }
                    </div>

                    <footer class="uk-modal-footer uk-text-right">
                        <button class="uk-button uk-button-default" type="button" onclick={close}>
                            { "Cancel" }
                        </button>
                        <button
                            class="uk-button uk-button-primary"
                            type="submit"
                            disabled={*saving || *loading}
                        >
                            { if *saving { "Saving..." } else { "Save" } }
                        </button>
                    </footer>
                </form>
            </section>
        </div>
    }
}

fn render_field(
    field: &crate::state::entity_form::FormFieldSpec,
    values: &UseStateHandle<FormValues>,
    options: &FormOptionLists,
) -> Html {
    match field.field_type {
        FieldType::Text => render_text_input("text", field, values),
        FieldType::Number => render_number_input(field, values, "1"),
        FieldType::Decimal => render_number_input(field, values, "0.01"),
        FieldType::TextArea => {
            let key = field.key.to_string();
            let value = values.text(field.key);
            let values = values.clone();
            let oninput = Callback::from(move |event: InputEvent| {
                let input: web_sys::HtmlTextAreaElement = event.target_unchecked_into();
                let mut next = (*values).clone();
                next.set_text(&key, input.value());
                values.set(next);
            });
            html! {
                <label class={field_classes(field.wide)}>
                    { render_field_label(field.label) }
                    <textarea class="uk-textarea" rows="4" value={value} {oninput} />
                </label>
            }
        }
        FieldType::Toggle => {
            let key = field.key.to_string();
            let checked = values.toggle(field.key);
            let values = values.clone();
            let onchange = Callback::from(move |event: Event| {
                let input: web_sys::HtmlInputElement = event.target_unchecked_into();
                let mut next = (*values).clone();
                next.set_toggle(&key, input.checked());
                values.set(next);
            });
            html! {
                <label class="abe-create-toggle">
                    <input class="uk-checkbox" type="checkbox" checked={checked} {onchange} />
                    <span>{ field.label }</span>
                </label>
            }
        }
        FieldType::Select(source) => render_select(field, values, options, source),
    }
}

fn render_text_input(
    input_type: &'static str,
    field: &crate::state::entity_form::FormFieldSpec,
    values: &UseStateHandle<FormValues>,
) -> Html {
    let key = field.key.to_string();
    let value = values.text(field.key);
    let values = values.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let input: web_sys::HtmlInputElement = event.target_unchecked_into();
        let mut next = (*values).clone();
        next.set_text(&key, input.value());
        values.set(next);
    });
    html! {
        <label class={field_classes(field.wide)}>
            { render_field_label(field.label) }
            <input class="uk-input" type={input_type} value={value} {oninput} />
        </label>
    }
}

fn render_number_input(
    field: &crate::state::entity_form::FormFieldSpec,
    values: &UseStateHandle<FormValues>,
    step: &'static str,
) -> Html {
    let key = field.key.to_string();
    let value = values.text(field.key);
    let values = values.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let input: web_sys::HtmlInputElement = event.target_unchecked_into();
        let mut next = (*values).clone();
        next.set_text(&key, input.value());
        values.set(next);
    });
    html! {
        <label class={field_classes(field.wide)}>
            { render_field_label(field.label) }
            <input class="uk-input" type="number" step={step} value={value} {oninput} />
        </label>
    }
}

fn render_select(
    field: &crate::state::entity_form::FormFieldSpec,
    values: &UseStateHandle<FormValues>,
    options: &FormOptionLists,
    source: SelectSource,
) -> Html {
    let key = field.key.to_string();
    let current = values.text(field.key);
    let selected_value = current.clone();
    let values = values.clone();
    let items = options.for_source(source);
    let onchange = Callback::from(move |event: Event| {
        let input: web_sys::HtmlSelectElement = event.target_unchecked_into();
        let mut next = (*values).clone();
        next.set_text(&key, input.value());
        values.set(next);
    });
    html! {
        <label class={field_classes(field.wide)}>
            { render_field_label(field.label) }
            <select class="uk-select" value={current} {onchange}>
                <option value="" selected={selected_value.is_empty()}>{ "Select" }</option>
                { for items.iter().map(|item| html! {
                    <option value={item.value.clone()} selected={item.value == selected_value}>{ item.label.clone() }</option>
                }) }
            </select>
        </label>
    }
}

fn field_classes(wide: bool) -> Classes {
    classes!("abe-create-field", wide.then_some("abe-create-field-wide"))
}

fn render_field_label(label: &'static str) -> Html {
    html! {
        <span class="uk-label uk-margin-small-bottom abe-create-label">
            { label }
        </span>
    }
}
