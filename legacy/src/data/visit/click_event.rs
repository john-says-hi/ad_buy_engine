use crate::data::visit::click_map::ClickMap;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickEvent {
    pub timestamp: NaiveDateTime,
    pub is_suspicious: bool,
    pub element_clicked: ClickableElement,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClickableElement {
    PreLandingPage(Uuid),
    LandingPage(Uuid),
    Offer(Uuid),
}
