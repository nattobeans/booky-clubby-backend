use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;


#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=members)]
pub struct NewMember {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
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

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    pub review: String,
    pub member_id: i32,
    pub book_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=reviews)]
pub struct NewReview {
    pub review: String,
    pub member_id: i32,
    pub book_id: i32,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
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

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct GroupMember {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=group_members)]
pub struct NewGroupMember {
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Chat {
    pub id: i32,
    pub message: String,
    pub member_id: i32,
    pub to_member_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=chats)]
pub struct NewChat {
    pub message: String,
    pub member_id: i32,
    pub to_member_id: i32,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct GroupChat {
    pub id: i32,
    pub message: String,
    pub group_id: i32,
    pub to_member_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=group_chats)]
pub struct NewGroupChat {
    pub message: String,
    pub group_id: i32,
    pub to_member_id: i32,
}