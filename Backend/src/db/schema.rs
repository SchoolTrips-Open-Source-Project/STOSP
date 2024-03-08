// @generated automatically by Diesel CLI.

pub mod school_trips {
    diesel::table! {
        school_trips.auth (id) {
            #[max_length = 255]
            id -> Varchar,
            mobile_number -> Varchar,
            #[max_length = 10]
            country_code -> Varchar,
            token -> Varchar,
            created_at -> Timestamptz,
            #[max_length = 10]
            otp -> Varchar,
            updated_at -> Timestamptz,
            token_expiry -> Timestamptz,
        }
    }
}
