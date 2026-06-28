use yew::prelude::*;

use crate::route::Route;
use crate::state::create_form::{CreateFormDefinition, CreateFormField, TokenTableRow};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateModalProps {
    pub route: Route,
    pub on_close: Callback<()>,
}

#[function_component(CreateModal)]
pub fn create_modal(props: &CreateModalProps) -> Html {
    let Some(form) = CreateFormDefinition::for_route(props.route) else {
        return Html::default();
    };

    let close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    html! {
        <div class="abe-modal-backdrop" role="presentation">
            <section
                id={form.modal_id}
                class="abe-create-modal"
                role="dialog"
                aria-modal="true"
                aria-labelledby={format!("{}-title", form.modal_id)}
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
                    <h2 id={format!("{}-title", form.modal_id)} class="uk-modal-title uk-text-center">
                        { form.title }
                    </h2>
                </header>

                <div class="uk-modal-body abe-create-modal-body">
                    <form class="abe-create-form">
                        { for form.sections.iter().map(render_section) }
                    </form>
                </div>

                <footer class="uk-modal-footer uk-text-right">
                    <button class="uk-button uk-button-default" type="button" onclick={close}>
                        { "Cancel" }
                    </button>
                    <button
                        class="uk-button uk-button-primary"
                        type="button"
                        disabled={true}
                        uk-tooltip="title: Static prototype only"
                    >
                        { "Save" }
                    </button>
                </footer>
            </section>
        </div>
    }
}

fn render_section(section: &crate::state::create_form::CreateFormSection) -> Html {
    html! {
        <section class="abe-create-section">
            {
                section.title.map(|title| html! {
                    <h4 class="abe-create-section-title">{ title }</h4>
                }).unwrap_or_default()
            }
            <div class="abe-create-field-grid">
                { for section.fields.iter().map(render_field) }
            </div>
        </section>
    }
}

fn render_field(field: &CreateFormField) -> Html {
    match *field {
        CreateFormField::Text {
            label,
            placeholder,
            value,
        } => render_input_field("text", label, placeholder, value),
        CreateFormField::Number {
            label,
            placeholder,
            value,
        } => render_input_field("number", label, placeholder, value),
        CreateFormField::Select {
            label,
            selected,
            options,
        } => html! {
            <label class="abe-create-field">
                { render_field_label(label) }
                <select class="uk-select" aria-label={label}>
                    { for options.iter().map(|option| html! {
                        <option selected={*option == selected}>{ *option }</option>
                    }) }
                </select>
            </label>
        },
        CreateFormField::TextArea {
            label,
            placeholder,
            value,
            rows,
        } => html! {
            <label class="abe-create-field abe-create-field-wide">
                { render_field_label(label) }
                <textarea
                    class="uk-textarea"
                    rows={rows.to_string()}
                    placeholder={placeholder}
                    value={value}
                />
            </label>
        },
        CreateFormField::Toggle { label, checked } => html! {
            <label class="abe-create-toggle">
                <input class="uk-checkbox" type="checkbox" checked={checked} />
                <span>{ label }</span>
            </label>
        },
        CreateFormField::RadioGroup {
            label,
            selected,
            options,
        } => html! {
            <fieldset class="abe-create-radio-group">
                <legend>{ render_field_label(label) }</legend>
                <div class="abe-create-radio-options">
                    { for options.iter().map(|option| html! {
                        <label>
                            <input class="uk-radio" type="radio" name={label} checked={*option == selected} />
                            <span>{ *option }</span>
                        </label>
                    }) }
                </div>
            </fieldset>
        },
        CreateFormField::TokenTable { label, rows } => html! {
            <div class="abe-create-field abe-create-field-wide">
                { render_field_label(label) }
                <div class="abe-create-table-wrap">
                    <table class="uk-table uk-table-small uk-table-divider abe-create-token-table">
                        <thead>
                            <tr>
                                <th>{ "Name" }</th>
                                <th>{ "Token" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for rows.iter().map(render_token_row) }
                        </tbody>
                    </table>
                </div>
            </div>
        },
        CreateFormField::GeneratedValue { label, value } => html! {
            <label class="abe-create-field abe-create-field-wide">
                { render_field_label(label) }
                <input class="uk-input abe-generated-input" type="text" value={value} readonly={true} />
            </label>
        },
    }
}

fn render_input_field(
    input_type: &'static str,
    label: &'static str,
    placeholder: &'static str,
    value: &'static str,
) -> Html {
    html! {
        <label class="abe-create-field">
            { render_field_label(label) }
            <input class="uk-input" type={input_type} placeholder={placeholder} value={value} />
        </label>
    }
}

fn render_token_row(row: &TokenTableRow) -> Html {
    html! {
        <tr>
            <td>{ row.name }</td>
            <td>
                <input class="uk-input" type="text" value={row.token} />
            </td>
        </tr>
    }
}

fn render_field_label(label: &'static str) -> Html {
    html! {
        <span class="uk-label uk-margin-small-bottom abe-create-label">
            { label }
        </span>
    }
}
