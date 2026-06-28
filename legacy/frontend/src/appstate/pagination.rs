#[derive(Deserialize, Serialize, Clone)]
pub struct Pagination {
    pub total_pages: usize,
    pub current_page: usize,
}
