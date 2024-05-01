use actix_web::{get, http::header::AUTHORIZATION, web, HttpRequest, HttpResponse};

use crate::{
    common::tools::{session_utils::SessionToken, utils::get_token_from_bearer},
    types::ServerState,
};

#[get("/verifySession")]
pub async fn verify_session(state: web::Data<ServerState>, req: HttpRequest) -> HttpResponse {
    let headers = req.headers();
    if headers.contains_key(AUTHORIZATION) {
        match headers.get(AUTHORIZATION) {
            Some(token) => {
                match SessionToken::decode(
                    get_token_from_bearer(token.to_str().unwrap().to_owned()),
                    state.config.jwt_secret.to_owned(),
                ) {
                    Some(_) => HttpResponse::Ok().body("Authorized").into(),
                    None => HttpResponse::Unauthorized().body("UnAuthorized").into(),
                }
            }
            None => HttpResponse::Unauthorized().into(),
        }
    } else {
        HttpResponse::Unauthorized().into()
    }
}
