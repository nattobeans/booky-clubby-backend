mod models;
mod schema;
mod repositories;
mod routes;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            routes::members::get_members,
            routes::members::view_member,
            routes::members::create_member,
            routes::members::update_member,
            routes::members::delete_member,
            routes::books::get_books,
            routes::books::view_book,
            routes::books::create_book,
            routes::books::update_book,
            routes::books::delete_book,
            routes::groups::get_groups,
            routes::groups::view_group,
            routes::groups::create_group,
            routes::groups::update_group,
            routes::groups::delete_group,
        ])
        .attach(routes::DbConn::init())
        .launch()
        .await;
}
