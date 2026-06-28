use ad_buy_engine_domain::EntityRow;
use yew::prelude::*;

use crate::state::report::{ReportState, ReportTotals};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ReportTableProps {
    pub report: ReportState,
    pub rows: Vec<EntityRow>,
    pub loading: bool,
    pub actions_enabled: bool,
    pub on_edit: Callback<String>,
    pub on_archive: Callback<String>,
}

#[function_component(ReportTable)]
pub fn report_table(props: &ReportTableProps) -> Html {
    let totals = ReportTotals::from_rows(&props.rows);
    html! {
        <div class="abe-table-wrap">
            <table class="uk-table uk-table-divider uk-table-striped uk-table-hover uk-table-small abe-table">
                <thead>
                    <tr>
                        <th class="uk-table-shrink" uk-tooltip="title: Select All">
                            <input class="uk-checkbox uk-disabled uk-margin-left" type="checkbox" disabled={true} />
                        </th>
                        <th class="uk-table-shrink">{ "Name" }</th>
                        <th class="uk-table-shrink">{ "Visits" }</th>
                        <th class="uk-table-shrink">{ "Unique" }</th>
                        <th class="uk-table-shrink">{ "Actions" }</th>
                    </tr>
                </thead>
                <tfoot class="uk-margin-top-large">
                    <tr>
                        <td colspan="2">{ "Totals:" }</td>
                        <td>{ totals.visit_total }</td>
                        <td>{ totals.unique_total }</td>
                        <td></td>
                    </tr>
                </tfoot>
                <tbody>
                    {
                        if props.loading {
                            html! {
                                <tr><td colspan="6">{ "Loading..." }</td></tr>
                            }
                        } else if props.rows.is_empty() {
                            html! {
                                <tr><td colspan="6">{ "No rows yet" }</td></tr>
                            }
                        } else {
                            html! {
                                <>
                                    { for props.rows.iter().map(|row| render_row(row, props)) }
                                </>
                            }
                        }
                    }
                </tbody>
            </table>
        </div>
    }
}

fn render_row(row: &EntityRow, props: &ReportTableProps) -> Html {
    let action_cell = if props.actions_enabled {
        let edit_id = row.id.clone();
        let archive_id = row.id.clone();
        let on_edit = props.on_edit.clone();
        let on_archive = props.on_archive.clone();
        let edit = Callback::from(move |_| on_edit.emit(edit_id.clone()));
        let archive = Callback::from(move |_| on_archive.emit(archive_id.clone()));
        html! {
            <td class="abe-row-actions">
                <button class="uk-button uk-button-default uk-button-small" type="button" onclick={edit}>{ "Edit" }</button>
                <button class="uk-button uk-button-danger uk-button-small" type="button" onclick={archive}>{ "Archive" }</button>
            </td>
        }
    } else {
        html! { <td></td> }
    };

    html! {
        <tr>
            <td><input class="uk-checkbox" type="checkbox" /></td>
            <td>
                <div class="abe-row-name">{ row.name.clone() }</div>
                <div class="abe-row-detail">{ row.detail.clone() }</div>
                {
                    row.tracking_url.as_ref().map(|url| html! {
                        <a class="abe-row-link" href={url.clone()} target="_blank" rel="noreferrer">{ url.clone() }</a>
                    }).unwrap_or_default()
                }
            </td>
            <td>{ row.visits }</td>
            <td>{ row.unique_visits }</td>
            { action_cell }
        </tr>
    }
}
