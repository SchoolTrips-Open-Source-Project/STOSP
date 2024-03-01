use actix_web::{web::{self}, Error};



use crate::{
    db,
    models::{
        self,
        auth::{AuthResponse, RegisterUserResponse},
    },
};

pub fn handle_auth(
    db: web::Data<db::database::Database>,
    req: web::Json<models::auth::AuthRequest>,
) -> Result<AuthResponse, Error> {
    let _ = req;
    let _ = db;
    return Ok(AuthResponse {
        auth_id : "".to_owned()
    });
}


pub fn handle_verify_auth(
    _db: web::Data<db::database::Database>,
    _req: web::Json<models::auth::VerifyAuthRequest>,
    _path: web::Path<String>
) -> Result<RegisterUserResponse, Error> {
        Ok(RegisterUserResponse {
                id: "".to_owned(),
                user_name: Some ("".to_owned()),
        mobile_number: Some ("".to_owned()),
        country_code: Some ("".to_owned())
    })
}