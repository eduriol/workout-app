#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[cfg(test)] mod tests;

use rocket_contrib::json::JsonValue;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, workout app!"
}

#[get("/workout/<group>")]
fn workout(group: String) -> Option<JsonValue> {
    match group.as_str() {
        "legs" => Some(json!({
                "group": "legs",
                "exercises": [
                    {
                        "name": "squats",
                        "sets": 4,
                        "reps": 5
                    },
                    {
                        "name": "lunges",
                        "sets": 3,
                        "reps": 12
                    }
                ]
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
