use actix_web::{get, route, web, Error, HttpResponse, Responder, HttpRequest};
use actix_web::web::Bytes;
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use serde_json::json;

use crate::{
    db::Pool,
    schemas::root::{create_schema, Context, Schema},
    pubsub,
};

/// REST endpoint for health check
#[route("/health", method = "GET", method = "POST")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!("I'm healthy"))
}

/// REST endpoint for file upload
#[route("/opt_file_upload", method = "POST")]
pub async fn opt_file_upload(bytes: Bytes) -> HttpResponse {
    pubsub::publish_opt_file(bytes).await;
    println!("I worked");
    HttpResponse::Ok().json(json!("I'm healthy"))
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
        .service(opt_file_upload);
}
