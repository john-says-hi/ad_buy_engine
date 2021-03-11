use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BillingInformation {
    stripe_profile: Uuid,
    history: Vec<String>,
}
