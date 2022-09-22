use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub type Pool = deadpool_postgres::Pool;

// todo!("Add reading environment variables")
/// Create database pool to use in Juniper Context
pub fn get_db_pool(port: u16) -> Pool {
    // Initializing database connection config
    let mut cfg = Config::new();
    cfg.dbname = Some("postgres".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(port);
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("mysecretpassword".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
