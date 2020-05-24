use rocket;
use rocket::local::Client;
use rocket::http::Status;

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
    let mut response = client.get("/workout/legs").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Squats: 4 x 5\nLunges: 3 x 12".into()));
}

#[test]
fn get_unknown_workout() {
    let client = Client::new(rocket()).expect("Failed to create test client");
    let response = client.get("/workout/unknown").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
