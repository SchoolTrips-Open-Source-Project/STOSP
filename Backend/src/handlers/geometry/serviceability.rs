use crate::{
    storage::models::geometry::ServiceabilityRequest,
    tools::polygon_utils::geo_json::get_available_shape_files, web, HttpResponse, ServerState,
};
use geo::{coord, Intersects};

pub fn origin_serviceability(
    _: web::Data<ServerState>,
    req: web::Json<ServiceabilityRequest>,
) -> HttpResponse {
    let areas = get_available_shape_files();
    for area in areas {
        return HttpResponse::Ok().json(
            area.get_polygon()
                .intersects(&coord! {x: req.point.lon, y: req.point.lat}),
        );
    }
    return HttpResponse::Ok().json(false);
}
