#[macro_use]
extern crate rocket;
mod api;
mod structs;
use crate::api::account::{create_account, verify_account};
use crate::api::database::NotesDB;
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    rocket::build()
        // Attach the database and mount routes
        .attach(NotesDB::init())
        .mount("/", routes![create_account, verify_account])
}
