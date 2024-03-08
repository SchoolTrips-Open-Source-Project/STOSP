
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::db::schema::school_trips::auth::dsl;
use crate::db::schema::school_trips::auth::{self};

use crate::{db::database::DBPool, storage::{models::auth::Auth, result}};

// Implementaiton of Auth table and related Queries
impl Auth {
    /// Implementation for inserting a record in auth table
    pub fn insert(self, conn: &DBPool) -> result::QueryResult<Auth>{
        let insert_query = diesel::insert_into(auth::table).values(&self);
        match insert_query.execute(&mut conn.get().unwrap()) {
            Ok(_) => return result::QueryResult::Success,
            Err(err) => return result::QueryResult::Failed(err.to_string()),
        };
    }

    /// Implementation for updating a record in auth table
    pub fn update(self, conn: &DBPool) -> result::QueryResult<Auth>{
        let update_query = diesel::update(auth::table).set(&self);
        match update_query.execute(&mut conn.get().unwrap()) {
            Ok(_) => return result::QueryResult::Record(self),
            Err(err) => return result::QueryResult::Failed(err.to_string()),
        };
    }

    /// Implementation to find all the records by primary key.
    pub fn get_all_by_key(key: String, conn: &DBPool) -> result::QueryResult<Auth>{
        let get_all_result = dsl::auth.filter(auth::id.eq(key.to_owned()));
        let results = get_all_result.load::<Auth,>(&mut conn.get().unwrap());
        match results {
            Ok (results) => result::QueryResult::Records(results),
            Err(err) => return result::QueryResult::Failed(err.to_string()),
        }
    }

    /// Implementation to find the record by mobile number.
    pub fn get_one_by_mobile(key: String, conn: &DBPool) -> result::QueryResult<Option<Auth>>{
        let get_all_result = dsl::auth.filter(auth::mobile_number.eq(key.to_owned()));
        let results = get_all_result.load::<Auth,>(&mut conn.get().unwrap());
        match results {
            Ok (results) => result::QueryResult::Record(results.into_iter().nth(0)),
            Err(err) => result::QueryResult::Failed(err.to_string()),
        }
    }

    /// Implementation to find the record by primary key.
    pub fn get_one_by_key(key: String, conn: &DBPool) -> result::QueryResult<Option<Auth>>{
        let get_all_result = dsl::auth.filter(auth::id.eq(key.to_owned()));
        let results = get_all_result.load::<Auth,>(&mut conn.get().unwrap());
        match results {
            Ok (results) => result::QueryResult::Record(results.into_iter().nth(0)),
            Err(err) => result::QueryResult::Failed(err.to_string()),
        }
    }
    /// Implementation to find the record by token.
    pub fn get_by_token(key: String, conn: &DBPool) -> result::QueryResult<Option<Auth>>{
        let get_all_result = dsl::auth.filter(auth::token.eq(key.to_owned()));
        let results = get_all_result.load::<Auth,>(&mut conn.get().unwrap());
        match results {
            Ok (results) => result::QueryResult::Record(results.into_iter().nth(0)),
            Err(err) => result::QueryResult::Failed(err.to_string()),
        }
    }
}