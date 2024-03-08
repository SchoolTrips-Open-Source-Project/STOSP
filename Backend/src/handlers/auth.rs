use actix_web::HttpResponse;
use actix_web::{
    http::Error,
    web::{self},
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use db::schema::school_trips::auth::dsl;
use db::schema::school_trips::auth::table;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use crate::storage::result::QueryResult;

use crate::storage::models;
use crate::storage::models::auth::{Auth, AuthResponse, RegisterUserResponse};
use crate::utils;
use crate::{
    db::{self, database::Database},
    ServerState,
};

pub fn handle_auth(db: &Database, req: web::Json<models::auth::AuthRequest>) -> HttpResponse {
    let mobile_hash = STANDARD.encode(req.mobile_number.clone());
    let id = uuid::Uuid::new_v4().to_string();
    let auth_filter =  Auth::get_one_by_mobile(mobile_hash.clone(), &db.pool);
    match auth_filter {
        crate::storage::result::QueryResult::Record(record) => {
            match record {
                Some(existing_user) => {
                    let updated_auth = Auth {
                        token: id.clone(),
                        updated_at: Utc::now().naive_utc(),
                        token_expiry: utils::utils::get_exipry_from_minutes(3),
                        ..existing_user
                    };
                    let _get_result = diesel::update(
                        table.filter(dsl::mobile_number.eq(updated_auth.mobile_number.clone())),
                    )
                    .set(updated_auth)
                    .get_result::<Auth>(&mut db.pool.get().unwrap());
                    let res = AuthResponse { auth_id: id };
                    return HttpResponse::Ok().json(res);
                },
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
                        QueryResult::Success => {let res = AuthResponse { auth_id: id.clone() };
                        return HttpResponse::Ok().json(res);
                        },
                        QueryResult::Failed(err) => HttpResponse::InternalServerError().body(err.to_string()),
                        _ => HttpResponse::InternalServerError().body("INTERNAL_ERROR"),
                    }
                },
        }
    }
    QueryResult::Failed(err) => HttpResponse::InternalServerError().body(err.to_string()),
    _ => HttpResponse::InternalServerError().body("INTERNAL_ERROR"),
}
}

pub fn handle_verify_auth(
    _db: web::Data<ServerState>,
    _req: web::Json<models::auth::VerifyAuthRequest>,
    _path: web::Path<String>,
) -> Result<RegisterUserResponse, Error> {
        Ok(RegisterUserResponse {
                id: "".to_owned(),
                user_name: Some ("".to_owned()),
        mobile_number: Some ("".to_owned()),
        country_code: Some ("".to_owned())
    })
}