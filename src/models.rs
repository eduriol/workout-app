use crate::schema::{exercises, workouts};
use diesel::pg::PgConnection;
use diesel::*;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Exercise {
    pub id: i32,
    pub name: String,
    pub sets: i32,
    pub reps: i32,
}

impl Exercise {
    pub fn read_all(connection: &PgConnection) -> Vec<Exercise> {
        exercises::table
            .order(exercises::id)
            .load::<Exercise>(connection)
            .expect("Unable to load exercises data.")
    }
}

#[derive(Serialize, Queryable)]
pub struct Workout {
    pub id: i32,
    pub muscular_group: String,
}

impl Workout {
    pub fn read_all(connection: &PgConnection) -> Vec<Workout> {
        workouts::table
            .order(workouts::id)
            .load::<Workout>(connection)
            .expect("Unable to load workouts data.")
    }
}
