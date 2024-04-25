use geo::Contains;

use crate::{storage::models::geometry::ServiceabilityRequest, tools::polygon_utils::geo_json::get_available_shape_files, web, HttpResponse, ServerState};

pub fn origin_serviceability(
    state: web::Data<ServerState>,
    req: web::Json<ServiceabilityRequest>,
) -> HttpResponse {
    let areas = get_available_shape_files();
    let mut result = false;
    for area in areas {
        if (area.get_polygon().contains(&req.point)) {
            result = true; 
        }
    }
    return HttpResponse::Ok().json(result);
}
