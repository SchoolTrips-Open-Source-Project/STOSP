use crate::db;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[serde(rename_all = "snake_case")]
#[diesel(table_name = db::schema::school_trips::auth)]
pub struct Auth {
    pub id: String,
    pub mobile_number: String,
    pub country_code: String,
    pub created_at: NaiveDateTime,
    pub otp: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub mobile_number: String,
    pub country_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthResponse {
    pub auth_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyAuthRequest {
    pub otp: String
}

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

#[derive(Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[serde(rename_all = "snake_case")]
#[diesel(table_name = db::schema::school_trips::user)]
pub struct User {
    pub id: String,
    pub user_name: Option<String>,
    pub mobile_number: Option<String>,
    pub country_code: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_pass: Option<String>,
}

