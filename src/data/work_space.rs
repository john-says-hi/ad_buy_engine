use crate::AError;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Clearance {
    Everyone,
    WorkSpace(WorkSpace),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkSpace {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AdditionalUser {
    pub user_id: Uuid,
    pub email: String,
    pub work_space_id: Uuid,
    pub role: AdditionalUserRole,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AdditionalUserRole {
    Worker,
    Admin,
}
