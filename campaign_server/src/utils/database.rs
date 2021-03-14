use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;
use crate::utils::errors::ApiError;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, ApiError> {
    pool.get().map_err(|_| ApiError::PoolError("Can't get connection".to_string()))
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    
    let postgres_db_host = env::var("POSTGRES_DB_HOST").expect("POSTGRES_DB_HOST must be set");
    
    let postgres_db =  env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    
    let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let postgres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        postgres_user, postgres_password, postgres_db_host, postgres_db
    );
    
    init_pool(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
