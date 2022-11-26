use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use std::env;

pub type Pool = deadpool_postgres::Pool;

/// Create database pool to use in Juniper Context
pub fn get_db_pool(port: u16) -> Pool {
    // Initializing database connection config
    let mut cfg = Config::new();
    cfg.dbname = Some(env::var("POSTGRES_DB").unwrap());
    cfg.host = Some("postgres".to_string());
    cfg.port = Some(port);
    cfg.user = Some(env::var("POSTGRES_USER").unwrap());
    cfg.password = Some(env::var("POSTGRES_PASSWORD").unwrap());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
