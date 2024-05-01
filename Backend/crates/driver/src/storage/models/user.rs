use std::io::Write;

use crate::db::{
    self,
    schema::school_trips_driver::{
        sql_types::RoleType,
        users::{self},
    },
};
use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, Deserialize, Clone, Copy)]
#[diesel(sql_type = RoleType)]
pub enum Role {
    PARENT,
    DRIVER,
}

impl ToSql<RoleType, Pg> for Role {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Role::PARENT => out.write_all(b"PARENT")?,
            Role::DRIVER => out.write_all(b"DRIVER")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<RoleType, Pg> for Role {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"PARENT" => Ok(Role::PARENT),
            b"DRIVER" => Ok(Role::DRIVER),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

/// User table sturct
#[derive(AsChangeset, Queryable, Insertable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = db::schema::school_trips_driver::users)]
pub struct User {
    pub name: Option<String>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub id: String,
    pub mobile_number: String,
    pub role: Role,
}

impl Selectable<Pg> for Role {
    type SelectExpression = users::role;

    fn construct_selection() -> Self::SelectExpression {
        users::role
    }
}
