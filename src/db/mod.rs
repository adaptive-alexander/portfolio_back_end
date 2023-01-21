use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use std::env;

pub type Pool = deadpool_postgres::Pool;

/// Create database pool to use in Juniper Context
pub fn get_db_pool() -> Pool {
    // Initializing database connection config
    let mut cfg = Config::new();
    cfg.dbname = Some(env::var("POSTGRES_DB").unwrap());
    cfg.host = Some(env::var("POSTGRES_HOST").unwrap());
    cfg.port = Some(env::var("POSTGRES_PORT").unwrap().parse().unwrap());
    cfg.user = Some(env::var("POSTGRES_USER").unwrap());
    cfg.password = Some(env::var("POSTGRES_PASSWORD").unwrap());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    println!("Database connection established");
    pool
}
