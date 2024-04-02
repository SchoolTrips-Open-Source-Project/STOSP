use crate::{
    handlers::auth::{handle_auth, handle_verify_auth},
    storage::models,
    ServerState,
};
use actix_web::{post, web, HttpResponse};

/// Handler for auth Endpoint
#[post("/auth")]
pub async fn create_auth(
    state: web::Data<ServerState>,
    json_payload: web::Json<models::auth::AuthRequest>,
) -> HttpResponse {
    handle_auth(state, json_payload)
}

/// Handler for auth/verify Endpoint
#[post("/auth/{auth_id}/verify")]
pub async fn verify_auth(
    state: web::Data<ServerState>,
    json_payload: web::Json<models::auth::VerifyAuthRequest>,
    path: web::Path<String>,
) -> HttpResponse {
    handle_verify_auth(state, json_payload, path.into_inner())
}
