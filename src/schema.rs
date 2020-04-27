table! {
    exercises (id) {
        id -> Int4,
        origin_id -> Int4,
        workout_id -> Int4,
        name -> Varchar,
        exercise_type -> Int4,
        description -> Varchar,
        notes -> Varchar,
        create_time -> Timestamp,
        complete_time -> Timestamp,
    }
}

table! {
    processlog (id) {
        id -> Int4,
        create_time -> Timestamp,
        process_time -> Timestamp,
        log -> Varchar,
    }
}

table! {
    rawdata (id) {
        id -> Int4,
        create_time -> Timestamp,
        blob -> Nullable<Bytea>,
        file_size -> Nullable<Int8>,
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
        name -> Varchar,
        email -> Varchar,
        passwd -> Varchar,
    }
}

table! {
    workouts (id) {
        id -> Int4,
        origin_id -> Int4,
        exercise -> Int4,
        name -> Varchar,
        description -> Varchar,
        notes -> Varchar,
        created_time -> Timestamp,
        completed_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    exercises,
    processlog,
    rawdata,
    sets,
    users,
    workouts,
);
