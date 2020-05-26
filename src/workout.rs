use serde::Serialize;

#[derive(Serialize)]
pub struct Exercise {
    pub name: String,
    pub sets: u8,
    pub reps: u8,
}

#[derive(Serialize)]
pub struct Workout {
    pub group: String,
    pub exercises: Vec<Exercise>,
}
