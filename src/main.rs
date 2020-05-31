#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate serde;

pub mod schema;

mod db;
#[cfg(test)]
mod tests;

use rocket_contrib::json::JsonValue;

mod models;
use models::*;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, workout app!"
}

#[get("/exercises")]
fn exercises(connection: db::Connection) -> JsonValue {
    json!(Exercise::read_all(&connection))
}

#[get("/workouts")]
fn workouts(connection: db::Connection) -> JsonValue {
    json!(Workout::read_all(&connection))
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![hello, exercises, workouts])
}

fn main() {
    rocket().launch();
}
