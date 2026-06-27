use crate::config::ClickServerConfig;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Arc<ClickServerConfig>,
}

impl AppState {
    pub fn new(pool: SqlitePool, config: ClickServerConfig) -> Self {
        Self {
            pool,
            config: Arc::new(config),
        }
    }
}
