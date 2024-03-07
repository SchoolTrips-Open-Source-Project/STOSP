extern crate diesel;
extern crate diesel_migrations;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use db::database::Database;
use serde;
mod db;
mod routes;
mod models;
mod handlers;
mod transformers;


#[derive(serde::Deserialize, serde::Serialize)]
pub struct Response {
    pub message: String,
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}


pub struct ServerState {
    data : Database,
    request_id: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let my_chat = ServerState::new();
    let app_data: web::Data<_> = web::Data::new(my_chat);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(routes::routes::add_all_routes)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8844))?
        .run()
        .await
}

impl ServerState {
    pub fn new() -> Self {
        ServerState { data: db::database::Database::new(), request_id: "".to_owned() }
    }

}
