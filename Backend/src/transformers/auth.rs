use crate::models::auth::{User, RegisterUserResponse};
use diesel::result::Error;
use diesel::{RunQueryDsl, ExpressionMethods, query_dsl, QueryDsl};
use crate::db;
use crate::db::database::DBPool;
use db::schema::school_trips::user;


pub fn create_user_api_entity(user:User, pool:DBPool) -> Result<RegisterUserResponse,Error> {
    let insert_result = diesel::insert_into(db::schema::school_trips::user::table)
                    .values(&user)
                    .execute(&mut pool.get().unwrap());
                match insert_result {
                    Ok(_) => Ok (RegisterUserResponse {
                        id: user.id,
                        user_name: user.user_name,
                        mobile_number: user.mobile_number,
                        country_code: user.country_code,
                    }),
                    Err(err) => Err(err)
                }
}