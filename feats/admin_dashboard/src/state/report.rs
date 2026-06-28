use ad_buy_engine_domain::EntityRow;

use crate::route::Route;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReportState {
    pub first_grouping: &'static str,
    pub second_grouping: &'static str,
    pub third_grouping: &'static str,
    pub date_range: &'static str,
    pub row_limit: &'static str,
    pub filter: &'static str,
    pub name_total: u32,
    pub visit_total: u32,
    pub unique_total: u32,
}

impl ReportState {
    pub const fn for_route(route: Route) -> Self {
        Self {
            first_grouping: route.render_route().label(),
            second_grouping: "Drill Down",
            third_grouping: "Drill Down",
            date_range: "Today",
            row_limit: "50",
            filter: "All",
            name_total: 0,
            visit_total: 0,
            unique_total: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReportTotals {
    pub name_total: i64,
    pub visit_total: i64,
    pub unique_total: i64,
}

impl ReportTotals {
    pub fn from_rows(rows: &[EntityRow]) -> Self {
        Self {
            name_total: i64::try_from(rows.len()).unwrap_or(i64::MAX),
            visit_total: rows.iter().map(|row| row.visits).sum(),
            unique_total: rows.iter().map(|row| row.unique_visits).sum(),
        }
    }
}
