// @generated automatically by Diesel CLI.

pub mod school_trips {
    diesel::table! {
        school_trips.auth (id) {
            id -> Varchar,
            mobile_number -> Varchar,
            country_code -> Varchar,
            created_at -> Timestamp,
            otp -> Varchar,
            updated_at -> Timestamp,
        }
    }

    diesel::table! {
        school_trips.user (id) {
            id -> Varchar,
            user_name -> Nullable<Varchar>,
            mobile_number -> Nullable<Varchar>,
            country_code -> Nullable<Varchar>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
            user_pass -> Nullable<Varchar>,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(
        auth,
        user,
    );
}
