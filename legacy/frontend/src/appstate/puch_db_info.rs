use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct LocalDatabaseInfo {
    pub pouchdb_name: Uuid,
    pub max_items: usize,
    pub current_items: usize,
}
