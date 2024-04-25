use actix_web::{get, http::header::AUTHORIZATION, web, HttpRequest, HttpResponse};

use crate::{
    handlers::geometry::serviceability::origin_serviceability, storage::models, tools::{session_utils::SessionToken, utils::get_token_from_bearer}, types::ServerState
};

#[get("origin/serviceability")]
pub async fn serviceability(state: web::Data<ServerState>, req: web::Json<models::geometry::ServiceabilityRequest>,) -> HttpResponse {
    return origin_serviceability(state,req);
}
