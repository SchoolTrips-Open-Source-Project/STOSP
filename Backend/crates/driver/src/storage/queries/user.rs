use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;

use crate::db::schema::school_trips_driver::users::{self};

use crate::{
    db::database::DBPool,
    storage::{models::user::User, result},
};

// Implementaiton of User table and related Queries
impl User {
    /// Implementation for inserting a record in users table
    pub fn insert(self, conn: &DBPool) -> result::QueryResult<User> {
        let insert_query = diesel::insert_into(users::table).values(&self);
        match insert_query.execute(&mut conn.get().unwrap()) {
            Ok(_) => return result::QueryResult::Success,
            Err(err) => return result::QueryResult::Failed(err.to_string()),
        };
    }

    /// Implementation for updating a record in users table
    pub fn update(self, conn: &DBPool) -> result::QueryResult<User> {
        let update_query = diesel::update(users::table).set(&self);
        match update_query.execute(&mut conn.get().unwrap()) {
            Ok(_) => return result::QueryResult::Record(self),
            Err(err) => return result::QueryResult::Failed(err.to_string()),
        };
    }

    /// Implementation to find all the records by primary key.
    pub fn get_all_by_key(key: String, conn: &DBPool) -> result::QueryResult<User> {
        let results = users::table
            .select(User::as_select())
            .filter(users::id.eq(key))
            .load(&mut conn.get().unwrap());
        match results {
            Ok(results) => result::QueryResult::Records(results),
            Err(err) => return result::QueryResult::Failed(err.to_string()),
        }
    }

    /// Implementation to find the record by mobile number.
    pub fn get_by_mobile(key: &String, conn: &DBPool) -> result::QueryResult<Option<User>> {
        let results = users::table
            .select(User::as_select())
            .filter(users::mobile_number.eq(key))
            .load(&mut conn.get().unwrap());
        match results {
            Ok(results) => result::QueryResult::Record(results.into_iter().nth(0)),
            Err(err) => result::QueryResult::Failed(err.to_string()),
        }
    }

    /// Implementation to find the record by primary key.
    pub fn get_one_by_key(key: String, conn: &DBPool) -> result::QueryResult<Option<User>> {
        let results = users::table
            .select(User::as_select())
            .filter(users::id.eq(key))
            .load(&mut conn.get().unwrap());
        match results {
            Ok(results) => result::QueryResult::Record(results.into_iter().nth(0)),
            Err(err) => result::QueryResult::Failed(err.to_string()),
        }
    }
}
