use actix_web::web::{self};

/// Adding all the endpoints
pub fn add_all_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(super::auth::create_auth)
            .service(super::auth::verify_auth)
            .service(super::session::verify_session)
            .service(super::serviceability::serviceability),
    );
}
