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

#[derive(Queryable, AsChangeset, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub current_book_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=groups)]
pub struct NewGroup {
    pub name: String,
    pub current_book_id: i32,
}

#[derive(Queryable, AsChangeset, Serialize)]
pub struct group_members {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=group_members)]
pub struct NewGroup {
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Queryable, AsChangeset, Serialize)]
pub struct Chats {
    pub id: i32,
    pub message: String,
    pub member_id: i32,
    pub to_member_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=group_members)]
pub struct NewChat {
    pub message: String,
    pub member_id: i32,
    pub to_member_id: i32,
}

#[derive(Queryable, AsChangeset, Serialize)]
pub struct GroupChats {
    pub id: i32,
    pub message: String,
    pub group_id: i32,
    pub to_member_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=group_members)]
pub struct NewGroupCHats {
    pub message: String,
    pub group_id: i32,
    pub to_member_id: i32,
}