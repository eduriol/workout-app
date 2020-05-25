use rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn hello_workout_app() {
    let client = Client::new(rocket()).expect("Failed to create test client");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, workout app!".into()));
}

#[test]
fn get_legs_workout() {
    let client = Client::new(rocket()).expect("Failed to create test client");
    let mut response = client.get("/workout/legs").header(ContentType::JSON).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.body().unwrap().into_string().unwrap();
    assert!(body.contains("squats"));
}

#[test]
fn get_unknown_workout() {
    let client = Client::new(rocket()).expect("Failed to create test client");
    let response = client.get("/workout/unknown").header(ContentType::JSON).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
