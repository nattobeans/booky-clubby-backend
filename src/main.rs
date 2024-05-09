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
        ])
        .attach(routes::DbConn::init())
        .launch()
        .await;
}
