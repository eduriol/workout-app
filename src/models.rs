use crate::schema::{exercises, workouts};
use diesel::pg::PgConnection;
use diesel::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Associations, Identifiable)]
#[belongs_to(Workout)]
pub struct Exercise {
    pub id: i32,
    pub name: String,
    pub sets: i32,
    pub reps: i32,
    pub workout_id: Option<i32>,
}

#[derive(Serialize, Queryable, Identifiable)]
pub struct Workout {
    pub id: i32,
    pub muscular_group: String,
}

impl Workout {
    pub fn read_all(connection: &PgConnection) -> Vec<(Workout, Vec<Exercise>)> {
        let workouts = workouts::table
            .order(workouts::id)
            .load::<Workout>(connection)
            .expect("Unable to load workouts data.");
        let exercises = Exercise::belonging_to(&workouts)
            .load::<Exercise>(connection)
            .expect("Unable to load exercises data.")
            .grouped_by(&workouts);
        workouts.into_iter().zip(exercises).collect::<Vec<_>>()
    }
}
