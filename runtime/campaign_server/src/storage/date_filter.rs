use sqlx::sqlite::SqliteArguments;
use sqlx::{Sqlite, query::Query, query::QueryScalar};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VisitDateFilter {
    pub start_at_millis: Option<i64>,
    pub end_at_millis: Option<i64>,
}

impl VisitDateFilter {
    pub const fn new(start_at_millis: Option<i64>, end_at_millis: Option<i64>) -> Self {
        Self {
            start_at_millis,
            end_at_millis,
        }
    }
}

pub fn bind_visit_date_filter<'q>(
    query: Query<'q, Sqlite, SqliteArguments<'q>>,
    date_filter: VisitDateFilter,
) -> Query<'q, Sqlite, SqliteArguments<'q>> {
    query
        .bind(date_filter.start_at_millis)
        .bind(date_filter.start_at_millis)
        .bind(date_filter.end_at_millis)
        .bind(date_filter.end_at_millis)
}

pub fn bind_visit_date_filter_scalar<'q, O>(
    query: QueryScalar<'q, Sqlite, O, SqliteArguments<'q>>,
    date_filter: VisitDateFilter,
) -> QueryScalar<'q, Sqlite, O, SqliteArguments<'q>>
where
    O: Send + Unpin,
{
    query
        .bind(date_filter.start_at_millis)
        .bind(date_filter.start_at_millis)
        .bind(date_filter.end_at_millis)
        .bind(date_filter.end_at_millis)
}
