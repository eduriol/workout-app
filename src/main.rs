#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate rocket_cors;
extern crate serde;

pub mod schema;

mod db;
#[cfg(test)]
mod tests;

use rocket::http::Method;

use rocket_contrib::json::JsonValue;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};

mod models;
use models::Workout;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, workout app!"
}

#[get("/workouts")]
fn workouts(connection: db::Connection) -> JsonValue {
    json!(Workout::read_all(&connection))
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![hello, workouts])
        .attach(make_cors())
}

fn main() -> Result<(), Error> {
    rocket().launch();
    Ok(())
}
