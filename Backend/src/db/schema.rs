// @generated automatically by Diesel CLI.

pub mod school_trips {
    pub mod sql_types {
        #[derive(diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "role_type"))]
        pub struct RoleType;
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::RoleType;

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
            role -> RoleType,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::RoleType;

        school_trips.users (id) {
            #[max_length = 255]
            name -> Nullable<Varchar>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            session_token -> Varchar,
            mobile_number -> Varchar,
            role -> RoleType,
            id -> Varchar,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(auth, users,);
}
