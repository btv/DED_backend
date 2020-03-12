table! {
    exercises (id) {
        id -> Int4,
        origin_id -> Nullable<Int4>,
        set_id -> Int4,
        fname -> Varchar,
        exercise_type -> Int4,
        description -> Varchar,
        notes -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
        complete_time -> Nullable<Timestamp>,
        create_id -> Int4,
        completed_id -> Int4,
    }
}

table! {
    sets (id) {
        id -> Int4,
        exercise_id -> Nullable<Int4>,
        style -> Varchar,
        unit -> Nullable<Varchar>,
        goal_reps -> Nullable<Int2>,
        goal_value -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        created_or_completed -> Nullable<Timestamp>,
        completed_reps -> Nullable<Int2>,
        completed_value -> Nullable<Varchar>,
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
    workout (id) {
        id -> Int4,
        origin_id -> Nullable<Int4>,
        exercise -> Int4,
        uname -> Varchar,
        description -> Varchar,
        notes -> Nullable<Varchar>,
        created_time -> Nullable<Timestamp>,
        completed_time -> Nullable<Timestamp>,
        created_id -> Int4,
        completed_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    exercises,
    sets,
    users,
    workout,
);
