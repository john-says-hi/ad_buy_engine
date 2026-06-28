use serde::Deserialize;

use crate::storage::date_filter::VisitDateFilter;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct DateFilterQuery {
    pub start_at_millis: Option<i64>,
    pub end_at_millis: Option<i64>,
}

impl From<DateFilterQuery> for VisitDateFilter {
    fn from(query: DateFilterQuery) -> Self {
        Self::new(query.start_at_millis, query.end_at_millis)
    }
}
