use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CampaignLaw {
    pub name: String,
    pub relates_to: LawRelatesTo,
    pub include_in_revenue_column: bool,
    pub send_postback_to_traffic_source: bool,
    pub include_in_cost_column: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LawRelatesTo {
    Campaign(Vec<Uuid>),
    Offer(Vec<Uuid>),
    Dimension,
}
