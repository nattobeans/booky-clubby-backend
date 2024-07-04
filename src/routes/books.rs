use crate::{models::{NewBook, Book}, repositories::BookRepository};
use rocket::{http::Status, response::status::{Custom, NoContent}, serde::json::json};
use rocket_db_pools::Connection;
use rocket::serde::json::{Value, Json};

use super::DbConn;

#[rocket::get("/books")]
pub async fn get_books(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    BookRepository::find_multiple(&mut db, 100).await
        .map(|book| json!(book))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::get("/books/<id>")]
pub async fn view_book(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
    BookRepository::find(&mut db, id).await
        .map(|book| json!(book))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::post("/books", format="json", data="<new_book>")]
pub async fn create_book(mut db: Connection<DbConn>, new_book: Json<NewBook>) -> Result<Custom<Value>, Custom<Value>>{
    BookRepository::create(&mut db, new_book.into_inner()).await
        .map(|book| Custom(Status::Created, json!(book)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::put("/books", format="json", data="<book>")]
pub async fn update_book(mut db: Connection<DbConn>, book: Json<Book>) -> Result<Value, Custom<Value>>{
    BookRepository::update(&mut db, book.into_inner()).await
        .map(|book| json!(book))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}

#[rocket::delete("/books/<id>")]
pub async fn delete_book(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>>{
    BookRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
}