use crate::storage::models;
use crate::storage::models::auth::{Auth, AuthResponse};
use crate::storage::result::QueryResult;
use crate::utils;
use crate::utils::utils::is_expired;
use crate::{db::database::Database, ServerState};
use actix_web::web::{self};
use actix_web::HttpResponse;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, log};

pub fn handle_auth(db: &Database, req: web::Json<models::auth::AuthRequest>) -> HttpResponse {
    let mobile_hash = STANDARD.encode(req.mobile_number.clone());
    let id = uuid::Uuid::new_v4().to_string();
    let auth_filter = Auth::get_one_by_mobile(mobile_hash.clone(), &db.pool);
    match auth_filter {
        QueryResult::Record(record) => match record {
            Some(existing_user) => {
                let updated_auth = Auth {
                    token: id.clone(),
                    updated_at: Utc::now().naive_utc(),
                    token_expiry: utils::utils::get_exipry_from_minutes(3),
                    ..existing_user
                };
                updated_auth.update(&db.pool);
                let res = AuthResponse { auth_id: id };
                return HttpResponse::Ok().json(res);
            }
            None => {
                dotenv().ok();
                let default_otp = std::env::var("default_otp");
                let auth = Auth {
                    id: uuid::Uuid::new_v4().to_string().to_owned(),
                    token: id.clone(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    mobile_number: mobile_hash.clone(),
                    country_code: req.country_code.clone(),
                    otp: default_otp.unwrap_or("7891".to_owned()).to_owned(),
                    token_expiry: utils::utils::get_exipry_from_minutes(3),
                };
                match auth.insert(&db.pool) {
                    QueryResult::Success => {
                        let res = AuthResponse {
                            auth_id: id.clone(),
                        };
                        return HttpResponse::Ok().json(res);
                    }
                    QueryResult::Failed(err) => {
                        HttpResponse::InternalServerError().body(err.to_string())
                    }
                    _ => HttpResponse::InternalServerError().body("INTERNAL_ERROR"),
                }
            }
        },
        QueryResult::Failed(err) => HttpResponse::InternalServerError().body(err.to_string()),
        _ => HttpResponse::InternalServerError().body("INTERNAL_ERROR"),
    }
}

pub fn handle_verify_auth(
    db: web::Data<ServerState>,
    req: web::Json<models::auth::VerifyAuthRequest>,
    token: String,
) -> HttpResponse {
    match Auth::get_by_token(token.to_owned(), &db.data.pool) {
        QueryResult::Record(existing_token) => match existing_token {
            Some(auth) => {
                let expiry_time = auth.token_expiry;
                if is_expired(expiry_time) {
                    HttpResponse::BadRequest().body("Token Expired - ".to_string() + &token)
                } else {
                    if auth.otp == req.otp {
                        HttpResponse::Ok().body("Verified Successfully")
                    } else {
                        HttpResponse::BadRequest()
                            .body("Wrong OTP - ".to_string() + &req.otp.to_owned())
                    }
                }
            }
            None => HttpResponse::BadRequest().body("Invalid Token - ".to_string() + &token),
        },
        _ => HttpResponse::InternalServerError().body("INTERNAL_ERROR"),
    }
}
