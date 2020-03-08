table! {
    use super::TypeMapping;
    set(id) {
        id -> Integer,
        exercise_id -> Integer,
        type -> TypeMapping,
        unit -> Text,
        goal_reps -> SmallInt,
        goal_value -> Text,
        description -> Text,
        created_or_completed -> Timestamp,
        completed_reps -> SmallInt,
        completed_value -> Text
    }
}
