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
