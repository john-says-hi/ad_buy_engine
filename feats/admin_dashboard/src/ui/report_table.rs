use yew::prelude::*;

use crate::state::report::ReportState;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Properties)]
pub struct ReportTableProps {
    pub report: ReportState,
}

#[function_component(ReportTable)]
pub fn report_table(props: &ReportTableProps) -> Html {
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
                    </tr>
                </thead>
                <tfoot class="uk-margin-top-large">
                    <tr>
                        <td>{ "Totals:" }</td>
                        <td>{ props.report.name_total }</td>
                        <td>{ props.report.visit_total }</td>
                        <td>{ props.report.unique_total }</td>
                    </tr>
                </tfoot>
                <tbody></tbody>
            </table>
        </div>
    }
}
