use crate::{models::{NewGroup, Group}, repositories::GroupRepository};
use rocket::{http::Status, response::status::{Custom, NoContent}, serde::json::json};
use rocket_db_pools::Connection;
use rocket::serde::json::{Value, Json};

use super::DbConn;

#[rocket::get("/groups")]
pub async fn get_groups(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    GroupRepository::find_multiple(&mut db, 100).await
        .map(|group| json!(group))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::get("/groups/<id>")]
pub async fn view_group(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
    GroupRepository::find(&mut db, id).await
        .map(|group| json!(group))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::post("/groups", format="json", data="<new_group>")]
pub async fn create_group(mut db: Connection<DbConn>, new_group: Json<NewGroup>) -> Result<Custom<Value>, Custom<Value>>{
    GroupRepository::create(&mut db, new_group.into_inner()).await
        .map(|group| Custom(Status::Created, json!(group)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::put("/groups", format="json", data="<group>")]
pub async fn update_group(mut db: Connection<DbConn>, group: Json<Group>) -> Result<Value, Custom<Value>>{
    GroupRepository::update(&mut db, group.into_inner()).await
        .map(|group| json!(group))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::delete("/groups/<id>")]
pub async fn delete_group(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>>{
    GroupRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}