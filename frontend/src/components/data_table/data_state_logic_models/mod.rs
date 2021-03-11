use crate::appstate::lists::PrimeElement;

pub mod app_state_logic;
pub mod lists;
pub mod table_state;

pub struct ElementalVisitRow {
    pub element_type: PrimeElement,
    pub visit_total: u64,
    pub unique_visits: u64,
    pub click_total: u64,
    pub conversions: u32,
    pub revenue: f64,
    pub cost: f64,
    pub profit: f64,
    pub cost_per_view: f32,
    pub click_through_rate: f32,
    pub conversion_rate: f32,
    pub return_on_investment: f32,
    pub earnings_per_view: f32,
    pub earnings_per_click: f32,
    pub average_payout: f32,
}
