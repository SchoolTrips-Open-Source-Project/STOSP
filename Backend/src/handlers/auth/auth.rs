use crate::storage::models;
use crate::storage::models::auth::{Auth, AuthResponse, VerifyAuthResponse};
use crate::storage::models::user::User;
use crate::storage::result::QueryResult;
use crate::utils;
use crate::utils::utils::is_expired;
use crate::ServerState;
use actix_web::web::{self};
use actix_web::HttpResponse;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;

pub fn handle_auth(
    state: web::Data<ServerState>,
    req: web::Json<models::auth::AuthRequest>,
) -> HttpResponse {
    let mobile_hash = STANDARD.encode(req.mobile_number.clone());
    let id = uuid::Uuid::new_v4().to_string();
    let auth_filter = Auth::get_by_mobile(mobile_hash.clone(), &state.data.pool);
    match auth_filter {
        QueryResult::Record(record) => match record {
            Some(existing_user) => {
                let updated_auth = Auth {
                    token: id.clone(),
                    updated_at: Utc::now().naive_utc(),
                    token_expiry: utils::utils::get_exipry_from_minutes(state.config.auth_timeout),
                    ..existing_user
                };
                updated_auth.update(&state.data.pool);
                let res = AuthResponse { auth_id: id };
                return HttpResponse::Ok().json(res);
            }
            None => {
                let auth = Auth {
                    id: uuid::Uuid::new_v4().to_string().to_owned(),
                    token: id.clone(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    mobile_number: mobile_hash.clone(),
                    country_code: req.country_code.clone(),
                    otp: state.config.default_otp.clone(),
                    role: req.role.to_owned(),
                    token_expiry: utils::utils::get_exipry_from_minutes(state.config.auth_timeout),
                };
                match auth.insert(&state.data.pool) {
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
    state: web::Data<ServerState>,
    req: web::Json<models::auth::VerifyAuthRequest>,
    token: String,
) -> HttpResponse {
    match Auth::get_by_token(token.to_owned(), &state.data.pool) {
        QueryResult::Record(existing_token) => match existing_token {
            Some(auth) => {
                let expiry_time = auth.token_expiry;
                if is_expired(expiry_time) {
                    HttpResponse::BadRequest().body("Token Expired - ".to_string() + &token)
                } else {
                    if auth.otp == req.otp {
                        let existing_user =
                            User::get_by_mobile(&auth.mobile_number, &state.data.pool);
                        match existing_user {
                            QueryResult::Record(user) => {
                                let response: VerifyAuthResponse;
                                match user {
                                    Some(register_user) => {
                                        let upadated_user = User {
                                            updated_at: Utc::now().naive_utc(),
                                            ..register_user
                                        };
                                        response = VerifyAuthResponse {
                                            name: upadated_user.name.clone(),
                                            updated_at: upadated_user.updated_at.to_string(),
                                            created_at: upadated_user.created_at.to_string(),
                                            id: upadated_user.id.clone(),
                                            role: upadated_user.role.clone(),
                                        };
                                        upadated_user.update(&state.data.pool);
                                    }
                                    None => {
                                        let user = User {
                                            name: None,
                                            updated_at: Utc::now().naive_utc(),
                                            created_at: Utc::now().naive_utc(),
                                            session_token: uuid::Uuid::new_v4().to_string(),
                                            id: uuid::Uuid::new_v4().to_string(),
                                            mobile_number: auth.mobile_number.to_owned(),
                                            role: auth.role.to_owned(),
                                        };
                                        response = VerifyAuthResponse {
                                            name: user.name.clone(),
                                            updated_at: user.updated_at.to_string(),
                                            created_at: user.created_at.to_string(),
                                            id: user.id.clone(),
                                            role: user.role.clone(),
                                        };
                                        user.insert(&state.data.pool);
                                    }
                                }
                                HttpResponse::Ok().json(response)
                            }
                            _ => HttpResponse::BadRequest().finish(),
                        }
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
