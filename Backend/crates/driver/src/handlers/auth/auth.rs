use crate::storage::models;
use crate::storage::models::auth::{Auth, AuthResponse};
use crate::storage::result::QueryResult;
// use crate::tools::utils::get_current_time;
use crate::ServerState;
use actix_web::web::{self};
use actix_web::HttpResponse;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use common::tools::utils::{get_current_time, get_exipry_from_minutes};

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
                    updated_at: get_current_time(),
                    token_expiry: get_exipry_from_minutes(state.config.auth_timeout),
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
                    created_at: get_current_time(),
                    updated_at: get_current_time(),
                    mobile_number: mobile_hash.clone(),
                    country_code: req.country_code.clone(),
                    otp: state.config.default_otp.clone(),
                    role: req.role.to_owned(),
                    token_expiry: get_exipry_from_minutes(state.config.auth_timeout),
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
