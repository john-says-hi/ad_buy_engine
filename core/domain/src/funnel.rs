use serde::{Deserialize, Serialize};

use crate::conditions::ConditionRule;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceType {
    OffersOnly,
    LandingPageAndOffers,
    Matrix,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedReference {
    pub id: String,
    pub weight: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunnelPath {
    pub id: String,
    pub weight: u32,
    pub landing_page_id: Option<String>,
    pub offers: Vec<WeightedReference>,
    pub children: Vec<FunnelPath>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunnelSequence {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub weight: u32,
    pub sequence_type: SequenceType,
    pub conditions: Vec<ConditionRule>,
    pub paths: Vec<FunnelPath>,
}

impl FunnelSequence {
    pub fn default_offer(offer_id: impl Into<String>) -> Self {
        Self {
            id: "default".to_string(),
            name: "Default".to_string(),
            active: true,
            weight: 100,
            sequence_type: SequenceType::OffersOnly,
            conditions: Vec::new(),
            paths: vec![FunnelPath {
                id: "path-1".to_string(),
                weight: 100,
                landing_page_id: None,
                offers: vec![WeightedReference {
                    id: offer_id.into(),
                    weight: 100,
                }],
                children: Vec::new(),
            }],
        }
    }
}
