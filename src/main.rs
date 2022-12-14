use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;

use crate::db::get_db_pool;

mod db;
mod schemas;
mod handlers;
mod files;
mod options_listener;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = get_db_pool();

    println!("Server starting on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(handlers::register)
            // .wrap(Cors::default()
                // .allowed_origin("https://alexander.hyll.nu")
                // .allowed_methods(vec!["GET", "POST"])
                // .allow_any_header())
            .wrap(Logger::default())
    })
        .workers(2)
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
