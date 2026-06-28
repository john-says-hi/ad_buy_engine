use ad_buy_engine_domain::{EntityRow, ReportDimensionKey};

use crate::route::Route;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReportState {
    pub first_grouping: ReportDimensionKey,
    pub second_grouping: &'static str,
    pub third_grouping: &'static str,
    pub date_range: ReportDateRange,
    pub row_limit: &'static str,
    pub filter: &'static str,
    pub name_total: u32,
    pub visit_total: u32,
    pub unique_total: u32,
}

impl ReportState {
    pub fn for_route(_route: Route, first_grouping: ReportDimensionKey) -> Self {
        Self {
            first_grouping,
            second_grouping: "Drill Down",
            third_grouping: "Drill Down",
            date_range: ReportDateRange::Today,
            row_limit: "50",
            filter: "All",
            name_total: 0,
            visit_total: 0,
            unique_total: 0,
        }
    }
}

pub fn default_grouping_for_route(route: Route) -> ReportDimensionKey {
    route
        .default_report_dimension()
        .unwrap_or(ReportDimensionKey::Campaigns)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportDateRange {
    Today,
    Yesterday,
    Last3Days,
    Last7Days,
    Last14Days,
    Last30Days,
    Last6Months,
    AllTime,
}

pub const DATE_RANGE_OPTIONS: &[ReportDateRange] = &[
    ReportDateRange::Today,
    ReportDateRange::Yesterday,
    ReportDateRange::Last3Days,
    ReportDateRange::Last7Days,
    ReportDateRange::Last14Days,
    ReportDateRange::Last30Days,
    ReportDateRange::Last6Months,
    ReportDateRange::AllTime,
];

impl ReportDateRange {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Today => "Today",
            Self::Yesterday => "Yesterday",
            Self::Last3Days => "Last 3 Days",
            Self::Last7Days => "Last 7 Days",
            Self::Last14Days => "Last 14 Days",
            Self::Last30Days => "Last 30 Days",
            Self::Last6Months => "Last 6 Months",
            Self::AllTime => "All of Time",
        }
    }

    pub const fn storage_key(self) -> &'static str {
        match self {
            Self::Today => "today",
            Self::Yesterday => "yesterday",
            Self::Last3Days => "last_3_days",
            Self::Last7Days => "last_7_days",
            Self::Last14Days => "last_14_days",
            Self::Last30Days => "last_30_days",
            Self::Last6Months => "last_6_months",
            Self::AllTime => "all_time",
        }
    }

    pub fn from_storage_key(value: &str) -> Option<Self> {
        match value {
            "today" => Some(Self::Today),
            "yesterday" => Some(Self::Yesterday),
            "last_3_days" => Some(Self::Last3Days),
            "last_7_days" => Some(Self::Last7Days),
            "last_14_days" => Some(Self::Last14Days),
            "last_30_days" => Some(Self::Last30Days),
            "last_6_months" => Some(Self::Last6Months),
            "all_time" => Some(Self::AllTime),
            _ => None,
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

pub fn filter_rows_by_search(rows: &[EntityRow], search_query: &str) -> Vec<EntityRow> {
    let normalized_query = search_query.trim().to_lowercase();
    if normalized_query.is_empty() {
        return rows.to_vec();
    }

    rows.iter()
        .filter(|row| row_matches_search(row, &normalized_query))
        .cloned()
        .collect()
}

fn row_matches_search(row: &EntityRow, normalized_query: &str) -> bool {
    row.id.to_lowercase().contains(normalized_query)
        || row.name.to_lowercase().contains(normalized_query)
        || row.detail.to_lowercase().contains(normalized_query)
        || row
            .tracking_url
            .as_deref()
            .unwrap_or_default()
            .to_lowercase()
            .contains(normalized_query)
}
