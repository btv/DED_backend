table! {
    exercises (id) {
        id -> Int4,
        origin_id -> Int4,
        set_id -> Int4,
        fname -> Varchar,
        exercise_type -> Int4,
        description -> Varchar,
        notes -> Varchar,
        create_time -> Timestamp,
        complete_time -> Timestamp,
        create_id -> Int4,
        completed_id -> Int4,
    }
}

table! {
    sets (id) {
        id -> Int4,
        exercise_id -> Int4,
        style -> Varchar,
        unit -> Varchar,
        goal_reps -> Int2,
        goal_value -> Varchar,
        description -> Varchar,
        created_or_completed -> Timestamp,
        completed_reps -> Int2,
        completed_value -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        fname -> Varchar,
        email -> Varchar,
        salt -> Varchar,
    }
}

table! {
    workouts (id) {
        id -> Int4,
        origin_id -> Int4,
        exercise -> Int4,
        fname -> Varchar,
        description -> Varchar,
        notes -> Varchar,
        created_time -> Timestamp,
        completed_time -> Timestamp,
        create_id -> Int4,
        completed_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    exercises,
    sets,
    users,
    workouts,
);