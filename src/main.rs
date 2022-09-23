use std::path::Path;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;

use crate::db::get_db_pool;


mod db;
mod schemas;
mod handlers;
mod files;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // if executable exists std::process::command start
    if Path::new("/app/options_listener").exists() {
        println!("Starting listener");
        std::process::Command::new("/app/options_listener").spawn().expect("Failed to run options_listener");
    }

    let pool = get_db_pool(5432);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(handlers::register)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
        .workers(2)
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
