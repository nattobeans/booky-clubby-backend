use crate::{models::{NewMember, Member}, repositories::MemberRepository};
use rocket::{http::Status, response::status::{Custom, NoContent}, serde::json::json};
use rocket_db_pools::Connection;
use rocket::serde::json::{Value, Json};

use super::DbConn;

#[rocket::get("/members")]
pub async fn get_members(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    MemberRepository::find_multiple(&mut db, 100).await
        .map(|member| json!(member))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::get("/members/<id>")]
pub async fn view_member(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
    MemberRepository::find(&mut db, id).await
        .map(|member| json!(member))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::post("/members", format="json", data="<new_member>")]
pub async fn create_member(mut db: Connection<DbConn>, new_member: Json<NewMember>) -> Result<Custom<Value>, Custom<Value>>{
    MemberRepository::create(&mut db, new_member.into_inner()).await
        .map(|member| Custom(Status::Created, json!(member)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::put("/members", format="json", data="<member>")]
pub async fn update_member(mut db: Connection<DbConn>, member: Json<Member>) -> Result<Value, Custom<Value>>{
    MemberRepository::update(&mut db, member.into_inner()).await
        .map(|member| json!(member))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::delete("/members/<id>")]
pub async fn delete_member(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>>{
    MemberRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}