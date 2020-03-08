table! {
    sets(id) {
        id -> Integer,
        exercise_id -> Nullable<Integer>,
        style -> VarChar,
        unit -> Nullable<VarChar>,
        goal_reps -> SmallInt,
        goal_value -> VarChar,
        description -> Nullable<VarChar>,
        created_or_completed -> Nullable<Timestamp>,
        completed_reps -> Nullable<SmallInt>,
        completed_value -> Nullable<VarChar>,
    }
}
