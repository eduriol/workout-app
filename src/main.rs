#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;

#[cfg(test)] mod tests;

use rocket_contrib::json::Json;

mod workout;
use workout::{Workout, Exercise};

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, workout app!"
}

#[get("/workout/<group>")]
fn workout(group: String) -> Option<Json<Workout>> {
    match group.as_str() {
        "legs" => Some(Json(Workout {
            group: String::from("legs"),
            exercises: vec![
                Exercise {
                    name: String::from("squats"),
                    sets: 4,
                    reps: 5,
                },
                Exercise {
                    name: String::from("lunges"),
                    sets: 3,
                    reps: 12,
                }
            ],
        })),
        _ => None,
    }
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello, workout])
}

fn main() {
    rocket().launch();
}
