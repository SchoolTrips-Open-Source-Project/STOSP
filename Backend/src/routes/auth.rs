use crate::ServerState;
use crate::handlers::auth;
use crate::storage::models;
use actix_web::{post, web, HttpRequest, HttpResponse};

/// Handler for auth Endpoint
#[post("/auth")]
pub async fn create_auth(
    state: web::Data<ServerState>,
    _req: HttpRequest,
    json_payload: web::Json<models::auth::AuthRequest>,
) -> HttpResponse {
    return auth::handle_auth(&state.data, json_payload);
}


/// Handler for auth/verify Endpoint
#[post("/auth/{auth_id}/verify")]
pub async fn verify_auth(
    state: web::Data<ServerState>,
    _req: HttpRequest,
    json_payload: web::Json<models::auth::VerifyAuthRequest>,
    path: web::Path<String>
) -> HttpResponse {
    return match auth::handle_verify_auth(state, json_payload, path) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    };
}