use chrono::{NaiveDateTime, TimeDelta, Utc};

pub fn get_exipry_from_minutes(min: i64) -> NaiveDateTime {
    let current_time = Utc::now().naive_utc();
    return current_time
        .checked_add_signed(TimeDelta::minutes(min))
        .expect("Out of Bounds");
}

pub fn get_seconds_left(expiry_time: NaiveDateTime) -> i64 {
    let current_time = Utc::now().naive_utc();
    (expiry_time - current_time).num_seconds()
}

pub fn is_expired(expiry_time: NaiveDateTime) -> bool {
    get_seconds_left(expiry_time) <= 0
}
