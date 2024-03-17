use actix_web::web;

/// Adding all the endpoints
pub fn add_all_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(super::auth::create_auth)
            // .service(super::auth::register_user)
            .service(super::auth::verify_auth),
    );
}
