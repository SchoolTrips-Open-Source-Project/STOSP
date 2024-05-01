use crate::db::{self};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::user::{Role, User};

/// Auth table sturct
#[derive(AsChangeset, Queryable, Insertable)]
#[diesel(table_name = db::schema::school_trips_driver::auth)]
pub struct Auth {
    pub id: String,
    pub mobile_number: String,
    pub country_code: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub otp: String,
    pub updated_at: NaiveDateTime,
    pub token_expiry: NaiveDateTime,
    pub role: Role,
}
// Request and Response types..
#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub mobile_number: String,
    pub country_code: String,
    pub role: Role,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub auth_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyAuthRequest {
    pub otp: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyAuthResponse {
    pub session_token: String,
    pub user: User,
}
// TODO Add User creation
#[derive(Deserialize, Serialize)]
pub struct RegisterUserRequest {
    pub user_name: String,
    pub user_pass: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterUserResponse {
    pub id: String,
    pub user_name: Option<String>,
    pub mobile_number: Option<String>,
    pub country_code: Option<String>,
}
