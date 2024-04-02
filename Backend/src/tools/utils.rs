use chrono::{NaiveDateTime, TimeDelta, Utc};

pub fn get_exipry_from_minutes(min: i64) -> NaiveDateTime {
    let current_time = get_current_time();
    return current_time
        .checked_add_signed(TimeDelta::minutes(min))
        .expect("Out of Bounds");
}

pub fn get_seconds_left(expiry_time: NaiveDateTime) -> i64 {
    let current_time = get_current_time();
    (expiry_time - current_time).num_seconds()
}

pub fn is_expired(expiry_time: NaiveDateTime) -> bool {
    get_seconds_left(expiry_time) <= 0
}

pub fn get_current_time() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn get_token_from_bearer(token: String) -> String{
    if token.starts_with("Bearer") {
        match token.strip_prefix("Bearer") {
            Some(token) => token.trim().to_owned(),
            None => token,
        }
    } else {
        token
    }
}