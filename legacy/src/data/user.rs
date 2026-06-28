use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlimUser {
    pub user_id: Uuid,
    pub email: String,
}
