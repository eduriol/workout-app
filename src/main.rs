#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, workout app!"
}

#[get("/workout/<group>")]
fn workout(group: String) -> Option<&'static str> {
    match group.as_str() {
        "legs" => Some("Squats: 4 x 5\nLunges: 3 x 12"),
        "push" => Some("Barbell bench press: 4 x 5\nOverhead barbell press: 3 x 12"),
        "pull" => Some("Deadlift: 4 x 5\nPull-ups: 3 x 8"),
        "upper" => Some("Dumbbell bench press: 4 x 5\nOverhead dumbbell press: 3 x 12"),
        "lower" => Some("Squats: 4 x 5\nLunges: 3 x 12"),
        "full" => Some("Barbell bench press: 4 x 5\nOverhead barbell press: 3 x 12\nSquats: 4 x 5\nLunges: 3 x 12"),
        _ => None,
    }
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello, workout])
}

fn main() {
    rocket().launch();
}
