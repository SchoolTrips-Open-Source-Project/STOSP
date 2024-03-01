use crate::db;
use crate::handlers::auth;
use crate::models;
use actix_web::{post, web, HttpRequest, HttpResponse};

#[post("/auth")]
pub async fn create_auth(
    state: web::Data<db::database::Database>,
    _req: HttpRequest,
    json_payload: web::Json<models::auth::AuthRequest>,
) -> HttpResponse {
    return match auth::handle_auth(state, json_payload) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    };
}

#[post("/auth/{auth_id}/verify")]
pub async fn verify_auth(
    state: web::Data<db::database::Database>,
    _req: HttpRequest,
    json_payload: web::Json<models::auth::VerifyAuthRequest>,
    path: web::Path<String>
) -> HttpResponse {
    return match auth::handle_verify_auth(state, json_payload, path) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    };
}