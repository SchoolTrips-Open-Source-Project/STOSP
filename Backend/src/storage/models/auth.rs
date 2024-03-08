use crate::db;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;


/// Auth table sturct
#[derive(Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[serde(rename_all = "snake_case")]
#[diesel(table_name = db::schema::school_trips::auth)]
pub struct Auth {
    pub id: String,
    pub mobile_number: String,
    pub country_code: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub otp: String,
    pub updated_at: NaiveDateTime,
    pub token_expiry: NaiveDateTime
}
// Request and Response types..
#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub mobile_number: String,
    pub country_code: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub auth_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyAuthRequest {
    pub otp: String
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

