use crate::storage::models;
use crate::storage::models::auth::{Auth, VerifyAuthResponse};
use crate::storage::models::user::User;
use crate::storage::result::QueryResult;
use crate::ServerState;
use actix_web::web::{self};
use actix_web::HttpResponse;
use common::tools::contants::ONEWEEK;
use common::tools::session_utils::SessionToken;
use common::tools::utils::{get_current_time, get_exipry_from_minutes, is_expired};

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
                                            updated_at: get_current_time(),
                                            ..register_user
                                        };
                                        response = VerifyAuthResponse {
                                            session_token: SessionToken::new(
                                                upadated_user.id.clone(),
                                                get_exipry_from_minutes(ONEWEEK).timestamp(),
                                            )
                                            .encode(state.config.jwt_secret.clone()),
                                            user: upadated_user.to_owned()
                                        };
                                        upadated_user.update(&state.data.pool);
                                    }
                                    None => {
                                        let user = User {
                                            name: None,
                                            updated_at: get_current_time(),
                                            created_at: get_current_time(),
                                            id: uuid::Uuid::new_v4().to_string(),
                                            mobile_number: auth.mobile_number.to_owned(),
                                            role: auth.role.to_owned(),
                                        };
                                        response = VerifyAuthResponse {
                                            session_token: SessionToken::new(
                                                user.id.clone(),
                                                get_exipry_from_minutes(ONEWEEK).timestamp(),
                                            )
                                            .encode(state.config.jwt_secret.clone()),
                                            user: user.to_owned()
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
