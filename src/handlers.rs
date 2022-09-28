use actix_multipart::Multipart;
use actix_web::{get, route, web, Error, HttpResponse, Responder, HttpRequest};
use std::path::{Path, PathBuf};
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use serde_json::json;
use tokio::fs;


use crate::{
    db::Pool,
    schemas::root::{create_schema, Context, Schema},
    files,
};
use crate::options_listener::{opt_file_healthy, run_api_calc};

/// REST endpoint for health check
#[route("/health", method = "GET", method = "POST")]
pub async fn health() -> HttpResponse {
    println!("Health check returned");
    HttpResponse::Ok().json(json!("I'm healthy"))
}

/// Rest endpoint for sample file
#[route("/opt_sample_file", method = "GET")]
pub async fn opt_sample_file(req: HttpRequest) -> HttpResponse {
    println!("Sending sample_options.csv");
    // Serve file
    let file = actix_files::NamedFile::open_async(Path::new("./assets/samples/opt_sample.csv")).await.unwrap();
    let res = file.set_content_disposition(
        ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![
                DispositionParam::Name(String::from("file")),
                DispositionParam::Filename(String::from("sample_options.csv")),
            ],
        }
    ).into_response(&req);
    res
}

/// REST endpoint for file upload
#[route("/opt_file_upload", method = "POST")]
pub async fn opt_file_upload(payload: Multipart) -> HttpResponse {
    // Generate uuid for file name
    let id = uuid::Uuid::new_v4().to_string();
    let file_path = format!("./input{id}.csv");

    files::save_file(payload, file_path.clone()).await;

    if !(opt_file_healthy(file_path.clone())) {
        fs::remove_file(format!("./input{id}.csv")).await.unwrap();
        println!("File unhealthy, removing file: input{id}.csv, bad request");
        return HttpResponse::BadRequest().finish();
    }

    println!("Upload complete: ./input{id}.csv");

    // Calculate options, saves output as filename {id}.csv
    run_api_calc(PathBuf::from(file_path), id.clone());
    println!("Calculation complete for {id}");

    // Remove input file
    fs::remove_file(format!("./input{id}.csv")).await.unwrap();

    // Returns id to query processed file
    HttpResponse::Ok().json(json!(id))
}

/// Rest endpoint for processed opt file retrieval
#[route("/get_opt_file/{id}", method = "GET")]
pub async fn get_opt_file(path: web::Path<String>, req: HttpRequest) -> HttpResponse {
    // id to query
    let id = path.into_inner();
    let s = format!("{id}.csv");
    let file_path = Path::new(&s);

    println!("File being retrieved: {}", id);

    // Serve file
    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();
    let res = file.set_content_disposition(
        ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![
                DispositionParam::Name(String::from("file")),
                DispositionParam::Filename(String::from("options_results.csv")),
            ],
        }
    ).into_response(&req);

    // Remove file
    println!("Removing file: {}", id);
    fs::remove_file(Path::new(file_path)).await.unwrap();
    res
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
        .service(get_opt_file)
        .service(opt_sample_file);
}
