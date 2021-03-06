use crate::{rocket, RocketSetup};
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn hello_workout_app() {
    let client = Client::new(rocket(RocketSetup::Testing))
        .expect("Failed to create test client");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, workout app!".into()));
}

#[test]
fn get_unknown() {
    let client = Client::new(rocket(RocketSetup::Testing)).expect("Failed to create test client");
    let response = client.get("/unknown").header(ContentType::JSON).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn get_all_workouts() {
    let client = Client::new(rocket(RocketSetup::Testing)).expect("Failed to create test client");
    let mut response = client.get("/workouts").header(ContentType::JSON).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.body().unwrap().into_string().unwrap();
    assert!(body.contains("squats"));
    assert!(body.contains("upper"));
    assert!(body.contains("bench press"));
    assert!(body.contains("min_reps"));
    assert!(body.contains("max_reps"));
}
