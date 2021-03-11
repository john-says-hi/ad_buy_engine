use crate::appstate::lists::PrimeElement;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SelectedElement {
    pub element_type: PrimeElement,
    pub index: usize,
}
