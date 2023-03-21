use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;

use crate::db::get_db_pool;

mod db;
mod schemas;
mod handlers;
mod files;
mod options_listener;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize database connection pool
    let pool = get_db_pool();

    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Server starting on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(handlers::register)
            .wrap(Cors::default()
            //     .allowed_origin("https://alexander.hyll.nu")
                .allowed_origin("http://localhost:3000")
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header())
            .wrap(Logger::default())
    })
        .workers(10)
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
