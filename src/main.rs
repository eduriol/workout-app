#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, world!"
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello])
}

fn main() {
    rocket().launch();
}
