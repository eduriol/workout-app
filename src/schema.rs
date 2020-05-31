table! {
    exercises (id) {
        id -> Int4,
        name -> Varchar,
        sets -> Int4,
        reps -> Int4,
    }
}

table! {
    workouts (id) {
        id -> Int4,
        muscular_group -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(exercises, workouts,);
