use crate::appstate::app_state::AppState;
use crate::appstate::lists::PrimeElement;
use crate::utils::routes::AppRoute;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct PrimeGroupingColumns {
    pub first_column: PrimeElement,
    pub second_column: PrimeElement,
    pub third_column: PrimeElement,
}

impl PrimeGroupingColumns {
    pub fn count_columns(&self) -> u8 {
        if self.second_column == PrimeElement::Nothing {
            1
        } else if self.third_column == PrimeElement::Nothing {
            2
        } else {
            3
        }
    }
}

impl Default for PrimeGroupingColumns {
    fn default() -> Self {
        PrimeGroupingColumns {
            first_column: PrimeElement::Campaigns,
            second_column: PrimeElement::default(),
            third_column: PrimeElement::default(),
        }
    }
}

impl PrimeGroupingColumns {
    pub fn get_first_grouping_column(&self) -> PrimeElement {
        self.first_column.clone()
    }

    pub fn get_second_grouping_column(&self) -> PrimeElement {
        self.second_column.clone()
    }

    pub fn get_third_grouping_column(&self) -> PrimeElement {
        self.third_column.clone()
    }

    pub fn set_first_column_group(&mut self, new: PrimeElement) {
        self.first_column = new
    }

    pub fn set_second_column_group(&mut self, new: PrimeElement) {
        self.second_column = new
    }

    pub fn set_third_column_group(&mut self, new: PrimeElement) {
        self.third_column = new
    }
}
