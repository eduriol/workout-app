use crate::schema::exercises;
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
