use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use crate::db::get_db_pool;

mod db;
mod schemas;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_db_pool(5432);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(handlers::register)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
        .workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
