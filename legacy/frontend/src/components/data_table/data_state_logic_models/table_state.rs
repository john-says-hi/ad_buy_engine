use crate::appstate::lists::PrimeElement;
use std::rc::Rc;
use uuid::Uuid;

pub mod footer;
pub mod header;
pub mod row;

pub struct RelationalEvents {
    relational_events: Rc<String>,
}

pub struct TableData {
    report_type: PrimeElement,
}
