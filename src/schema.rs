table! {
    exercises (id) {
        id -> Int4,
        name -> Varchar,
        sets -> Int4,
        reps -> Int4,
        workout_id -> Nullable<Int4>,
    }
}

table! {
    workouts (id) {
        id -> Int4,
        muscular_group -> Varchar,
    }
}

joinable!(exercises -> workouts (workout_id));

allow_tables_to_appear_in_same_query!(exercises, workouts,);
