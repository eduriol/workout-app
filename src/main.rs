#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, workout app!"
}

#[get("/workout/<group>")]
fn workout(group: String) -> Option<&'static str> {
    if group.as_str() == "legs" {
        Some("Squats: 4 x 5\nLunges: 3 x 12")
    } else {
        None
    }
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello, workout])
}

fn main() {
    rocket().launch();
}
