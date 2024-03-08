use chrono::{NaiveDateTime, TimeDelta, Utc};


pub fn get_exipry_from_minutes(min:i64) -> NaiveDateTime {
    let current_time = Utc::now().naive_utc();
    return  current_time.checked_add_signed(TimeDelta::minutes(min)).expect("Out of Bounds");
}