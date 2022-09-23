use actix_multipart::Multipart;
use actix_web::{get, route, web, Error, HttpResponse, Responder, HttpRequest};
use std::path::Path;
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use serde_json::json;
use tokio::fs;

use crate::{
    db::Pool,
    schemas::root::{create_schema, Context, Schema},
    files,
};

/// REST endpoint for health check
#[route("/health", method = "GET", method = "POST")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!("I'm healthy"))
}

/// REST endpoint for file upload
#[route("/opt_file_upload", method = "POST")]
pub async fn opt_file_upload(payload: Multipart) -> HttpResponse {
    let id = uuid::Uuid::new_v4().to_string();
    files::save_file(payload, format!("./incoming{id}.csv")).await;
    fs::rename(Path::new(&format!("./incoming{id}.csv")),
               Path::new(&format!("./input/{id}.csv"))).await.unwrap();

    HttpResponse::Ok().json(json!(id))
}

/// Rest endpoint for processed opt file retrieval
#[route("/get_opt_file/{id}", method = "GET")]
pub async fn get_opt_file(path: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let id = path.into_inner();
    let s = format!("./output/{id}.csv");
    let file_path = Path::new(&s);

    println!("Id to retrieve: {}", id);

    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();

    file.into_response(&req)
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    pool: web::Data<Pool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        db_pool: pool.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;
    Ok(HttpResponse::Ok().json(res))
}

/// GraphiQL UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// Register services
pub fn register(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(create_schema()))
        .service(graphql)
        .service(graphql_playground)
        .service(health)
        .service(opt_file_upload)
        .service(get_opt_file);
}
