use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;


#[derive(Queryable, AsChangeset, Serialize)]
pub struct Member {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=members)]
pub struct NewMember {
    pub name: Stirng,
    pub email: String,
}

#[derive(Queryable, AsChangeset, Serialize)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=books)]
pub struct NewBook {
    pub name: String,
    pub description: String,
}

#[derive(Queryable, AsChangeset, Serialize)]
pub struct Review {
    pub id: i32,
    pub review: String,
    pub member_id: i32,
    pub book_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=reviews)]
pub struct New_Review {
    pub review: String,
    pub member_id: i32,
    pub book_id: i32,
}