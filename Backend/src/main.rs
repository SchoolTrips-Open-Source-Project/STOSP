extern crate diesel;
extern crate diesel_migrations;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde;
use types::ServerState;
mod db;
mod handlers;
mod routes;
mod storage;
mod tools;
mod transformers;
mod types;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Response {
    pub message: String,
}
/// 404 errors for unknown endpoints
async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

/// actix's main function which starts the server and adds the listners.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_state = ServerState::new(); // creating the server state
    let app_data: web::Data<_> = web::Data::new(server_state);
    env_logger::init();
    HttpServer::new(
        move || {
            App::new()
                .app_data(app_data.clone())
                .configure(routes::routes::add_all_routes) // adding all the endpoints
                .default_service(web::route().to(not_found)) // sending 404 is route is not defined.
                .wrap(actix_web::middleware::Logger::default())
        }, // middleware to log the API calls. TODO need to imporve.
    )
    .bind(("127.0.0.1", 8844))? // Server Port and Host
    .run()
    .await
}
