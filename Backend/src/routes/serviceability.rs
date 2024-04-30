use actix_web::{get, web, HttpResponse};

use crate::{
    handlers::geometry::serviceability::origin_serviceability,
    storage::models,
    types::ServerState,
};

#[get("origin/serviceability")]
pub async fn serviceability(
    state: web::Data<ServerState>,
    req: web::Json<models::geometry::ServiceabilityRequest>,
) -> HttpResponse {
    return origin_serviceability(state, req);
}
